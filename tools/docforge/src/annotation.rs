use anyhow::{Context, Result};
use image::{imageops, DynamicImage, GenericImageView, Rgba, RgbaImage};
use imageproc::drawing::{draw_hollow_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use std::path::Path;

use crate::atomic;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationKind {
    Rectangle,
    Arrow,
    Brush,
    Mosaic,
    Crop,
}

impl AnnotationKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Rectangle => "框选重点",
            Self::Arrow => "箭头指向",
            Self::Brush => "自由涂画",
            Self::Mosaic => "涂抹遮挡",
            Self::Crop => "裁剪画面",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub kind: AnnotationKind,
    pub points: Vec<[f32; 2]>,
    pub color: [u8; 4],
    pub width: f32,
}

impl Annotation {
    pub fn new(kind: AnnotationKind, first: [f32; 2]) -> Self {
        Self {
            kind,
            points: vec![first],
            color: [232, 72, 85, 255],
            width: 4.0,
        }
    }
}

pub fn save_annotated(input: &Path, output: &Path, annotations: &[Annotation]) -> Result<()> {
    let mut session = AnnotationSession::load(input)?;
    let (width, height) = session.dimensions();
    let point = |value: [f32; 2]| [value[0] * width as f32, value[1] * height as f32];
    for annotation in annotations {
        if annotation.points.len() < 2 {
            continue;
        }
        let points: Vec<[f32; 2]> = annotation.points.iter().copied().map(point).collect();
        match annotation.kind {
            AnnotationKind::Rectangle => session.push(Mark::Box {
                from: points[0],
                to: *points.last().expect("two points"),
                color: annotation.color,
                width: annotation.width,
            }),
            AnnotationKind::Arrow => session.push(Mark::Arrow {
                from: points[0],
                to: *points.last().expect("two points"),
                color: annotation.color,
                width: annotation.width,
            }),
            AnnotationKind::Brush => session.push(Mark::Stroke {
                points,
                color: annotation.color,
                width: annotation.width,
            }),
            AnnotationKind::Mosaic => session.push(Mark::Mosaic {
                points,
                radius: annotation.width.max(4.0) * 3.0,
            }),
            AnnotationKind::Crop => {
                session.crop = Some((points[0], *points.last().expect("two points")));
            }
        }
    }
    session.save_atomic(output)
}

#[derive(Debug, Clone)]
pub enum Mark {
    Stroke {
        points: Vec<[f32; 2]>,
        color: [u8; 4],
        width: f32,
    },
    Mosaic {
        points: Vec<[f32; 2]>,
        radius: f32,
    },
    Box {
        from: [f32; 2],
        to: [f32; 2],
        color: [u8; 4],
        width: f32,
    },
    Arrow {
        from: [f32; 2],
        to: [f32; 2],
        color: [u8; 4],
        width: f32,
    },
}

#[derive(Clone)]
pub struct AnnotationSession {
    original: DynamicImage,
    pub marks: Vec<Mark>,
    pub crop: Option<([f32; 2], [f32; 2])>,
}

impl AnnotationSession {
    pub fn load(path: &Path) -> Result<Self> {
        let original =
            image::open(path).with_context(|| format!("无法打开图片 {}", path.display()))?;
        Ok(Self {
            original,
            marks: Vec::new(),
            crop: None,
        })
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.original.dimensions()
    }

    pub fn push(&mut self, mark: Mark) {
        self.marks.push(mark);
    }

    pub fn render(&self) -> RgbaImage {
        let mut image = self.original.to_rgba8();
        for mark in &self.marks {
            match mark {
                Mark::Stroke {
                    points,
                    color,
                    width,
                } => draw_stroke(&mut image, points, Rgba(*color), *width),
                Mark::Mosaic { points, radius } => {
                    for point in points {
                        pixelate_area(&mut image, *point, *radius);
                    }
                }
                Mark::Box {
                    from,
                    to,
                    color,
                    width,
                } => draw_box(&mut image, *from, *to, Rgba(*color), *width),
                Mark::Arrow {
                    from,
                    to,
                    color,
                    width,
                } => draw_arrow(&mut image, *from, *to, Rgba(*color), *width),
            }
        }
        if let Some((from, to)) = self.crop {
            let (x, y, width, height) = normalized_rect(from, to, image.width(), image.height());
            if width > 1 && height > 1 {
                return imageops::crop_imm(&image, x, y, width, height).to_image();
            }
        }
        image
    }

    pub fn save_atomic(&self, path: &Path) -> Result<()> {
        let rendered = self.render();
        let mut cursor = std::io::Cursor::new(Vec::new());
        DynamicImage::ImageRgba8(rendered).write_to(&mut cursor, image::ImageFormat::Png)?;
        atomic::atomic_write(path, cursor.get_ref())
    }
}

fn draw_stroke(image: &mut RgbaImage, points: &[[f32; 2]], color: Rgba<u8>, width: f32) {
    if points.len() == 1 {
        draw_thick_line(image, points[0], points[0], color, width);
    }
    for pair in points.windows(2) {
        draw_thick_line(image, pair[0], pair[1], color, width);
    }
}

fn draw_box(image: &mut RgbaImage, from: [f32; 2], to: [f32; 2], color: Rgba<u8>, width: f32) {
    let (x, y, rect_width, rect_height) = normalized_rect(from, to, image.width(), image.height());
    for offset in 0..width.max(1.0).round() as i32 {
        let rect = Rect::at(x as i32 - offset, y as i32 - offset).of_size(
            rect_width.saturating_add((offset * 2) as u32),
            rect_height.saturating_add((offset * 2) as u32),
        );
        draw_hollow_rect_mut(image, rect, color);
    }
}

fn draw_arrow(image: &mut RgbaImage, from: [f32; 2], to: [f32; 2], color: Rgba<u8>, width: f32) {
    draw_thick_line(image, from, to, color, width);
    let angle = (to[1] - from[1]).atan2(to[0] - from[0]);
    let length = (width * 4.0).max(18.0);
    let spread = 0.55;
    for direction in [
        angle + std::f32::consts::PI - spread,
        angle + std::f32::consts::PI + spread,
    ] {
        let head = [
            to[0] + length * direction.cos(),
            to[1] + length * direction.sin(),
        ];
        draw_thick_line(image, to, head, color, width);
    }
}

fn draw_thick_line(
    image: &mut RgbaImage,
    from: [f32; 2],
    to: [f32; 2],
    color: Rgba<u8>,
    width: f32,
) {
    let radius = (width.max(1.0) / 2.0).round() as i32;
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius * radius {
                draw_line_segment_mut(
                    image,
                    (from[0] + x as f32, from[1] + y as f32),
                    (to[0] + x as f32, to[1] + y as f32),
                    color,
                );
            }
        }
    }
}

fn pixelate_area(image: &mut RgbaImage, center: [f32; 2], radius: f32) {
    let block = 12u32;
    let min_x = (center[0] - radius).max(0.0) as u32;
    let min_y = (center[1] - radius).max(0.0) as u32;
    let max_x = (center[0] + radius).min(image.width() as f32) as u32;
    let max_y = (center[1] + radius).min(image.height() as f32) as u32;
    for block_y in (min_y..max_y).step_by(block as usize) {
        for block_x in (min_x..max_x).step_by(block as usize) {
            let sample_x = (block_x + block / 2).min(image.width().saturating_sub(1));
            let sample_y = (block_y + block / 2).min(image.height().saturating_sub(1));
            let color = *image.get_pixel(sample_x, sample_y);
            for y in block_y..(block_y + block).min(max_y) {
                for x in block_x..(block_x + block).min(max_x) {
                    let dx = x as f32 - center[0];
                    let dy = y as f32 - center[1];
                    if dx * dx + dy * dy <= radius * radius {
                        image.put_pixel(x, y, color);
                    }
                }
            }
        }
    }
}

fn normalized_rect(
    from: [f32; 2],
    to: [f32; 2],
    max_width: u32,
    max_height: u32,
) -> (u32, u32, u32, u32) {
    let x1 = from[0].min(to[0]).clamp(0.0, max_width as f32) as u32;
    let y1 = from[1].min(to[1]).clamp(0.0, max_height as f32) as u32;
    let x2 = from[0].max(to[0]).clamp(0.0, max_width as f32) as u32;
    let y2 = from[1].max(to[1]).clamp(0.0, max_height as f32) as u32;
    (x1, y1, x2.saturating_sub(x1), y2.saturating_sub(y1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_reverse_selection() {
        assert_eq!(
            normalized_rect([10.0, 20.0], [2.0, 3.0], 100, 100),
            (2, 3, 8, 17)
        );
    }

    #[test]
    fn saves_rectangle_arrow_brush_mosaic_and_crop() {
        let directory = tempfile::tempdir().expect("创建测试目录");
        let input = directory.path().join("input.png");
        let output = directory.path().join("output.png");
        DynamicImage::new_rgba8(100, 80)
            .save(&input)
            .expect("保存输入图片");

        let mark = |kind, points| Annotation {
            kind,
            points,
            color: [255, 0, 0, 255],
            width: 4.0,
        };
        let annotations = vec![
            mark(AnnotationKind::Rectangle, vec![[0.1, 0.1], [0.9, 0.9]]),
            mark(AnnotationKind::Arrow, vec![[0.1, 0.8], [0.8, 0.2]]),
            mark(
                AnnotationKind::Brush,
                vec![[0.1, 0.5], [0.5, 0.5], [0.9, 0.5]],
            ),
            mark(
                AnnotationKind::Mosaic,
                vec![[0.2, 0.2], [0.5, 0.4], [0.8, 0.6]],
            ),
            mark(AnnotationKind::Crop, vec![[0.2, 0.25], [0.8, 0.75]]),
        ];

        save_annotated(&input, &output, &annotations).expect("保存编辑图片");
        assert_eq!(
            image::open(output).expect("打开输出").dimensions(),
            (60, 40)
        );
    }
}

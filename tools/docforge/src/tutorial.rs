use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TutorialStep {
    pub frame: PathBuf,
    pub title: String,
    pub description: String,
    pub timestamp: Option<f64>,
}

impl TutorialStep {
    pub fn new(frame: PathBuf, index: usize, timestamp: Option<f64>) -> Self {
        Self {
            frame,
            title: format!("步骤 {}", index + 1),
            description: "请根据画面完成这一操作，然后继续下一步。".to_string(),
            timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TutorialMedia {
    pub title: String,
    pub description: String,
    pub public_path: String,
    pub timestamp: Option<f64>,
}

pub fn render_document(
    title: &str,
    summary: &str,
    prerequisites: &str,
    steps: &[TutorialMedia],
) -> String {
    let title = non_empty(title, "视频操作教程");
    let summary = non_empty(summary, "本教程根据操作视频的关键画面生成。");
    let prerequisites = non_empty(prerequisites, "准备好需要使用的 AUTO-MAS 环境。");
    let mut output =
        format!("# {title}\n\n{summary}\n\n## 开始前准备\n\n- {prerequisites}\n\n## 操作步骤\n\n");
    output.push_str(&render_steps(steps));
    output.push_str(
        "::: tip 完成检查\n完成全部步骤后，请返回 AUTO-MAS 检查配置是否已经生效。\n:::\n",
    );
    output
}

pub fn render_section(title: &str, summary: &str, steps: &[TutorialMedia]) -> String {
    let title = non_empty(title, "视频操作教程");
    let summary = non_empty(summary, "以下步骤根据操作视频的关键画面生成。");
    format!("\n## {title}\n\n{summary}\n\n{}", render_steps(steps))
}

fn render_steps(steps: &[TutorialMedia]) -> String {
    let mut output = String::new();
    for (index, step) in steps.iter().enumerate() {
        let fallback_title = format!("步骤 {}", index + 1);
        let title = non_empty(&step.title, &fallback_title);
        let description = non_empty(
            &step.description,
            "请根据画面完成这一操作，然后继续下一步。",
        );
        output.push_str(&format!("### {}. {title}\n\n", index + 1));
        if let Some(seconds) = step.timestamp {
            output.push_str(&format!("> 视频位置：{}\n\n", format_timestamp(seconds)));
        }
        output.push_str(&format!(
            "{description}\n\n![{title}]({})\n\n",
            step.public_path
        ));
    }
    output
}

fn non_empty<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    let value = value.trim();
    if value.is_empty() {
        fallback
    } else {
        value
    }
}

fn format_timestamp(seconds: f64) -> String {
    let total = seconds.max(0.0).round() as u64;
    format!("{:02}:{:02}", total / 60, total % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_complete_vitepress_tutorial() {
        let output = render_document(
            "安装教程",
            "跟随画面完成安装。",
            "下载绿色包。",
            &[TutorialMedia {
                title: "打开设置".to_string(),
                description: "点击左侧设置入口。".to_string(),
                public_path: "/docs/img/generated/step.png".to_string(),
                timestamp: Some(65.0),
            }],
        );
        assert!(output.contains("# 安装教程"));
        assert!(output.contains("> 视频位置：01:05"));
        assert!(output.contains("![打开设置](/docs/img/generated/step.png)"));
        assert!(output.contains("::: tip 完成检查"));
    }
}

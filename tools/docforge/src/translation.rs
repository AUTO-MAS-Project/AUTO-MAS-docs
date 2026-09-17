use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct TranslationConfig {
    pub endpoint: String,
    pub model: String,
    pub api_key: String,
    pub source_language: String,
    pub target_language: String,
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.openai.com/v1".to_string(),
            model: "gpt-4.1-mini".to_string(),
            api_key: String::new(),
            source_language: "简体中文".to_string(),
            target_language: "English".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    temperature: f32,
    messages: Vec<Message<'a>>,
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

pub fn translate_markdown(source: &str, config: &TranslationConfig) -> Result<String> {
    if source.trim().is_empty() {
        bail!("当前文档没有可翻译内容");
    }
    if config.endpoint.trim().is_empty() || config.model.trim().is_empty() {
        bail!("请填写翻译接口地址和模型名称");
    }

    let endpoint = normalize_endpoint(&config.endpoint);
    let instruction = format!(
        "你是 AUTO-MAS VitePress 文档翻译助手。请把以下 Markdown 从{}翻译为{}。\n\
         必须完整保留 Markdown 结构、标题级别、frontmatter、代码块、行内代码、HTML/Vue 组件、\
         ::: 容器、URL、文件路径、锚点、命令、配置键和产品名。图片 URL 不得改变，只翻译图片替代文字。\n\
         不要增加说明，不要用代码围栏包住全文，只返回翻译后的 Markdown。",
        config.source_language, config.target_language
    );
    let request = ChatRequest {
        model: config.model.trim(),
        temperature: 0.1,
        messages: vec![
            Message {
                role: "system",
                content: &instruction,
            },
            Message {
                role: "user",
                content: source,
            },
        ],
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(180))
        .build()?;
    let mut http = client.post(endpoint).json(&request);
    if !config.api_key.trim().is_empty() {
        http = http.bearer_auth(config.api_key.trim());
    }
    let response = http.send().context("无法连接翻译接口")?;
    let status = response.status();
    let body = response.text().context("无法读取翻译接口响应")?;
    if !status.is_success() {
        bail!("翻译接口返回 {}：{}", status, shorten(&body, 600));
    }
    let parsed: ChatResponse = serde_json::from_str(&body)
        .with_context(|| format!("翻译接口响应格式不兼容：{}", shorten(&body, 300)))?;
    let translated = parsed
        .choices
        .into_iter()
        .next()
        .map(|choice| choice.message.content)
        .filter(|content| !content.trim().is_empty())
        .context("翻译接口没有返回文本")?;
    Ok(strip_outer_fence(translated))
}

fn normalize_endpoint(value: &str) -> String {
    let value = value.trim().trim_end_matches('/');
    if value.ends_with("/chat/completions") {
        value.to_string()
    } else {
        format!("{value}/chat/completions")
    }
}

fn strip_outer_fence(value: String) -> String {
    let trimmed = value.trim();
    if (trimmed.starts_with("```markdown") || trimmed.starts_with("```md"))
        && trimmed.ends_with("```")
    {
        let first_newline = trimmed.find('\n').unwrap_or(0);
        return trimmed[first_newline + 1..trimmed.len() - 3]
            .trim()
            .to_string();
    }
    value
}

fn shorten(value: &str, limit: usize) -> String {
    let mut output: String = value.chars().take(limit).collect();
    if value.chars().count() > limit {
        output.push_str("...");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_openai_compatible_path() {
        assert_eq!(
            normalize_endpoint("https://example.test/v1"),
            "https://example.test/v1/chat/completions"
        );
    }

    #[test]
    fn removes_markdown_wrapper() {
        assert_eq!(
            strip_outer_fence("```markdown\n# Title\n```".into()),
            "# Title"
        );
    }
}

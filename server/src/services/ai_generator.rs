use crate::errors::AppError;
use reqwest::Client;
use serde_json::json;

pub struct AiGeneratorService;

impl AiGeneratorService {
    pub async fn generate_cake_images(
        ai_api_url: &str,
        ai_api_key: &str,
        scene: &str,
        theme: &str,
        blessing: &str,
        color_preference: &str,
        style: &str,
    ) -> Result<(Vec<String>, Vec<i64>, i64), AppError> {
        if ai_api_url.is_empty() {
            tracing::warn!("AI_API_URL not set — returning mock images");
            let mock_images = (1..=5)
                .map(|i| format!("placeholder://cake_design/{}", i))
                .collect();
            let mock_template_ids = vec![1, 2, 3, 4, 5];
            return Ok((mock_images, mock_template_ids, 0));
        }

        let provider = dotenvy::var("AI_PROVIDER").unwrap_or_else(|_| "tongyi".into());
        match provider.to_lowercase().as_str() {
            "tongyi" | "qwen" | "alibaba" => {
                Self::generate_tongyi(ai_api_url, ai_api_key, scene, theme, blessing, color_preference, style).await
            }
            _ => {
                Self::generate_openai_compat(ai_api_url, ai_api_key, scene, theme, blessing, color_preference, style).await
            }
        }
    }

    async fn generate_tongyi(
        api_url: &str,
        api_key: &str,
        scene: &str,
        theme: &str,
        blessing: &str,
        color_preference: &str,
        style: &str,
    ) -> Result<(Vec<String>, Vec<i64>, i64), AppError> {
        let prompt = Self::build_prompt(scene, theme, blessing, color_preference, style);

        let client = Client::new();
        let body = json!({
            "model": dotenvy::var("AI_MODEL").unwrap_or_else(|_| "wanx-v1".into()),
            "input": {
                "prompt": prompt,
            },
            "parameters": {
                "n": 5,
                "size": "512x512",
                "style": "<auto>",
            }
        });

        let response = client.post(format!("{}/api/v1/services/aigc/text2image/image-synthesis", api_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .header("X-DashScope-Async", "enable")
            .json(&body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Tongyi API call failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("Tongyi API error: status={}, body={}", status, text);
            return Err(AppError::Internal("AI service unavailable".into()));
        }

        let resp_body: serde_json::Value = response.json().await
            .map_err(|e| AppError::Internal(format!("Tongyi API response parse failed: {}", e)))?;

        let task_id = resp_body["output"]["task_id"].as_str().unwrap_or("");
        if task_id.is_empty() {
            return Err(AppError::Internal("AI task creation failed: no task_id".into()));
        }

        // Poll for async task result
        let task_result = Self::poll_ai_task(&client, api_url, api_key, task_id).await?;
        let images: Vec<String> = task_result["output"]["results"]
            .as_array()
            .map(|arr| arr.iter().filter_map(|v| v["url"].as_str().map(String::from)).collect())
            .unwrap_or_default();

        let template_ids: Vec<i64> = (1..=(images.len() as i64)).collect();
        let generation_id = chrono::Utc::now().timestamp_millis();

        tracing::info!("AI generated {} images for task {}", images.len(), task_id);
        Ok((images, template_ids, generation_id))
    }

    async fn generate_openai_compat(
        api_url: &str,
        api_key: &str,
        scene: &str,
        theme: &str,
        blessing: &str,
        color_preference: &str,
        style: &str,
    ) -> Result<(Vec<String>, Vec<i64>, i64), AppError> {
        let prompt = Self::build_prompt(scene, theme, blessing, color_preference, style);

        let client = Client::new();
        let body = json!({
            "prompt": prompt,
            "n": 5,
            "size": "512x512",
        });

        let response = client.post(format!("{}/v1/images/generations", api_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .timeout(std::time::Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("AI API call failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("AI API error: status={}, body={}", status, text);
            return Err(AppError::Internal("AI service unavailable".into()));
        }

        let resp_body: serde_json::Value = response.json().await
            .map_err(|e| AppError::Internal(format!("AI API response parse failed: {}", e)))?;

        let images: Vec<String> = resp_body["data"].as_array()
            .map(|arr| arr.iter()
                .filter_map(|v| {
                    v["url"].as_str().map(String::from)
                        .or_else(|| v["b64_json"].as_str().map(|_| "data:image/png;base64,...".into()))
                })
                .collect())
            .unwrap_or_default();

        let template_ids: Vec<i64> = (1..=(images.len() as i64)).collect();
        let generation_id = chrono::Utc::now().timestamp_millis();

        tracing::info!("AI generated {} images via OpenAI-compat API", images.len());
        Ok((images, template_ids, generation_id))
    }

    async fn poll_ai_task(client: &Client, api_url: &str, api_key: &str, task_id: &str) -> Result<serde_json::Value, AppError> {
        let max_attempts = 30;
        let poll_interval = std::time::Duration::from_secs(2);

        for _ in 0..max_attempts {
            let resp = client.get(format!("{}/api/v1/tasks/{}", api_url, task_id))
                .header("Authorization", format!("Bearer {}", api_key))
                .timeout(std::time::Duration::from_secs(10))
                .send()
                .await
                .map_err(|e| AppError::Internal(format!("AI task poll failed: {}", e)))?;

            if !resp.status().is_success() {
                tokio::time::sleep(poll_interval).await;
                continue;
            }

            let body: serde_json::Value = resp.json().await
                .map_err(|e| AppError::Internal(format!("AI task poll response parse failed: {}", e)))?;

            let status = body["output"]["task_status"].as_str().unwrap_or("");
            match status {
                "SUCCEEDED" => return Ok(body),
                "FAILED" => {
                    tracing::error!("AI task {} failed: {:?}", task_id, body);
                    return Err(AppError::Internal("AI generation task failed".into()));
                }
                _ => {
                    tokio::time::sleep(poll_interval).await;
                }
            }
        }

        Err(AppError::Internal("AI generation task timed out".into()))
    }

    pub fn build_prompt(scene: &str, theme: &str, blessing: &str, color_preference: &str, style: &str) -> String {
        let sanitize = |s: &str| -> String {
            s.chars()
                .filter(|c| c.is_alphanumeric() || c.is_whitespace() || "\u{FF0C}\u{3002}\u{FF01}\u{FF1F}\u{3001}\u{FF1A}\u{FF1B}".contains(*c))
                .take(100)
                .collect()
        };

        let scene_zh = match scene {
            "birthday" => "生日蛋糕",
            "children" => "儿童蛋糕",
            "festival" => "节庆蛋糕",
            "wedding" => "婚庆蛋糕",
            _ => "创意蛋糕",
        };

        format!(
            "一款精美的{}设计，{}风格，{}色系，主题元素包含{}，装饰文字\"{}\"，食品级摄影，高清细节，白色背景",
            scene_zh,
            sanitize(style),
            sanitize(color_preference),
            sanitize(theme),
            sanitize(blessing),
        )
    }
}

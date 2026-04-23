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
    ) -> Result<(Vec<String>, Vec<i64>), AppError> {
        if ai_api_url.is_empty() {
            let mock_images = vec![
                "https://cdn.example.com/cake_1.png".to_string(),
                "https://cdn.example.com/cake_2.png".to_string(),
                "https://cdn.example.com/cake_3.png".to_string(),
                "https://cdn.example.com/cake_4.png".to_string(),
                "https://cdn.example.com/cake_5.png".to_string(),
            ];
            let mock_template_ids = vec![1, 2, 3, 4, 5];
            return Ok((mock_images, mock_template_ids));
        }

        let prompt = format!(
            "A {} cake design: theme={}, blessing={}, colors={}, style={}",
            scene, theme, blessing, color_preference, style
        );

        let client = Client::new();
        let response = client.post(ai_api_url)
            .header("Authorization", format!("Bearer {}", ai_api_key))
            .json(&json!({
                "prompt": prompt,
                "n": 5,
                "size": "512x512",
            }))
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("AI API call failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::Internal("AI service unavailable".into()));
        }

        let body: serde_json::Value = response.json().await
            .map_err(|e| AppError::Internal(format!("AI API response parse failed: {}", e)))?;

        let images = body["images"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        let template_ids = body["template_ids"].as_array()
            .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
            .unwrap_or_default();

        Ok((images, template_ids))
    }
}

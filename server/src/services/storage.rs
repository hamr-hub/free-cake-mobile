use crate::errors::AppError;

pub struct StorageService;

impl StorageService {
    pub async fn upload(
        supabase_url: &str,
        supabase_api_key: &str,
        bucket: &str,
        file_name: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<String, AppError> {
        let client = reqwest::Client::new();
        let url = format!("{supabase_url}/storage/v1/object/{bucket}/{file_name}");

        let response = client
            .post(&url)
            .header("apikey", supabase_api_key)
            .header("Authorization", format!("Bearer {supabase_api_key}"))
            .header("Content-Type", "multipart/form-data")
            .multipart(
                reqwest::multipart::Form::new()
                    .part("file", reqwest::multipart::Part::bytes(data.to_vec())
                        .file_name(file_name.to_string())
                        .mime_str(content_type)
                        .unwrap_or_else(|_| reqwest::multipart::Part::bytes(data.to_vec()))),
            )
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("Supabase Storage upload failed: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(format!("Supabase Storage error {status}: {body}")));
        }

        Ok(format!("{supabase_url}/storage/v1/object/public/{bucket}/{file_name}"))
    }
}

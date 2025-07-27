use anyhow::Result;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use base64;

#[derive(Debug, Serialize)]
struct CloudinaryUploadParams {
    upload_preset: String,
    folder: String,
}

#[derive(Debug, Deserialize)]
pub struct CloudinaryResponse {
    pub public_id: String,
    pub url: String,
    pub secure_url: String,
}

pub struct CloudinaryService {
    cloud_name: String,
    api_key: String,
    api_secret: String,
    upload_preset: String,
}

impl CloudinaryService {
    pub fn new(
        cloud_name: String,
        api_key: String,
        api_secret: String,
        upload_preset: String,
    ) -> Self {
        Self {
            cloud_name,
            api_key,
            api_secret,
            upload_preset,
        }
    }

    pub async fn upload_image(
        &self,
        image_data: Vec<u8>,
        filename: &str,
        folder: &str,
    ) -> Result<CloudinaryResponse> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );

        // Create basic auth header
        let auth = base64::encode(format!("{}:{}", self.api_key, self.api_secret));

        let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(image_data).file_name(filename.to_string()))
            .text("upload_preset", self.upload_preset.clone())
            .text("folder", folder.to_string());

        let response = client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .multipart(form)
            .send()
            .await?;

        let cloudinary_response: CloudinaryResponse = response.json().await?;
        Ok(cloudinary_response)
    }
}
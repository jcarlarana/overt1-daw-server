#[cfg(test)]
mod tests {
    use std::fs;
    use reqwest;

    #[tokio::test]
    async fn test_audio_upload() -> Result<(), reqwest::Error> {
        // URL of your audio upload endpoint
        let url = "http://localhost:8000/api/audio/upload";

        // Path to your test audio file
        let file_path = "fixtures/test_audio.mp3";

        // Read the test audio file
        let bytes = fs::read(file_path).expect("Failed to read test audio file");

        // Create a multipart/form-data request
        let form = reqwest::multipart::Form::new()
            .text("description", "Test Audio File")
            .part("audio", reqwest::multipart::Part::bytes(bytes).file_name("test_audio.mp3"));

        // Send the request to the upload endpoint
        let client = reqwest::Client::new();
        let response = client.post(url)
            .multipart(form)
            .send()
            .await?;

        // Check if the upload was successful
        assert!(response.status().is_success(), "Audio upload failed: {:?}", response.text().await);

        Ok(())
    }
}


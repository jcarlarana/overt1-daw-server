#[cfg(test)]
mod tests {
    use reqwest;

    #[tokio::test]
    async fn test_audio_retrieval() -> Result<(), reqwest::Error> {
        // URL of your audio retrieval endpoint
        // You need to replace `your_audio_file_id` with an actual file ID that exists in your S3 bucket for testing
        let url = "http://localhost:8000/api/audio/retrieve/your_audio_file_id";

        // Send a GET request to the retrieval endpoint
        let client = reqwest::Client::new();
        let response = client.get(url)
            .send()
            .await?;

        // Check if the retrieval was successful by looking at the HTTP status code
        assert!(response.status().is_success(), "Audio retrieval failed: {:?}", response.text().await);

        // Optionally, you might want to check the headers to ensure the content type is as expected for audio files
        let headers = response.headers();
        assert!(headers.contains_key(reqwest::header::CONTENT_TYPE), "Response is missing Content-Type header");
        let content_type = headers[reqwest::header::CONTENT_TYPE].to_str().unwrap();
        assert!(content_type.starts_with("audio/"), "Unexpected content type: {}", content_type);

        // Optionally, you could also download the file and check its size, format, etc. to ensure it matches expectations

        Ok(())
    }
}


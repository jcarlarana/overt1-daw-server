#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;
    use std::fs;

    #[tokio::test]
    async fn test_audio_effect_application() -> Result<(), reqwest::Error> {
        // URL of your audio effect application endpoint
        let url = "http://localhost:8000/api/audio/effect";

        // Constructing multipart form with audio file and effect parameters
        let path = "tests/fixtures/sample_audio.mp3";
        let audio_part = reqwest::multipart::Part::file(path).expect("File should exist");

        let form = reqwest::multipart::Form::new()
            .part("audio", audio_part)
            .text("effect_type", "reverb")
            .text("effect_params", json!({ "depth": 0.8, "decay": 1.2 }).to_string());

        // Send a POST request with the form to the endpoint
        let client = reqwest::Client::new();
        let response = client.post(url)
            .multipart(form)
            .send()
            .await?;

        // Check if the audio effect application was successful
        assert_eq!(response.status().as_u16(), 200, "Audio effect application failed: {:?}", response.text().await.unwrap());

        // Optionally, you can write the processed audio to a file for manual inspection
        let processed_audio = response.bytes().await?;
        fs::write("tests/output/processed_audio.mp3", &processed_audio).expect("Unable to write file");

        Ok(())
    }
}


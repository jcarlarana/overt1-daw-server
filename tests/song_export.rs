#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_song_export() -> Result<(), reqwest::Error> {
        // URL of your song export endpoint
        let url = "http://localhost:8000/api/song/export";

        // Prepare the payload (example of project or song info)
        let payload = json!({
            "project_id": "12345",
            "tracks": [
                {
                    "track_id": "1",
                    "effects": [
                        {
                            "effect_type": "reverb",
                            "parameters": { "depth": 0.8, "decay": 1.2 }
                        }
                    ]
                },
                {
                    "track_id": "2",
                    "effects": [
                        {
                            "effect_type": "equalization",
                            "parameters": { "frequencies": [100, 200, 400], "gains": [1.0, -1.5, 0.5] }
                        }
                    ]
                }
            ]
        });

        // Send a POST request with the payload to the endpoint
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&payload)
            .send()
            .await?;

        // Check if the song export was successful
        assert_eq!(response.status().as_u16(), 200, "Song export failed: {:?}", response.text().await.unwrap());

        // Retrieve the export link from the response (adjust as needed based on actual response structure)
        let export_link: String = response.json().await.expect("Failed to deserialize response");

        // Optionally, you may want to download the exported song and perform additional checks
        let song_data = client.get(&export_link).send().await?.bytes().await?;
        assert!(!song_data.is_empty(), "Exported song data is empty");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_user_registration() -> Result<(), reqwest::Error> {
        // URL of your user registration endpoint
        let url = "http://localhost:8000/api/user/register";

        // Construct the user registration data
        // Replace these values with the actual data structure you use for user registration
        let user_data = json!({
            "username": "testuser",
            "email": "testuser@example.com",
            "password": "password123"
        });

        // Send a POST request to the registration endpoint with the user data
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&user_data)
            .send()
            .await?;

        // Check if the registration was successful by looking at the HTTP status code
        assert_eq!(response.status().as_u16(), 200, "User registration failed: {:?}", response.text().await.unwrap());

        // Optionally: If your endpoint returns specific data upon success, you can check it here

        Ok(())
    }
}


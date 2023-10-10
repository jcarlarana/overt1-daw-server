#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_user_login() -> Result<(), reqwest::Error> {
        // URL of your user login endpoint
        let url = "http://localhost:8000/api/user/login";

        // Construct the user login data
        // This should match the credentials of a user that you know exists in the database
        let login_data = json!({
            "username": "existinguser",
            "password": "password123"
        });

        // Send a POST request to the login endpoint with the login data
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&login_data)
            .send()
            .await?;

        // Check if the login was successful by looking at the HTTP status code
        assert_eq!(response.status().as_u16(), 200, "User login failed: {:?}", response.text().await.unwrap());

        // Optionally: If your endpoint returns a token or user data upon successful login, you can check it here

        Ok(())
    }
}


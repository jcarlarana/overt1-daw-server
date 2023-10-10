#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_project_creation_and_saving() -> Result<(), reqwest::Error> {
        // URL of your project creation endpoint
        let url = "http://localhost:8000/api/project/create";

        // Construct the project data
        let project_data = json!({
            "name": "Test Project",
            "description": "This is a test project",
            "userId": "some_existing_user_id"
            // ... add other necessary project attributes ...
        });

        // Send a POST request to the project creation endpoint with the project data
        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&project_data)
            .send()
            .await?;

        // Check if the project creation was successful by looking at the HTTP status code
        assert_eq!(response.status().as_u16(), 201, "Project creation failed: {:?}", response.text().await.unwrap());

        // Optionally: If your endpoint returns the created project data or ID, you can check it here.
        // For example, you can extract the project ID from the response, then send a GET request to retrieve and check the project data.

        // Example: Getting created project's ID from response (adjust this based on your actual response format)
        let created_project: serde_json::Value = response.json().await?;
        let project_id = created_project["id"].as_str().expect("Project ID not found in response");

        // Example: Sending a GET request to retrieve the created project (adjust this based on your actual endpoint and data format)
        let get_url = format!("http://localhost:8000/api/project/{}", project_id);
        let get_response = client.get(&get_url).send().await?;
        assert_eq!(get_response.status().as_u16(), 200, "Failed to retrieve the created project: {:?}", get_response.text().await.unwrap());

        // ... You can also check if the retrieved project data matches the data you used for creation ...

        Ok(())
    }
}


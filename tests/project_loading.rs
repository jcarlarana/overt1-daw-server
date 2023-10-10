#[cfg(test)]
mod tests {
    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_project_loading() -> Result<(), reqwest::Error> {
        // URL of your project loading endpoint (adjust this to your actual endpoint)
        // Assuming you have a project with ID `existing_project_id` in your database
        let url = "http://localhost:8000/api/project/load/existing_project_id";

        // Send a GET request to the project loading endpoint
        let client = reqwest::Client::new();
        let response = client.get(url)
            .send()
            .await?;

        // Check if the project loading was successful by looking at the HTTP status code
        assert_eq!(response.status().as_u16(), 200, "Project loading failed: {:?}", response.text().await.unwrap());

        // Extract and check the loaded project data from the response (adjust this based on your actual response format)
        let loaded_project: serde_json::Value = response.json().await?;
        assert_eq!(loaded_project["name"].as_str().unwrap(), "Expected Project Name", "Project name does not match expected value");
        assert_eq!(loaded_project["description"].as_str().unwrap(), "Expected Project Description", "Project description does not match expected value");
        // ... add other necessary checks for project attributes ...

        Ok(())
    }
}


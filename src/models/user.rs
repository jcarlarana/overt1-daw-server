use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Enum representing the subscription level of a user
#[derive(Debug, Serialize, Deserialize)]
pub enum Subscription {
    Free,
    Premium,
}

// Struct representing a User in the system
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid, // Unique identifier for the user
    pub username: String, // Username chosen by the user
    pub email: String, // Email address of the user
    pub subscription: Subscription, // Subscription level of the user
    pub projects: Vec<Uuid>, // List of project IDs associated with the user
}

impl User {
    // Function to create a new User
    pub fn new(username: String, email: String, subscription: Subscription) -> Self {
        Self {
            id: Uuid::new_v4(), // Generate a unique ID for the user
            username,
            email,
            subscription,
            projects: Vec::new(), // Initialize with no associated projects
        }
    }

    // Method to add a project ID to the user's list of projects
    pub fn add_project(&mut self, project_id: Uuid) {
        self.projects.push(project_id);
    }
}

// You can add more functions and methods as needed to support your application's logic.


use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

impl User {
    /// Get a reference to the user's username.
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }
}

use serde::Deserialize;

// createsession
#[derive(Debug, Deserialize)]
pub struct CreateSessionRes {
    pub session_id: String,
}
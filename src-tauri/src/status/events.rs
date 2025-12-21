use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)] 
pub struct BackendReadyEvent;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub enum StatusType {
    Idle,
    Indexing,
    Processing,
    Notification,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct StatusEvent {
    pub status: StatusType,
    pub message: Option<String>,

    pub total: Option<i32>,
    pub processed: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
pub struct ErrorEvent {
    pub message: String,
    pub context: Option<String>,
}
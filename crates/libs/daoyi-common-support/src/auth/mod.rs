use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Principal {
    pub id: String,
    pub name: String,
}
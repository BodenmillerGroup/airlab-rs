use crate::model::member::Member;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ViewGroup {
    pub id: i32,
    pub name: String,
    pub institution: String,
    pub url: String,
    #[serde(rename = "isOpen")]
    pub is_open: bool,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub members: Vec<Member>,
}

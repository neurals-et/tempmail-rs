use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    pub mailbox: String,
    #[serde(default)]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub mailbox: String,
    pub messages: Vec<MessageSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSummary {
    pub _id: String,
    pub from: String,
    pub subject: String,
    #[serde(rename = "receivedAt", default)]
    pub received_at: Option<f64>,
    #[serde(rename = "bodyPreview", default)]
    pub body_preview: Option<String>,
    #[serde(rename = "attachmentsCount", default)]
    pub attachments_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentInfo {
    #[serde(rename = "_id", default)]
    pub id: Option<String>,
    #[serde(rename = "attachmentId", default)]
    pub attachment_id: Option<i32>,
    pub filename: Option<String>,
    pub size: Option<i64>,
    pub mimetype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDetail {
    pub _id: String,
    pub from: String,
    pub subject: String,
    #[serde(rename = "receivedAt", default)]
    pub received_at: Option<f64>,
    #[serde(rename = "bodyPreview", default)]
    pub body_preview: Option<String>,
    #[serde(rename = "bodyHtml", default)]
    pub body_html: Option<String>,
    #[serde(rename = "attachmentsCount", default)]
    pub attachments_count: Option<i32>,
    #[serde(default)]
    pub attachments: Vec<AttachmentInfo>,
}

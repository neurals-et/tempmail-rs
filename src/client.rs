use std::time::Duration;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use tokio::time::sleep;

use crate::error::{Result, TempMailError};
use crate::models::{Mailbox, MessageDetail, MessagesResponse, MessageSummary};

pub const DEFAULT_BASE_URL: &str = "https://mob2.temp-mail.org";
pub const DEFAULT_USER_AGENT: &str = "TempMail/3.14.0 (Android 15)";

#[derive(Clone, Debug)]
pub struct TempMailClient {
    client: reqwest::Client,
    base_url: String,
    user_agent: String,
    pub email: Option<String>,
    pub token: Option<String>,
}

impl Default for TempMailClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TempMailClient {
    pub fn new() -> Self {
        Self::with_config(DEFAULT_BASE_URL, DEFAULT_USER_AGENT)
    }

    pub fn with_config(base_url: &str, user_agent: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            user_agent: user_agent.to_string(),
            email: None,
            token: None,
        }
    }

    pub fn with_credentials(token: String, email: String) -> Self {
        let mut client = Self::new();
        client.token = Some(token);
        client.email = Some(email);
        client
    }

    fn headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(
            USER_AGENT,
            HeaderValue::from_str(&self.user_agent).unwrap_or(HeaderValue::from_static("TempMail")),
        );
        h.insert(ACCEPT, HeaderValue::from_static("application/json"));
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(token) = &self.token {
            if let Ok(val) = HeaderValue::from_str(token) {
                h.insert(AUTHORIZATION, val);
            }
        }
        h
    }

    /// Generate a fresh disposable mailbox and receive an authentication token.
    pub async fn new_mailbox(&mut self) -> Result<Mailbox> {
        let url = format!("{}/mailbox", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .body("{}")
            .send()
            .await?;

        if !resp.status().is_success() {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TempMailError::Api { code, message: text });
        }

        let mb: Mailbox = resp.json().await?;
        self.email = Some(mb.mailbox.clone());
        self.token = mb.token.clone();
        Ok(mb)
    }

    /// Verify active mailbox status.
    pub async fn get_mailbox(&self) -> Result<Mailbox> {
        if self.token.is_none() {
            return Err(TempMailError::NoActiveMailbox);
        }

        let url = format!("{}/mailbox", self.base_url);
        let resp = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !resp.status().is_success() {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TempMailError::Api { code, message: text });
        }

        Ok(resp.json().await?)
    }

    /// List all messages in current mailbox inbox.
    pub async fn get_messages(&self, after_id: Option<&str>) -> Result<Vec<MessageSummary>> {
        if self.token.is_none() {
            return Err(TempMailError::NoActiveMailbox);
        }

        let mut url = format!("{}/messages", self.base_url);
        if let Some(after) = after_id {
            url.push_str(&format!("?after={}", after));
        }

        let resp = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !resp.status().is_success() {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TempMailError::Api { code, message: text });
        }

        let data: MessagesResponse = resp.json().await?;
        Ok(data.messages)
    }

    /// Retrieve full message details (including HTML content and attachments).
    pub async fn get_message_detail(&self, email_id: &str) -> Result<MessageDetail> {
        if self.token.is_none() {
            return Err(TempMailError::NoActiveMailbox);
        }

        let url = format!("{}/messages/{}/", self.base_url, email_id);
        let resp = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .await?;

        if !resp.status().is_success() {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TempMailError::Api { code, message: text });
        }

        Ok(resp.json().await?)
    }

    /// Retrieve raw RFC822/EML message source.
    pub async fn get_message_source(&self, email_id: &str) -> Result<String> {
        if self.token.is_none() {
            return Err(TempMailError::NoActiveMailbox);
        }

        let url = format!("{}/messages/{}/source", self.base_url, email_id);
        let mut headers = self.headers();
        headers.insert(ACCEPT, HeaderValue::from_static("application/octet-stream"));

        let resp = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?;

        if !resp.status().is_success() {
            let code = resp.status().as_u16();
            let text = resp.text().await.unwrap_or_default();
            return Err(TempMailError::Api { code, message: text });
        }

        Ok(resp.text().await?)
    }

    /// Poll for an incoming email and parse any 4-8 digit OTP code.
    pub async fn wait_for_otp(&self, timeout_secs: u64, interval_secs: u64) -> Result<String> {
        if self.token.is_none() {
            return Err(TempMailError::NoActiveMailbox);
        }

        let re = Regex::new(r"\b\d{4,8}\b").unwrap();
        let start = tokio::time::Instant::now();
        let timeout = Duration::from_secs(timeout_secs);

        while start.elapsed() < timeout {
            if let Ok(messages) = self.get_messages(None).await {
                for msg in messages {
                    let combined = format!("{} {}", msg.subject, msg.body_preview.unwrap_or_default());
                    if let Some(mat) = re.find(&combined) {
                        return Ok(mat.as_str().to_string());
                    }
                }
            }
            sleep(Duration::from_secs(interval_secs)).await;
        }

        Err(TempMailError::Timeout)
    }
}

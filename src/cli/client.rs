//! HTTP client for Level API

use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};

/// Level API client
pub struct LevelClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl LevelClient {
    /// Create a new client
    pub fn new(base_url: &str, api_key: Option<&str>) -> Result<Self> {
        let client = Client::builder()
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.map(|s| s.to_string()),
        })
    }

    /// Build a request with auth headers
    fn request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);

        if let Some(key) = &self.api_key {
            req = req.header("X-API-Key", key);
        }

        req
    }

    /// GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .request(reqwest::Method::GET, path)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// POST request with JSON body
    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let response = self
            .request(reqwest::Method::POST, path)
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// PUT request with JSON body
    pub async fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        let response = self
            .request(reqwest::Method::PUT, path)
            .json(body)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .request(reqwest::Method::DELETE, path)
            .send()
            .await
            .context("Failed to send request")?;

        self.handle_response(response).await
    }

    /// Handle response and parse JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();

            // Try to parse as JSON error
            if let Ok(error) = serde_json::from_str::<serde_json::Value>(&error_text) {
                if let Some(message) = error.get("message").and_then(|m| m.as_str()) {
                    anyhow::bail!("{} ({})", message, status);
                }
            }

            anyhow::bail!("Request failed: {} - {}", status, error_text);
        }

        let body = response
            .json::<T>()
            .await
            .context("Failed to parse response")?;

        Ok(body)
    }

    // Convenience methods for common operations

    /// List SOWs
    pub async fn list_sows(&self) -> Result<Vec<serde_json::Value>> {
        #[derive(serde::Deserialize)]
        struct SowListResponse {
            sows: Vec<serde_json::Value>,
        }
        let resp: SowListResponse = self.get("/sows").await?;
        Ok(resp.sows)
    }

    /// Get SOW by ID
    pub async fn get_sow(&self, sow_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("/sow/{}", sow_id)).await
    }

    /// Create SOW
    pub async fn create_sow(
        &self,
        short_name: &str,
        title: &str,
        description: Option<&str>,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "short_name": short_name,
            "title": title,
            "description": description,
        });
        self.post("/sows", &body).await
    }

    /// List nouns in SOW
    pub async fn list_nouns(
        &self,
        sow_id: &str,
        noun_type: Option<&str>,
        state: Option<&str>,
        blocked: Option<bool>,
    ) -> Result<serde_json::Value> {
        let mut path = format!("/nouns/{}/nouns?", sow_id);

        if let Some(t) = noun_type {
            path.push_str(&format!("type={}&", t));
        }
        if let Some(s) = state {
            path.push_str(&format!("state={}&", s));
        }
        if let Some(b) = blocked {
            path.push_str(&format!("blocked={}&", b));
        }

        self.get(&path).await
    }

    /// Get noun by ID
    pub async fn get_noun(&self, sow_id: &str, noun_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("/nouns/{}/noun/{}", sow_id, noun_id)).await
    }

    /// Create noun
    pub async fn create_noun(
        &self,
        sow_id: &str,
        noun_type: &str,
        title: &str,
        description: Option<&str>,
        parent_id: Option<&str>,
        due_date: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut body = serde_json::json!({
            "type": noun_type,
            "title": title,
        });

        if let Some(d) = description {
            body["description"] = serde_json::json!(d);
        }
        if let Some(p) = parent_id {
            body["parent_id"] = serde_json::json!(p);
        }
        if let Some(due) = due_date {
            body["due_date"] = serde_json::json!(due);
        }

        self.post(&format!("/nouns/{}/nouns", sow_id), &body).await
    }

    /// Update noun
    pub async fn update_noun(
        &self,
        sow_id: &str,
        noun_id: &str,
        updates: serde_json::Value,
    ) -> Result<serde_json::Value> {
        self.put(&format!("/nouns/{}/noun/{}", sow_id, noun_id), &updates).await
    }

    /// Delete noun
    pub async fn delete_noun(&self, sow_id: &str, noun_id: &str) -> Result<serde_json::Value> {
        self.delete(&format!("/nouns/{}/noun/{}", sow_id, noun_id)).await
    }

    /// Get noun children
    pub async fn get_children(&self, sow_id: &str, noun_id: &str) -> Result<Vec<serde_json::Value>> {
        self.get(&format!("/nouns/{}/noun/{}/children", sow_id, noun_id)).await
    }

    /// Get noun transactions
    pub async fn get_transactions(&self, sow_id: &str, noun_id: &str) -> Result<serde_json::Value> {
        self.get(&format!("/nouns/{}/noun/{}/transactions", sow_id, noun_id)).await
    }

    /// Apply verb to noun
    pub async fn apply_verb(
        &self,
        sow_id: &str,
        noun_id: &str,
        verb: &str,
        context: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "verb": verb,
            "context": context,
        });
        self.post(&format!("/nouns/{}/noun/{}/transactions", sow_id, noun_id), &body).await
    }

    /// Batch verb application
    pub async fn batch_verb(
        &self,
        sow_id: &str,
        noun_ids: Vec<String>,
        verb: &str,
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "noun_ids": noun_ids,
            "verb": verb,
        });
        self.post(&format!("/nouns/{}/batch/transactions", sow_id), &body).await
    }

    /// Get timeline view
    pub async fn timeline(
        &self,
        sow_id: &str,
        start: Option<&str>,
        end: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut path = format!("/views/{}/timeline?", sow_id);
        if let Some(s) = start {
            path.push_str(&format!("start={}&", s));
        }
        if let Some(e) = end {
            path.push_str(&format!("end={}&", e));
        }
        self.get(&path).await
    }

    /// Get kanban view
    pub async fn kanban(&self, sow_id: &str, container_id: Option<&str>) -> Result<serde_json::Value> {
        let mut path = format!("/views/{}/kanban?", sow_id);
        if let Some(c) = container_id {
            path.push_str(&format!("container_id={}&", c));
        }
        self.get(&path).await
    }

    /// Get calendar view
    pub async fn calendar(&self, sow_id: &str, month: Option<&str>) -> Result<serde_json::Value> {
        let mut path = format!("/views/{}/calendar?", sow_id);
        if let Some(m) = month {
            path.push_str(&format!("month={}&", m));
        }
        self.get(&path).await
    }

    /// List aliases
    pub async fn list_aliases(&self) -> Result<Vec<serde_json::Value>> {
        self.get("/user/aliases").await
    }

    /// Create alias
    pub async fn create_alias(&self, name: &str, noun_id: &str) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "name": name,
            "noun_id": noun_id,
        });
        self.post("/user/aliases", &body).await
    }

    /// Delete alias
    pub async fn delete_alias(&self, name: &str) -> Result<serde_json::Value> {
        self.delete(&format!("/user/aliases/{}", name)).await
    }

    /// Get user preferences
    pub async fn get_preferences(&self) -> Result<serde_json::Value> {
        self.get("/user/preferences").await
    }

    /// Update user preferences
    pub async fn update_preferences(&self, updates: serde_json::Value) -> Result<serde_json::Value> {
        self.put("/user/preferences", &updates).await
    }

    /// Help: noun types
    pub async fn help_nouns(&self, noun_type: Option<&str>) -> Result<serde_json::Value> {
        match noun_type {
            Some(t) => self.get(&format!("/help/nouns/{}", t)).await,
            None => self.get("/help/nouns").await,
        }
    }

    /// Help: verbs
    pub async fn help_verbs(&self, verb: Option<&str>) -> Result<serde_json::Value> {
        match verb {
            Some(v) => self.get(&format!("/help/verbs/{}", v)).await,
            None => self.get("/help/verbs").await,
        }
    }

    /// Help: containers
    pub async fn help_containers(&self) -> Result<serde_json::Value> {
        self.get("/help/containers").await
    }

    /// Help: roles
    pub async fn help_roles(&self) -> Result<serde_json::Value> {
        self.get("/help/roles").await
    }

    /// Help: states
    pub async fn help_states(&self) -> Result<serde_json::Value> {
        self.get("/help/states").await
    }
}

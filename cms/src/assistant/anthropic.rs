//! The model of the recipe assistant on Anthropic's Messages API; the
//! tests use a fake [`Llm`].

use std::time::Duration;

use serde_json::{Value, json};

const URL: &str = "https://api.anthropic.com/v1/messages";
const VERSION: &str = "2023-06-01";
const TIMEOUT: Duration = Duration::from_secs(120);
pub const DEFAULT_MODEL: &str = "claude-haiku-5-5";

/// What the assistant asks: the instructions, the user's content blocks
/// (text, later images) and the JSON schema of the answer.
#[derive(Debug, Clone)]
pub struct LlmRequest {
	pub system: String,
	pub content: Vec<Value>,
	pub schema: Value,
}

/// A model: a request in, the JSON text of its answer out.
#[async_trait::async_trait]
pub trait Llm: Send + Sync {
	async fn complete(&self, req: LlmRequest) -> Result<String, String>;
}

pub struct Anthropic {
	key: String,
	model: String,
	client: reqwest::Client,
}

impl Anthropic {
	pub fn new(key: String, model: String) -> Self {
		let client = reqwest::Client::builder()
			.timeout(TIMEOUT)
			.build()
			.expect("an http client");
		Self { key, model, client }
	}
}

/// The body of a Messages request: the answer is JSON of the schema
/// (structured outputs), little thinking for a quick answer.
pub fn body(model: &str, req: &LlmRequest) -> Value {
	json!({
		"model": model,
		"max_tokens": 8000,
		"system": req.system,
		"messages": [{ "role": "user", "content": req.content }],
		"output_config": {
			"effort": "low",
			"format": { "type": "json_schema", "schema": req.schema },
		},
	})
}

#[async_trait::async_trait]
impl Llm for Anthropic {
	async fn complete(&self, req: LlmRequest) -> Result<String, String> {
		let resp = self
			.client
			.post(URL)
			.header("x-api-key", &self.key)
			.header("anthropic-version", VERSION)
			.header("content-type", "application/json")
			.body(body(&self.model, &req).to_string())
			.send()
			.await
			.map_err(|e| format!("anthropic: {e}"))?;
		let status = resp.status();
		let text = resp.text().await.map_err(|e| format!("anthropic: {e}"))?;
		if !status.is_success() {
			return Err(format!("anthropic {status}: {text}"));
		}
		let v: Value = serde_json::from_str(&text)
			.map_err(|e| format!("anthropic answer: {e}"))?;
		let stop = v["stop_reason"].as_str().unwrap_or("");
		if stop == "refusal" || stop == "max_tokens" {
			return Err(format!("anthropic stopped: {stop}"));
		}
		// the thinking blocks come first
		v["content"]
			.as_array()
			.and_then(|c| c.iter().find(|b| b["type"] == "text"))
			.and_then(|b| b["text"].as_str())
			.map(str::to_string)
			.ok_or_else(|| format!("anthropic without text: {text}"))
	}
}

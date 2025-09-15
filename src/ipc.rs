use crate::error::{AppError, Result};
use crate::service;
use log::{debug, error, trace};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use wry::webview::WebView;

pub struct Params {
    inner: Map<String, serde_json::Value>,
}

impl Params {
    pub fn new(inner: Map<String, serde_json::Value>) -> Self {
        Self { inner }
    }

    pub fn str(&self, key: &str) -> Result<String> {
        let Some(value) = self.inner.get(key) else {
            return Err(AppError::NoParam(key.to_string()));
        };
        value
            .as_str()
            .map(|s| s.to_string())
            .ok_or(AppError::BadParam(key.to_string()))
    }

    pub fn i32(&self, key: &str) -> Result<i32> {
        let Some(value) = self.inner.get(key) else {
            return Err(AppError::NoParam(key.to_string()));
        };
        value
            .as_i64()
            .map(|i| i as i32)
            .ok_or(AppError::BadParam(key.to_string()))
    }
}

#[derive(Debug, Deserialize)]
struct IpcRequest {
    #[serde(rename = "transactionId")]
    transaction_id: u64,
    #[serde(rename = "type")]
    type_name: String,
    parameters: Map<String, Value>,
}

#[derive(Debug, Serialize)]
struct IpcResponse {
    #[serde(rename = "transactionId")]
    transaction_id: u64,
    #[serde(rename = "type")]
    type_name: &'static str,
    value: Value,
}

pub fn ipc_handler(webview: &WebView, ipc_message: String) {
    trace!("ipc_handler(webview: &WebView, ipc_message: String)");
    debug!("ipc message: {ipc_message}");

    let request: IpcRequest = match serde_json::from_str(&ipc_message) {
        Ok(request) => request,
        Err(error) => {
            error!("fail to parse ipc request: {error}");
            return;
        }
    };

    let parameters = Params::new(request.parameters);
    let value = match request.type_name.as_str() {
        "console" => json(service::console(parameters)),
        "Greet" => json(service::greet(parameters)),
        "IncrementCounter" => json(service::increment_counter(parameters)),
        "DecrementCounter" => json(service::decrement_counter(parameters)),
        "UpdateCounter" => json(service::update_counter(parameters)),
        "GetTime" => json(service::get_current_time(parameters)),
        _ => {
            error!("unknown ipc request type {}", request.type_name);
            return;
        }
    };

    let response = IpcResponse {
        transaction_id: request.transaction_id,
        type_name: value.0,
        value: value.1,
    };
    let response_json = serde_json::to_string(&response).unwrap();
    let script = format!("window.rpc.handleResponse({response_json})");
    debug!("response script: {script}");
    if let Err(error) = webview.evaluate_script(&script) {
        error!("fail to evaluate script: {error}");
    }
}

fn json<T: Serialize>(result: Result<T>) -> (&'static str, Value) {
    match result {
        Ok(result) => (
            get_type_name(&result),
            serde_json::to_value(&result).unwrap(),
        ),
        Err(error) => {
            error!("fail to serialize result: {error}");
            ("Error", serde_json::to_value(()).unwrap())
        }
    }
}

fn get_type_name<T>(value: &T) -> &'static str {
    match std::any::type_name_of_val(value)
        .split("::")
        .last()
        .unwrap_or("Unknown")
    {
        "()" => "Void",
        "str" => "String",
        other => other,
    }
}

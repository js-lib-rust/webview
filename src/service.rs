use crate::error::Result;
use crate::ipc::Params;
use log::{debug, error, info, trace, warn};
use serde::Serialize;

pub fn console(params: Params) -> Result<()> {
    trace!("console(params: Params) -> Result<()>");

    let level = params.str("level")?;
    debug!("level: {level}");
    let message = params.str("message")?;
    debug!("message: {message}");

    match level.as_str() {
        "log" => info!("{message}"),
        "error" => error!("{message}"),
        "warn" => warn!("{message}"),
        "info" => info!("{message}"),
        "debug" => debug!("{message}"),
        "trace" => trace!("{message}"),
        _ => error!("unknown level {level} for message {message}"),
    }
    Ok(())
}

#[derive(Debug, Serialize)]
pub struct User {
    name: String,
    age: u8,
}

pub fn greet(params: Params) -> Result<User> {
    trace!("greet(params: Params) -> Result<User>");

    let name = params.str("name")?;
    debug!("name: {name}");

    info!("greeting from {name}");
    Ok(User { name, age: 29 })
}

pub fn increment_counter(params: Params) -> Result<i32> {
    trace!("increment_counter(params: Params) -> Result<i32>");

    let value = params.i32("value")?;
    debug!("value: {value}");

    Ok(value + 1)
}

pub fn decrement_counter(params: Params) -> Result<i32> {
    trace!("decrement_counter(params: Params) -> Result<i32>");

    let value = params.i32("value")?;
    debug!("value: {value}");

    Ok(value - 1)
}

pub fn update_counter(params: Params) -> Result<()> {
    trace!("update_counter(params: Params) -> Result<()>");

    let value = params.i32("value")?;
    debug!("value: {value}");

    Ok(())
}

pub fn get_current_time(_params: Params) -> Result<String> {
    trace!("get_current_time(_params: Params) -> Result<(String)>");

    let time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    debug!("time: {time}");
    Ok(time)
}

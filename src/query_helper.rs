use crate::errors;
/// Use the chrono library to keep times consistent
use chrono::{DateTime, Duration, SecondsFormat, Utc};
use serde_json::{Result, Value};
use std::collections::HashMap;
use std::env;

pub const BOT_HEROKU_QUERY_URL: &'static str = "BOT_HEROKU_QUERY_URL";
pub const BOT_HEROKU_AUTH_TOKEN: &'static str = "BOT_HEROKU_AUTH_TOKEN";
pub const LAST_QUERY_TIMESTAMP: &'static str = "LAST_QUERY_TIMESTAMP";
pub const QUERY_INTERVAL_IN_HOURS: i64 = 1;

/// On a fresh server start, LAST_QUERY_TIMESTAMP will be unset, set that
/// to the current timestamp in that case and query every QUERY_INTERVAL_IN_HOURS hours.
/// For an existing session, use the last stored value and update it with
/// the timestamp of the current query (if applicable)
/// Returns the string that will be used by the github client to construct the query url.
pub fn check_query_viability() -> Option<String> {
    let now = Utc::now();

    if let Ok(last_query_ts_str) = get_config_var(LAST_QUERY_TIMESTAMP) {
        let last_query_ts = DateTime::parse_from_rfc3339(&last_query_ts_str).unwrap();
        if now
            >= (last_query_ts.checked_add_signed(Duration::hours(QUERY_INTERVAL_IN_HOURS))).unwrap()
        {
            let new_last_query_ts_str = now.to_rfc3339_opts(SecondsFormat::Secs, true);
            info!(
                "Updating LAST_QUERY_TIMESTAMP to {:?}",
                new_last_query_ts_str
            );
            set_config_var(LAST_QUERY_TIMESTAMP, new_last_query_ts_str);
            return Some(last_query_ts_str);
        } else {
            return None;
        }
    }
    let last_query_ts_str = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    info!(
        "No last query timestamp found. Updating LAST_QUERY_TIMESTAMP to {:?}",
        last_query_ts_str
    );
    set_config_var(LAST_QUERY_TIMESTAMP, last_query_ts_str);

    None
}

fn get_config_var(name: &str) -> errors::GenResult<String> {
    let client = reqwest::blocking::Client::new();
    let auth_token = env::var(BOT_HEROKU_AUTH_TOKEN).unwrap();
    let query_url = env::var(BOT_HEROKU_QUERY_URL).unwrap();
    let raw_var_value = client
        .get(&query_url)
        .header("Accept", "application/vnd.heroku+json;version=3")
        .header("Authorization", format!("Basic {}", auth_token))
        .send()
        .unwrap()
        .text()
        .unwrap();

    let raw_configs: Result<HashMap<String, Value>> = serde_json::from_str(&raw_var_value);
    match raw_configs {
        Ok(configs) => {
            let config_value: String = serde_json::from_value(configs[name].clone()).unwrap();
            return Ok(config_value);
        }
        Err(e) => return Err(Box::new(e)),
    }
}

fn set_config_var(name: &str, value: String) {
    let client = reqwest::blocking::Client::new();
    let auth_token = env::var(BOT_HEROKU_AUTH_TOKEN).unwrap();
    let query_url = env::var(BOT_HEROKU_QUERY_URL).unwrap();
    let body = format!("{{ \"{}\" : \"{}\" }}", name, value);
    let _res = client
        .post(&query_url)
        .header("X-Http-Method-Override", "PATCH")
        .header("Content-Type", "application/json")
        .header("Accept", "application/vnd.heroku+json;version=3")
        .header("Authorization", format!("Basic {}", auth_token))
        .body(body)
        .send()
        .unwrap();
}

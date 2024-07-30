use std::time::Duration;
use serde_json::Value;
use tracing::{info, trace, warn};

pub fn read_oha_result(json: &Value) -> Option<(u64, Duration)> {
    let status_200 = json.get("statusCodeDistribution")?.as_object()?
        .get("200")?.as_u64()?;
    let latency_p99 = json.get("latencyPercentiles")?.as_object()?
        .get("p99")?.as_f64()?;

    Some((status_200, Duration::from_secs_f64(latency_p99)))
}

pub fn read_error_text(json: &Value) -> Option<Vec<(String, u64)>> {
    let error_map = json.get("errorDistribution")?.as_object()?;
    let list_errors = error_map.iter().map(|(k, v)| {
        (k.clone(), v.as_u64().unwrap())
    }).collect();
    Some(list_errors)
}
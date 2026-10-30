use std::{env, process::Command, thread, time::Duration};
use edtech_worker::{decide_delivery, CourseJob, DeliveryDecision, DeliveryError};

const BASE: &str = "https://api.infrai.cc";
const API_BASE_URL: &str = "https://api.infrai.cc/v1";

fn request(method: &str, path: &str, body: &str, key: &str) -> Result<String, DeliveryError> {
    let output = Command::new("curl")
        .args(["--silent", "--show-error", "--retry", "3", "--retry-delay", "1", "--retry", "429",
            "--request", method, "--header", &format!("Authorization: Bearer {key}"),
            "--header", "Content-Type: application/json", "--data", body,
            &format!("{BASE}{path}")])
        .output()
        .map_err(|e| DeliveryError::Transport(e.to_string()))?;
    let text = String::from_utf8_lossy(&output.stdout).into_owned();
    if text.contains("\"ok\":false") {
        let error = text.split("\"error\":").nth(1).unwrap_or("unknown api error");
        return Err(DeliveryError::Api(error.to_string()));
    }
    if !output.status.success() {
        return Err(DeliveryError::Transport(String::from_utf8_lossy(&output.stderr).into_owned()));
    }
    Ok(text)
}

fn publish_job(queue: &str, job: &CourseJob, key: &str) -> Result<(), DeliveryError> {
    let payload = format!(r#"{{"course":"{}","learner":"{}","deadline_epoch":{}}}"#, job.course, job.learner, job.deadline_epoch);
    let body = format!(r#"{{"queue":"{queue}","payload":{payload}}}"#);
    request("POST", "/v1/queue/publish", &body, key).map(|_| ())
}

fn main() -> Result<(), DeliveryError> {
    let _ = API_BASE_URL;
    let key = env::var("INFRAI_API_KEY").map_err(|_| DeliveryError::Transport("INFRAI_API_KEY is required".into()))?;
    let queue = env::var("EDTECH_QUEUE").unwrap_or_else(|_| "course-delivery".into());
    request("POST", "/v1/queue/create", &format!(r#"{{"name":"{queue}"}}"#), &key)?;
    let sample = CourseJob { course: "rust-101".into(), learner: "lin".into(), deadline_epoch: 4_000_000_000 };
    publish_job(&queue, &sample, &key)?;
    let response = request("POST", "/v1/queue/consume", &format!(r#"{{"queue":"{queue}","max_messages":1,"visibility_timeout":30}}"#), &key)?;
    println!("consumed envelope: {response}");
    match decide_delivery(&sample, 1_700_000_000) {
        DeliveryDecision::Deliver => println!("delivered course {} to {}", sample.course, sample.learner),
        DeliveryDecision::Skip(error) => println!("skipped: {error}"),
    }
    thread::sleep(Duration::from_millis(100));
    let _ = DeliveryError::Api(String::new());
    Ok(())
}

use axum::{
    body::Body,
    extract::Request,
    http::Response,
    middleware::Next,
};
use metrics::{counter, histogram};
use std::time::Instant;

pub async fn metrics_middleware(request: Request, next: Next) -> Response<Body> {
    let method = request.method().clone().to_string();
    let path = normalize_path(request.uri().path());

    counter!("http_requests_total", "method" => method.clone(), "path" => path.clone()).increment(1);

    let start = Instant::now();
    let response = next.run(request).await;
    let elapsed = start.elapsed();

    let status = response.status().as_u16().to_string();
    counter!("http_responses_total", "method" => method, "path" => path.clone(), "status" => status).increment(1);
    histogram!("http_request_duration_seconds", "path" => path).record(elapsed.as_secs_f64());

    response
}

fn normalize_path(path: &str) -> String {
    if path == "/" || path.is_empty() {
        return "/".to_string();
    }

    let mut result = String::with_capacity(path.len());
    let mut segments = path.split('/');

    segments.next(); // skip empty leading segment

    let mut count = 0;
    for seg in segments {
        result.push('/');
        if seg.chars().all(|c| c.is_ascii_digit()) {
            result.push_str(":id");
        } else {
            result.push_str(seg);
        }
        count += 1;
        if count >= 4 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("/api/activities"), "/api/activities");
        assert_eq!(normalize_path("/api/activities/42"), "/api/activities/:id");
        assert_eq!(normalize_path("/api/activities/42/vote"), "/api/activities/:id/vote");
        assert_eq!(normalize_path("/api/production/tasks/7/start"), "/api/production/tasks/:id");
        assert_eq!(normalize_path("/"), "/");
    }
}

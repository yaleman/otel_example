use opentelemetry_otlp::Protocol;

pub mod metrics;
pub mod things;

pub const MAX_LEWPS: u64 = 500;

/// if you set the OTLP_ENDPOINT env var you can send this elsewhere
pub fn get_otlp_endpoint() -> String {
    std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string())
}
/// if you set the OTLP_ENDPOINT env var you can send this elsewhere
pub fn get_otlp_protocol() -> Protocol {
    let proto =
        std::env::var("OTLP_PROTOCOL").unwrap_or_else(|_| "http://localhost:4317".to_string());
    match proto.to_lowercase().as_str() {
        "grpc" => Protocol::Grpc,
        "http" => Protocol::HttpBinary,
        _ => Protocol::HttpBinary,
    }
}

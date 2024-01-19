use opentelemetry::KeyValue;
use opentelemetry_sdk::runtime::Tokio;
use otel_example::{
    get_otlp_endpoint,
    things::{do_other_stuff_loop, do_stuff_loop},
};
use std::time::Duration;

use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Sampler},
    Resource,
};
use tracing::error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&get_otlp_endpoint())
                .with_timeout(Duration::from_secs(5))
                .with_protocol(Protocol::HttpBinary),
        )
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_id_generator(RandomIdGenerator::default())
                .with_max_events_per_span(64)
                .with_max_attributes_per_span(16)
                .with_max_events_per_span(16)
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "example",
                )])),
        )
        .install_batch(Tokio)
        .map_err(|err| eprintln!("Failed to start OTLP pipeline: {:?}", err))?;

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_threads(true);

    let meter_provider = otel_example::metrics::init_metrics()
        .map_err(|err| eprintln!("failed to start metrics provider: {:?}", err))?;

    let forest_layer = tracing_forest::ForestLayer::default();

    let subscriber = Registry::default().with(telemetry).with(forest_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // let meter = meter_provider.meter("example");
    // let counter = meter.u64_counter("sleeps").init();

    let do_stuff_handle = tokio::spawn(do_stuff_loop());

    let do_other_stuff_handle = tokio::spawn(do_other_stuff_loop());

    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        if do_other_stuff_handle.is_finished() && do_stuff_handle.is_finished() {
            break;
        }
    }

    // shut down the metrics pipeline
    if let Err(err) = meter_provider.shutdown() {
        error!("Failed to shut down metrics provider: {:?}", err);
    };
    // finish off the logging pipeline
    opentelemetry_api::global::shutdown_tracer_provider();

    Ok(())

    // Trace executed code
}

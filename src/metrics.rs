use opentelemetry::metrics;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::metrics::MeterProvider;

#[allow(dead_code)]
pub fn init_metrics() -> metrics::Result<MeterProvider> {
    let export_config = opentelemetry_otlp::ExportConfig {
        endpoint: "http://localhost:4318/v1/metrics".to_string(),
        ..opentelemetry_otlp::ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(opentelemetry_sdk::runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .http()
                .with_export_config(export_config),
        )
        .build()
}

// pub fn init_meter_provider() -> MeterProvider {

//     let export_config = opentelemetry_otlp::ExportConfig {
//         endpoint: "http://localhost:4318/v1/metrics".to_string(),
//         ..opentelemetry_otlp::ExportConfig::default()
//     };
//     opentelemetry_otlp::new_pipeline()
//         .metrics(opentelemetry_sdk::runtime::Tokio)
//         .with_exporter(
//             opentelemetry_otlp::new_exporter()
//                 .http()
//                 .with_export_config(export_config),
//         )
//         .build()

//     let exporter = opentelemetry_stdout::MetricsExporterBuilder::default()
//         // uncomment the below lines to pretty print output.
//         //  .with_encoder(|writer, data|
//         //    Ok(serde_json::to_writer_pretty(writer, &data).unwrap()))
//         .build();
//     let reader = PeriodicReader::builder(exporter, runtime::Tokio).build();
//     MeterProvider::builder()
//         .with_reader(reader)
//         .with_resource(Resource::new(vec![KeyValue::new(
//             "service.name",
//             "example",
//         )]))
//         .build()
// }

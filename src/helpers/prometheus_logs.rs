use actix_web_prometheus::{PrometheusMetrics, PrometheusMetricsBuilder};
use std::{collections::HashMap, sync::LazyLock};

pub static PROMETHEUS: LazyLock<PrometheusMetrics> = LazyLock::new(|| {
    let mut labels = HashMap::new();
    labels.insert("service".to_string(), "my_service".to_string());
    PrometheusMetricsBuilder::new("my_app")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap()
});

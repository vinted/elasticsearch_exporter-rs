use elasticsearch::params::Time;

pub(crate) const SUBSYSTEM: &'static str = "cat_pending_tasks";

async fn metrics(exporter: &Exporter) -> Result<Vec<Metrics>, elasticsearch::Error> {
    let response = exporter
        .client
        .cat()
        .pending_tasks()
        .format("json")
        .h(&["*"])
        // Return local information, do not retrieve the state from master node (default: false)
        .local(true)
        .time(Time::Ms)
        .request_timeout(exporter.options.elasticsearch_global_timeout)
        .send()
        .await?;

    Ok(metric::from_values(response.json::<Vec<Value>>().await?))
}

crate::poll_metrics!();

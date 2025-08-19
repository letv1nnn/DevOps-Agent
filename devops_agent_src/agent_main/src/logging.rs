use tracing_subscriber::fmt::Subscriber;

pub fn init_logging() {
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::fs::File::create("agent.log").unwrap())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
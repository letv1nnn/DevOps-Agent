use std::fs::OpenOptions;
use tracing_subscriber::{
    fmt::writer::Tee, FmtSubscriber
};

pub fn init_logging() {

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("agent.log")
        .expect("Failed to open log file");

    let tee = Tee::new(std::io::stdout, file);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .with_writer(tee)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}


#[cfg(test)]
mod tests {

    use tokio::process::Command;

    #[tokio::test]
    async fn log_file_test() {
        let output = Command::new("cat")
            .arg("../agent.log")
            .output()
            .await
            .expect("Failed to read the log file");

        assert_eq!(output.status.success(), true);
    }
}

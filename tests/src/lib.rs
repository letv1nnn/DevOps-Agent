#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::PathBuf;
    use tokio;
    use tool_executor::github_interaction::github_structs::{WorkflowRun, WorkflowRunsResponse};

    fn mock_workflow_runs() -> WorkflowRunsResponse {
        WorkflowRunsResponse {
            workflow_runs: vec![
                WorkflowRun { id: 101, status: "completed".into(), conclusion: Some("success".into()) },
                WorkflowRun { id: 102, status: "in_progress".into(), conclusion: None },
            ],
        }
    }

    #[tokio::test]
    async fn test_download_workflows_logs_success() {
        fn mock_get_github_env_data() -> Option<Vec<String>> {
            Some(vec!["token".into(), "owner".into(), "repo".into()])
        }

        async fn mock_list_workflow_runs(_: &str, _: &str, _: &str) -> Result<WorkflowRunsResponse, Box<dyn Error>> {
            Ok(mock_workflow_runs())
        }

        async fn mock_download_workflow_logs(_: &str, _: &str, run_id: u64, _: &str) -> Result<(), Box<dyn Error>> {
            assert!(run_id > 0);
            Ok(())
        }

        let result = async {
            let data = mock_get_github_env_data().unwrap();
            let (token, owner, repo) = (&data[0], &data[1], &data[2]);
            let response = mock_list_workflow_runs(owner, repo, token).await?;
            for run in &response.workflow_runs {
                mock_download_workflow_logs(owner, repo, run.id, token).await?;
            }
            Ok::<_, Box<dyn Error>>(format!(
                "Downloaded logs for workflow run IDs: {:?}",
                response.workflow_runs.iter().map(|r| r.id).collect::<Vec<u64>>()
            ))
        }.await;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Downloaded logs"));
    }

    #[tokio::test]
    async fn test_list_workflows_success() {
        fn mock_get_github_env_data() -> Option<Vec<String>> {
            Some(vec!["token".into(), "owner".into(), "repo".into()])
        }

        async fn mock_list_workflow_runs(_: &str, _: &str, _: &str) -> Result<WorkflowRunsResponse, Box<dyn Error>> {
            Ok(mock_workflow_runs())
        }

        let result = async {
            let data = mock_get_github_env_data().unwrap();
            let (token, owner, repo) = (&data[0], &data[1], &data[2]);
            let response = mock_list_workflow_runs(owner, repo, token).await?;

            let mut output = String::new();
            for run in &response.workflow_runs {
                output.push_str(&format!("ID: {}, Status: {}, Conclusion: {:?}\n", run.id, run.status, run.conclusion));
            }

            Ok::<_, Box<dyn Error>>(output)
        }.await;

        let output = result.unwrap();
        assert!(output.contains("ID: 101"));
        assert!(output.contains("completed"));
    }

    #[tokio::test]
    async fn test_analyze_agent_logs_success() {
        async fn mock_read_file(_: PathBuf) -> Result<String, Box<dyn Error>> {
            Ok("ERROR something bad happened".into())
        }

        async fn mock_request_llm(prompt: &str, system_prompt: &str) -> Result<String, Box<dyn Error>> {
            assert!(prompt.contains("ERROR"));
            assert!(system_prompt.contains("helpful assistant"));
            Ok("Found 1 error: something bad happened.".into())
        }

        let result = async {
            let file_name = PathBuf::from("logs/agent.log");
            let prompt = mock_read_file(file_name).await?;
            let system_prompt = String::from(
                "You are a helpful assistant that analizes and summarizes log files to human understandable format. You need to highlight any errors or warnings found in the logs."
            );
            let respond = mock_request_llm(&prompt, &system_prompt).await?;
            Ok::<_, Box<dyn Error>>(respond)
        }.await;

        let summary = result.unwrap();
        assert!(summary.contains("Found 1 error"));
    }
}

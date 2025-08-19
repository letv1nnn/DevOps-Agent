#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod llm_plan_integration_tests {
        
        use agent_plan::llm_plan_integration::{llm_prompt_validation, sanitize_llm_response, write_prompt_to_json_file};
        use tempfile::NamedTempFile;

        #[tokio::test]
        async fn test_sanitize_llm_response() {
            let cases = vec![
                ("```json\n{\"test\": 1}\n```", "{\"test\": 1}"),
                ("```\n{\"test\": 1}\n```", "{\"test\": 1}"),
                ("{\"test\": 1}", "{\"test\": 1}"),
            ];

            for (input, expected) in cases {
                assert_eq!(sanitize_llm_response(input), expected);
            }
        }

        #[tokio::test]
        async fn test_llm_prompt_validation_valid() {
            let valid_plan = r#"
            [
                {
                    "task_type": "Lint",
                    "command": "cargo",
                    "args": ["clippy"],
                    "retry_on_failure": false,
                    "dir": "/path/to/project"
                }
            ]
            "#;
            
            let result = llm_prompt_validation(valid_plan).await;
            assert!(result.is_ok());
        }

        #[tokio::test]
        async fn test_llm_prompt_validation_invalid() {
            let invalid_cases = vec![
                // Missing field
                r#"[{"task_type": "Lint", "command": "cargo", "args": [], "retry_on_failure": false}]"#,
                // Invalid task type
                r#"[{"task_type": "Invalid", "command": "cargo", "args": [], "retry_on_failure": false, "dir": "/path"}]"#,
                // Forbidden command
                r#"[{"task_type": "Lint", "command": "rm", "args": ["-rf", "/"], "retry_on_failure": false, "dir": "/path"}]"#,
            ];

            for case in invalid_cases {
                let result = llm_prompt_validation(case).await;
                assert!(result.is_err());
            }
        }

        #[tokio::test]
        async fn test_write_prompt_to_json_file() {
            let temp_file = NamedTempFile::new().unwrap();
            let path = temp_file.path().to_str().unwrap();
            let valid_plan = r#"[{"task_type": "Test", "command": "cargo", "args": ["test"], "retry_on_failure": true, "dir": "/path"}]"#;

            let result = write_prompt_to_json_file(path, valid_plan).await;
            assert!(result.is_ok());

            let content = std::fs::read_to_string(path).unwrap();
            assert!(content.contains("\"task_type\": \"Test\""));
        }
    }

    #[cfg(test)]
    mod task_types_tests {
        use agent_plan::task_types_and_workflow_steps::{Task, TaskType};
        use serde_json;

        #[test]
        fn test_task_type_serialization() {
            let types = vec![
                (TaskType::Lint, "\"Lint\""),
                (TaskType::Test, "\"Test\""),
                (TaskType::Build, "\"Build\""),
                (TaskType::Deploy, "\"Deploy\""),
                (TaskType::Rollback, "\"Rollback\""),
            ];

            for (task_type, expected) in types {
                let serialized = serde_json::to_string(&task_type).unwrap();
                assert_eq!(serialized, expected);
            }
        }

        #[test]
        fn test_task_deserialization() {
            let json = r#"
            {
                "task_type": "Build",
                "command": "docker",
                "args": ["build", "-t", "myapp"],
                "retry_on_failure": true,
                "dir": "/project"
            }
            "#;

            let task: Task = serde_json::from_str(json).unwrap();
            assert!(matches!(task.task_type, TaskType::Build));
            assert_eq!(task.command, "docker");
            assert_eq!(task.args, vec!["build", "-t", "myapp"]);
            assert!(task.retry_on_failure);
        }
    }
}
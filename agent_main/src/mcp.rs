use rmcp::{handler::server::tool::ToolRouter, model::*, tool, tool_router, transport::stdio, ServerHandler, ServiceExt};
use rmcp::ErrorData;

use crate::cli::shell_cat;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Server {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl Server {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    // need to brainstorm how I am gonna organize everything,
    // I mean how to orginize tools and maybe  to integrate som db

    // same as with the cli version
    // llm based planning, executing the pipeline, reviewing everything, like agent logs, deployemnt logs, plan, etc.
    // create a sort of a tool that will analize the logs and provide its insights

    #[tool(description = "Some tool")]
    async fn some_tool(&self) -> Result<CallToolResult, ErrorData> {
        
        Ok(CallToolResult::success(vec![

        ]))
    } 

    #[tool(description = "Get agent logs")]
    async fn get_agent_logs(&self) -> Result<CallToolResult, ErrorData> {
        // not to just output the logs, maybe to analize them and provide some insights
        match shell_cat("agent.log").await {
            Ok(logs) => {
                Ok(CallToolResult::success(vec![
                    Content::text(logs)
                ]))
            },
            Err(e) => {
                return Err(ErrorData::new(ErrorCode::RESOURCE_NOT_FOUND, format!("Failed to read logs: {}", e), None));
            }
        }
    }

    #[tool(description = "get the plan")]
    async fn get_plan(&self) -> Result<CallToolResult, ErrorData> {
        // maybe output it in some stylish way 
        // and provided an llm inights about the 
        // plan possiblities of failing, etc.
        
        // alxo need to setup the parameters, like how they are going to be passed, 
        // so it would be able to use any file to print and analize
        let tasks = "tasks.json";

        let path = std::path::Path::new(tasks);
                        
        match tokio::fs::metadata(path).await {
            Ok(_) => {
                match shell_cat(tasks).await {
                    Ok(logs) => {
                        return Ok(CallToolResult::success(vec![
                            Content::text(logs)
                        ]));
                    },
                    Err(e) => {
                        return Err(ErrorData::new(ErrorCode::RESOURCE_NOT_FOUND, format!("Failed to read plan: {}", e), None));
                    },
                };
            },
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!("File does not exist.");
                return Err(ErrorData::new(ErrorCode::RESOURCE_NOT_FOUND, "Plan file not found".to_string(), None));
            }
            Err(err) => {
                println!("Error checking file: {}", err);
                return Err(ErrorData::new(ErrorCode::INTERNAL_ERROR, format!("Error checking file: {}", err), None));
            }
        }
    }
}

// implementaing a trait that provides an abstraction over mcp server behaviour
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            server_info: Implementation { 
                name: "DevOps Agent".to_string(), 
                version: "0.1.0".to_string(),
            },
            instructions: Some("Det DevOps Agent orchestrate your CI/CD pipeline".into()),
            capabilities: ServerCapabilities::builder()
                .enable_logging()
                .enable_tools()
                .build(),
            protocol_version: ProtocolVersion::V_2025_03_26,
        }
    }
    
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let service = Server::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("Error starting server: {}", e);
        })?;

    service.waiting().await?;

    Ok(())
}
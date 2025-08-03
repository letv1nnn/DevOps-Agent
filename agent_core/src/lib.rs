use rmcp::{
    handler::{server::tool::ToolRouter}, 
    model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, 
    tool, tool_handler, tool_router, ErrorData, ServerHandler
};

pub mod task_execution;
pub mod logging;
pub mod plan_generating;

pub struct DevOpsAgent {
    tool_router: ToolRouter<Self>,
}


// MCP SERVER, TEST VERSION

#[tool_router]
impl<'a> DevOpsAgent
where DevOpsAgent: 'static + Send + Sync, {

    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
    
    #[tool(description = "prints information about the agent")]
    pub async fn agent_information(&self) -> Result<CallToolResult, ErrorData> {
        eprintln!("Using \"agent information\" tool.");
        let info = "DevOps Agent is a secure, policy-driven automation assistant designed to orchestrate CI/CD workflows by executing, monitoring, and adapting tasks across environments. It can invoke shell commands, interact with tools like GitHub Actions, validate outputs, and escalate issues—all while maintaining full auditability, modular extensibility, and resilience.";
        Ok(CallToolResult::success(vec![
            Content::text(info.to_string()),
        ]))
    }
}

#[tool_handler]
impl ServerHandler for DevOpsAgent {
    fn get_info(&self) -> ServerInfo {
            let intruction = "MCP Server acts as the central control plane for DevOps Agents, providing authenticated access to tools, policies, and task definitions required for secure and consistent CI/CD execution. It manages agent coordination, enforces operational constraints, and serves as the authoritative source for logging, configuration, and extension modules.".to_string();
        ServerInfo {
            instructions: Some(intruction.into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
use rmcp::{model::*, transport::stdio, ServerHandler, ServiceExt};

pub struct Server {

}

impl Server {
    fn new() -> Self {
        Self {}
    }
    // need to implement and configure ome similar tools for mcp based agent
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


pub async fn run_server() {
    let service = Server::new()
        .serve(stdio())
        .await
        .expect("Failed to run the service");

    service.waiting().await.expect("Service failed");
}
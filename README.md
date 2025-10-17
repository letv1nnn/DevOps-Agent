# DevOps Agent

Modern CI/CD pipelines are messy and often fail, requiring manual debugging, reruns, and quick fixes that slow 
developers down. I am building an AI-powered DevOps Agent that runs pipelines on its own by scanning project 
directories, figuring out build systems, and generating execution plans. The agent tracks workflows in GitHub 
Actions, adjusts to errors with retries and rollbacks, and checks commands to prevent unsafe execution. 

To study concurrency and test the orchestration logic, I built a small [AI chat](https://github.com/letv1nnn/actor-model-ai-chat) application with a TypeScript/HTML/CSS frontend and a Rust backend using the Actor model, which forwards requests to Ollama or just echoes them in 
development. Made for real-time, independent operation on servers with full execution logging and secure 
integration, the agent aims to provide more reliable pipelines, cut manual work by up to 70%, and improve 
modern software delivery. 

- [Installation]()
- [Setup]()

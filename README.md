# DevOps Agent

Modern delivery pipelines are increaseingly complex, invlolving tasks like environment provisioning, code linting, test orchestration, build artifact generation, and deployment rollouts. These tasks are often automated, but rarely orchestrated by intelligent agents that understand context, adjust plans, or recover from failure.

A DevOps Agent can fill this gap by triggering, monitoring and adapting CI/CD workflow based on plans and goals. The agent is able to invoke shell commands, interact with GitHub Actions and external CI tools, validate results and escalate issues based on configurable rules. It also has a full logging capability.

To study concurrency and test the orchestration logic, I built a small [AI chat](https://github.com/letv1nnn/actor-model-ai-chat) application with a TypeScript/HTML/CSS frontend and a Rust backend using the Actor model, which forwards requests to Ollama or just echoes them in 
development. Made for real-time, independent operation on servers with full execution logging and secure 
integration, the agent aims to provide more reliable pipelines, cut manual work by up to 70%, and improve 
modern software delivery. 

### Docs

- [Installation](https://github.com/letv1nnn/DevOps-Agent/blob/main/docs/Installation.md)
- [Setup](https://github.com/letv1nnn/DevOps-Agent/blob/main/docs/Setup.md)
- [Examples](https://github.com/letv1nnn/DevOps-Agent/blob/main/docs/Examples.md)

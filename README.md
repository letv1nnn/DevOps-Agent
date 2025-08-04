# DevOps Agent

## Description
***DevOps Agent with CI/CD Tasking*** (I thought it to be a MCP server with clients).

Modern delivery pipelines are increaseingly complex, invlolving tasks like environment provisioning, code linting, test orchestration, build artifact generation, and deployment rollouts. These tasks are often automated, but rarely orchestrated by sintelligent agents that understand context, adjust plans, or recover from failure. 

A ***DevOps Agent*** can fill this gap by triggering, monitoring and adapting CI/CD workflow based on plans and goals. The agent is able to invoke shell commands, interact with GitHub Actions or external CI tools, validate results and escalate issues based on configurable rules. It also has a full logging capability.

***Core Agent Responsibilities***

The DevOps agent's primary functions include:

- Executing predefined CI/CD stages (lint, test, build, deploy).
- Receiving tasks via command-line, REST, or LLM-generated plans.
- Running system-level tools securely (e.g., docker, cargo, gh).
- Logging all results and detecting failed or flaky tasks.
- Supporting rollback or retry for critical stages.


## Current State
I've build a basic template for the agent, that implements basic logging, task execution pipeline and test mcp server. It can execute a pipeline of tasks and log results into the agent.log file.

## Build and Clone
You primarily need to install [rust](https://www.rust-lang.org/tools/install) compiler. 
For Windows go to the provided official website and download it from there, for other operating systems like Linux, Mac or WSL use:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

***Clone***
```bash
git clone https://github.com/letv1nnn/DevOps-Agent.git
cd DevOps-Agent/
```

***Build***

NOTE: Depending on your operating system, extension is different, so for Windows leave *".exe"*, otherwise remove this extension.   
```bash
cargo build --release
./target/release/agent_core.exe -- --config set_of_tasks.json # your json config file
```

## Ideas for Extension
- GitHub/GitLab API integration.
- LLM-based interpretation of build logs or error summarization.


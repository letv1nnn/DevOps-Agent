# Installation

There are several ways of how to set up the agent

### Mandatory utils:
- ***Rust and Cargo***
    > NOTE: For Windows, follow instractions from the following website [Rust and Cargo](https://rust-lang.org/tools/install/).
    
    On Linux, macOS or WSL systems, this is done as follows:
    ```sh
    curl https://sh.rustup.rs -sSf | sh
    ```
- ***Ollama***
    > NOTE: For Windows, download it from the following website [Ollama](https://ollama.com/download/windows)

    if you are using Ollama llm, then you will need to install it locally. However, I'd recommend to use Ollama only if you have 16 or more gb of RAM. Also, depending on your RAM, you can choose more faster Ollama model.
    On Linux and macOS systems, this is done as follows:
    ```sh
    curl -fsSL https://ollama.com/install.sh | sh
    ```

### Environment Configuration

Primarily, you will need to set up the `.env` file.

It is mandatory to specify the github token, so the agent could interact with github. Basically, just follow the template below.

```sh
# --------------------------------------------- CONFIGURATION FOR LLM
# For all llms, you need to fill these:
# MODEL=""

# For openai, you need to fill these:
# OPENAI_API_KEY=""

# For ollama, you need to fill these


# --------------------------------------------- CONFIGURATION FOR GITHUB
# Personal Access Token with repo and workflow read permissions
# GITHUB_TOKEN=""
# Owner and Repo name to analize
# OWNER=""
# REPO=""

# --------------------------------------------- CONFIGURATION FOR AGENT
# predefined pipeline name in form of string, possible values: "list_workflows download_workflows_logs analize_agent_logs"
# PIPELINE=""
# agent run interval in hours unsigned int 64, default is set up to 2 hours
# TIMEOUT_HOUR=u64 
```

### Cargo

I would recommend to use cargo in case of testing the agent.
> NOTE: agent requires you to specify the mode flag --mode agent/interaction

```sh
git clone https://github.com/letv1nnn/DevOps-Agent && cd DevOps-Agent
cargo build --release
./target/release/agent --mode agent # .exe for Windows, and mode can be agent or interaction
```

### Make

I would recommend to use make in case of testing the agent. The easiest way to test the agent.
```sh
git clone https://github.com/letv1nnn/DevOps-Agent && cd DevOps-Agent
make # you'll see the available commands.
```

### Docker

[***Docker***](https://docs.docker.com/engine/install/)

```sh
docker compose up --build 
```

### Kubernetes

[***Kubernetes***](https://kubernetes.io/docs/tasks/tools/)
I didn't deploy this agent to constantly run on any external services. I was using it locally, using [minikube](https://minikube.sigs.k8s.io/docs/start/?arch=%2Fwindows%2Fx86-64%2Fstable%2F.exe+download). Also, I was using Docker to set up the cluster, subsequently, if you are using the same stack as me, run the following command.

```sh
# to start the minikube
minikube start --driver=docker
# apply the kubernetes deployment
kubectl apply -f deployment.yaml
```

To check information about pods condition, run the following.

```sh
kubectl get pods # In column 'READY', it should be 1/1
```

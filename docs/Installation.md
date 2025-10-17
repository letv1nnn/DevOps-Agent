# Installation

There are several ways of how to set up the agent

Mandatory utils:
- ***Rust and Cargo***
    > NOTE: For Windows, follow instractions from the following website [Rust and Cargo](https://rust-lang.org/tools/install/).
    
    On Linux and macOS systems, this is done as follows:
    ```sh
    curl https://sh.rustup.rs -sSf | sh
    ```


## Cargo

> NOTE: agent requires you to specify the mode flag --mode agent/interaction

```sh
git clone https://github.com/letv1nnn/DevOps-Agent && cd DevOps-Agent
cargo build --release
./target/release/agent --mode agent # .exe for Windows, and mode can be agent or interaction
```

## Docker
(not done yet)

[***Docker***](https://docs.docker.com/engine/install/)

```sh
echo "Running agent with docker container"
```

## Kubernetes
(not done yet)

[***Kubernetes***](https://kubernetes.io/docs/tasks/tools/)
I didn't deploy this agent to constantly run on any external services. I was using it locally, using [minikube](https://minikube.sigs.k8s.io/docs/start/?arch=%2Fwindows%2Fx86-64%2Fstable%2F.exe+download). Also, I was using Docker to set up the cluster, subsequently, if you are using the same stack as me, run the following command.

```sh
minikube start --driver=docker
kubectl apply -f deployment.yaml
```

To check information about pods condition, run the following.

```sh
kubectl get pods # In column 'READY', it should be 1/1
```
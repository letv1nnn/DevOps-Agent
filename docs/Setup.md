# Setup

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
# GITHUB_TOKEN=""
# OWNER=""
# REPO=""

# --------------------------------------------- CONFIGURATION FOR AGENT
# predefined pipeline name in form of string, possible values: "list_workflows download_workflows_logs analize_agent_logs"
# PIPELINE=""
# agent run interval in hours unsigned int 64, default is set up to 2 hours
# TIMEOUT_HOUR=u64 
```
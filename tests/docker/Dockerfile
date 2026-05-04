FROM ubuntu:24.04

# Install dependencies (including Node.js for Claude Code CLI)
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash curl git ripgrep ca-certificates nodejs npm pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create test user with home directory
RUN useradd -m -s /bin/bash testuser

# Switch to test user for Claude CLI installation
USER testuser
WORKDIR /home/testuser

# Install Claude Code CLI as testuser (installs to ~/.claude)
RUN curl -fsSL https://claude.ai/install.sh | bash || true

# Install Codex CLI for plugin loading checks
RUN mkdir -p /home/testuser/.local/bin
RUN npm config set prefix /home/testuser/.local \
    && npm install -g @openai/codex \
    && /home/testuser/.local/bin/codex --version

# Add Claude and Codex CLIs to PATH
ENV PATH="/home/testuser/.claude/local/bin:/home/testuser/.local/bin:${PATH}"

# Entry point for testing
ENTRYPOINT ["/bin/bash"]

FROM node:22-bookworm-slim

# Install minimal dependencies for Claude Code
RUN apt-get update && apt-get install -y --no-install-recommends \
    bash curl git ripgrep ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create test user with home directory
RUN useradd -m -s /bin/bash testuser

# Switch to test user for Claude CLI installation
USER testuser
WORKDIR /home/testuser

# Install Claude Code CLI as testuser (installs to ~/.claude)
RUN curl -fsSL https://claude.ai/install.sh | bash || true

# Ensure .local/bin exists for user
RUN mkdir -p /home/testuser/.local/bin

# Add Claude CLI to PATH
ENV PATH="/home/testuser/.claude/local/bin:/home/testuser/.local/bin:${PATH}"

# Entry point for testing
ENTRYPOINT ["/bin/bash"]

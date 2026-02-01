FROM node:22-alpine

# Install minimal dependencies for Claude Code (Alpine-specific)
RUN apk --no-cache add bash curl git libgcc libstdc++ ripgrep

# Create test user with home directory
RUN adduser -D -s /bin/bash testuser

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

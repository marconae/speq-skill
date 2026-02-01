FROM node:22-alpine

# Install minimal dependencies for Claude Code (Alpine-specific)
RUN apk --no-cache add bash curl git libgcc libstdc++ ripgrep

# Install Claude Code CLI via native installer
RUN curl -fsSL https://claude.ai/install.sh | bash

# Create test user with home directory
RUN adduser -D -s /bin/bash testuser
USER testuser
WORKDIR /home/testuser

# Ensure .local/bin exists for user
RUN mkdir -p /home/testuser/.local/bin

# Entry point for testing
ENTRYPOINT ["/bin/bash"]

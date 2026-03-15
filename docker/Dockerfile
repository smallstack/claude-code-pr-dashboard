FROM node:24-bookworm

# Install system dependencies
RUN apt-get update && apt-get install -y \
	git \
	curl \
	bubblewrap \
	socat \
	sudo \
	&& rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
	| dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
	&& echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
	| tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
	&& apt-get update \
	&& apt-get install -y gh \
	&& rm -rf /var/lib/apt/lists/*

# Install Claude Code CLI globally
RUN npm install -g @anthropic-ai/claude-code

# Create a non-root user
RUN useradd -m -s /bin/bash claude && echo "claude ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Copy entrypoint and bundled commands (before USER switch so they're owned by root)
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
COPY commands/ /usr/local/share/claude-commands/
RUN chmod +x /usr/local/bin/entrypoint.sh

USER claude
WORKDIR /home/claude

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

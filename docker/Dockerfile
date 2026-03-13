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

USER claude
WORKDIR /home/claude

# Copy entrypoint and bundled commands
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
COPY commands/ /usr/local/share/claude-commands/

ENTRYPOINT ["entrypoint.sh"]

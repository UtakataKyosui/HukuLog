FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu

ARG USERNAME=utakata
ARG USER_UID=1100
ARG USER_GID=$USER_UID

# 必要なツールを入れつつユーザーを作成
RUN apt-get update && apt-get install -y curl sudo \
  && groupadd --gid $USER_GID $USERNAME \
  && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
  && echo "$USERNAME ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# ユーザー作成やsudo許可は省略、要件に応じて前述のように追加
USER utakata
ENV HOME=/home/utakata
ENV MISE_BIN="$HOME/.local/share/mise/bin"
ENV PATH="$MISE_BIN:$PATH"

RUN curl https://mise.run | bash \
 && echo 'export PATH="$HOME/.local/share/mise/bin:$PATH"' >> $HOME/.bashrc \
 && echo 'eval "$(mise activate bash)"' >> $HOME/.bashrc

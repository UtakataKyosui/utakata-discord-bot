FROM rust:1.83.0-bullseye

ARG USER_NAME=utakata
ARG USER_ID=1000
ARG GROUP_ID=1000

# 必要なパッケージをインストール
RUN apt update -y \
    && apt upgrade -y \
    && apt install git curl zip unzip -y \
    && rm -rf /var/lib/apt/lists/*

# ユーザーとグループを作成
RUN groupadd -g ${GROUP_ID} ${USER_NAME} && \
    useradd -m -u ${USER_ID} -g ${USER_NAME} ${USER_NAME}   

# デフォルトシェルをbashに設定
RUN chsh -s /bin/bash ${USER_NAME}

# shuttleをインストール
RUN cargo install cargo-shuttle \
    && cargo install sea-orm-cli \
    && rustup component add rustfmt

# 権限を設定
USER root
RUN chown -R ${USER_NAME}:${USER_NAME} /usr/local/cargo \
    && chmod -R g+rw /usr/local/cargo \
    && chmod -R g+rw /usr/local/lib/

# 最後にユーザーを切り替え
USER ${USER_NAME}

# 作業ディレクトリを設定
WORKDIR /workspace
RUN mkdir -p /workspace && chown -R ${USER_NAME}:${USER_NAME} /workspace
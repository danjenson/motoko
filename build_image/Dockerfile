FROM cirrusci/flutter:latest
RUN apt update && apt install pkg-config \
  && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
  && chmod +x $HOME/.cargo/env \
  && $HOME/.cargo/env \
  && curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" \
  && unzip awscliv2.zip \
  && sudo ./aws/install
ENV PATH="$PATH:$HOME:/root/.cargo/bin"
RUN rustup install nightly && rustup default nightly

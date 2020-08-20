FROM cirrusci/flutter:latest
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y \
  && chmod +x $HOME/.cargo/env \
  && $HOME/.cargo/env

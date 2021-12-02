FROM rust:1.56.1

RUN rustup override set nightly
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features mysql

WORKDIR "/home/work/rust-rocket-web/"

CMD ["cargo", "watch", "-x", "run"]

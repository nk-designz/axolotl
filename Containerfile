FROM rustlang/rust:nightly as builder

RUN USER=root cargo init --bin axolotl
WORKDIR /axolotl
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY . .
RUN touch ./src/main.rs && cargo build --release


FROM debian:stretch
WORKDIR /usr/bin
COPY --from=builder /axolotl/target/release/axolotl .
EXPOSE 3000
ENTRYPOINT [ "axolotl" ]

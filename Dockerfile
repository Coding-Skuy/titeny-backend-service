FROM rust:1.83-slim
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/titeny-backend-service"]

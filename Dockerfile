# syntax=docker/dockerfile:1.7
FROM rust:1-alpine AS build
WORKDIR /app
RUN apk add --no-cache musl-dev pkgconfig openssl-dev
COPY Cargo.toml rust-toolchain.toml ./
COPY .cargo .cargo
COPY crates/common crates/common
COPY services/depolyer services/depolyer
RUN cargo build --release -p executor

FROM gcr.io/distroless/cc AS runtime
COPY --from=build /app/target/release/executor /executor
ENV AUTH_BIND=0.0.0.0:8080 AUTH_SERVICE_NAME=executor
EXPOSE 8080
ENTRYPOINT ["/executor"]
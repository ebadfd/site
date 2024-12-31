FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare

FROM chef AS builder

COPY --from=planner /app/recipe.json .
COPY --from=planner /app/lib lib
COPY --from=planner /app/Config.toml Config.toml
COPY --from=planner /app/blog blog
COPY --from=planner /app/static static
COPY --from=planner /app/well-known well-known

RUN cargo chef cook --release
COPY . .

RUN cargo build --release
RUN mv ./target/release/standalone ./app

FROM debian:stable-slim AS runtime

RUN apt-get update && apt install -y openssl ca-certificates
RUN update-ca-certificates

WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
COPY --from=builder /app/Config.toml /app/Config.toml
COPY --from=builder /app/blog /app/blog
COPY --from=builder /app/static static

ENTRYPOINT ["/usr/local/bin/app"]

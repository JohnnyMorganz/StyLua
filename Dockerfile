FROM rust:1.67 AS build
ARG CARGO_FLAGS="--all-features --release"

WORKDIR /app
COPY . ./
RUN cargo build ${CARGO_FLAGS}

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/stylua /
CMD ["./stylua"]

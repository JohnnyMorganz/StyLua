FROM rust:1.67 AS build

WORKDIR /app
COPY . ./
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build /app/target/release/stylua /
CMD ["./stylua"]

FROM rust AS BUILD
WORKDIR /app
COPY ./ /app
RUN cargo build --release
RUN strip target/release/cloud-run-rust

FROM gcr.io/distroless/cc
COPY --from=BUILD /app/target/release/cloud-run-rust /
CMD ["./cloud-run-rust"]
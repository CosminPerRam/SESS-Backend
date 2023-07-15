
##### Builder
FROM rust:1.70.0-slim as builder

WORKDIR /usr/src

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# Now copy in the rest of the sources
COPY . /usr/src/

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

##### Runtime
FROM alpine:3.18.0 AS runtime

CMD ["mkdir", "/usr/local/bin/sess"]

# Copy application binary from builder image
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/sess_backend /usr/local/bin/sess
COPY --from=builder /usr/src/privkey.pem /usr/local/bin/sess
COPY --from=builder /usr/src/fullchain.pem /usr/local/bin/sess

# Run the application
CMD ["/usr/local/bin/sess/sess_backend"]

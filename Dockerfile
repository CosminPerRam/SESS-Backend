
##### Builder
FROM rust:1.71-alpine as builder

WORKDIR /usr/src

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN apk update && apk upgrade
RUN apk add --no-cache musl-dev
RUN rustup target add x86_64-unknown-linux-musl

# Now copy in the rest of the sources
COPY . /usr/src/

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

##### Runtime
FROM alpine:3.18.0 AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/target/x86_64-unknown-linux-musl/release/sess_backend /usr/local/bin
COPY --from=builder /usr/src/privkey.pem /usr/local/bin
COPY --from=builder /usr/src/cert.pem /usr/local/bin

# Run the application
CMD ["/usr/local/bin/sess_backend"]

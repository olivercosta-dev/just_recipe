# Builder stage
FROM alpine:3.16 as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    build-base \
    curl \
    gcc \
    musl-dev \
    openssl-dev

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Dummy build to cache dependencies
RUN mkdir src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -f src/main.rs

# Copy the rest of your application source code
COPY src/ ./src/
COPY scripts/ ./scripts/

# Final build
RUN cargo build --release --bin just_recipe

# Runtime stage
FROM alpine:3.16

WORKDIR /app

# Install runtime dependencies if there are any
RUN apk add --no-cache libgcc

# Copy the built executable and any other necessary files
COPY --from=builder /app/target/release/just_recipe /app/

# Command to run
ENTRYPOINT ["./just_recipe"]

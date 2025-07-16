# ---- Build Stage ----
    FROM rust:latest as builder

    # Install dependencies needed to build actix-web and openssl
    RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential
    
    WORKDIR /app
    
    # Copy main Cargo files
    COPY Cargo.toml Cargo.lock ./
    
    # Copy only Cargo.toml files from local crates for dependency caching
    COPY jwt-lib/Cargo.toml jwt-lib/Cargo.toml
    COPY utils/Cargo.toml utils/Cargo.toml
    
    # Create dummy main.rs to allow building dependencies
    RUN mkdir src && echo "fn main() {}" > src/main.rs
    RUN cargo build --release || true
    RUN rm -r src
    
    # Copy full source code after caching deps
    COPY src ./src
    COPY jwt-lib ./jwt-lib
    COPY utils ./utils
    
    # Final release build
    RUN cargo build --release
    
    # ---- Runtime Stage ----
    FROM debian:bookworm-slim
    
    # Install only the necessary OpenSSL runtime libs
    RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
    
    WORKDIR /app
    
    # Copy compiled binary from builder
    COPY --from=builder /app/target/release/broadcaster .
    
    # Expose Actix port
    EXPOSE 9090
    
    # Launch app
    CMD ["./broadcaster"]
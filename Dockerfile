FROM rust:latest

# Install trunk and wasm target
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build for web
RUN trunk build --release --features web --no-default-features

# Expose port for serving
EXPOSE 8080

# Serve the built web app
CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080", "--features", "web", "--no-default-features"]

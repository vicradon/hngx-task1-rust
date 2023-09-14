FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Create an empty dummy project and build dependencies (this step is separated for caching)
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of your application source code
COPY . .

# Build the Actix web application
RUN cargo build --release

RUN cargo run --release
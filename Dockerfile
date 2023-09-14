FROM rust:latest as builder

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

# Use a smaller base image for the final release container
FROM debian:buster-slim

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/app /usr/local/bin/app

# Expose the port that your Actix web server listens on
EXPOSE 8080

# Start your Actix web server when the container starts
CMD ["/usr/local/bin/app"]

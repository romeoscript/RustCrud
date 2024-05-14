# Build stage 
FROM rust:1.69-buster as builder

WORKDIR /app

# accept the build argument
ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

# copy the source code
COPY . .

# build the application
RUN cargo build --release

# Run stage 
FROM debian:buster-slim

WORKDIR /usr/local/bin

# copy the built binary from the build stage

COPY --from=builder /app/target/release/RustCrud .

# set the startup command to run your binary
CMD ["./RustCrud"]
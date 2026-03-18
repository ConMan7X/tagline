# Build stage for frontend
FROM node:20-alpine AS frontend-build
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm ci
COPY frontend/ .
RUN npm run build

# Build stage for backend
FROM rust:1.85-alpine AS backend-build
RUN apk add --no-cache musl-dev openssl-dev pkgconf curl git
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY data/movies.json ./
RUN cargo build --release

# Final minimal runtime image
FROM alpine:3.19
RUN apk add --no-cache ca-certificates
WORKDIR /app

# Copy runtime binary from backend build
COPY --from=backend-build /app/target/release/tagline ./tagline

# Copy movies.json for backend API
RUN mkdir -p /app/data
COPY --from=backend-build /app/movies.json ./data/movies.json

# Copy compiled frontend into the directory served by main.rs
COPY --from=frontend-build /app/frontend/dist ./frontend/dist

# Expose configured port
EXPOSE 3000

# Run the backend server (serves frontend with ServeDir in main.rs)
CMD ["./tagline"]

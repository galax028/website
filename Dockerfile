# 1: Frontend
FROM node:current-alpine AS frontend
WORKDIR /app
COPY ./src/frontend/package.json ./src/frontend/package-lock.json .
RUN npm ci
COPY ./src/frontend .
ARG OUTDIR_PROD=/app/dist
RUN npm run build

# 2: Backend
FROM clux/muslrust:1.96.0-stable AS chef
RUN cargo install --locked cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
ARG SQLX_OFFLINE=true
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend
COPY --from=planner /app/recipe.json recipe.json
ARG SQLX_OFFLINE=true
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine AS runtime
WORKDIR /app
COPY --from=backend /app/target/x86_64-unknown-linux-musl/release/website /app
COPY --from=frontend /app/dist /app/dist
ENV HOST=0.0.0.0
ENV PORT=8080
ENV STATIC_ROOT=/app/dist
CMD ["/app/website"]

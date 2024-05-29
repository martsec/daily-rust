# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen pkgconfig openssl-dev


RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
RUN npm install -D tailwindcss 
RUN npm install @tailwindcss/forms
COPY . .

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app
RUN mkdir -p /app/data

COPY --from=builder /work/target/release/plaicards /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/


EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/plaicards"]


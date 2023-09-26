####################################################################################################
## Builder
####################################################################################################
FROM rustlang/rust:nightly AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu
RUN update-ca-certificates

# Create appuser
ENV USER=app
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./prismaviz-rust .

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'

RUN cargo build --package prismaviz-api --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/prismaviz-api ./

# Use an unprivileged user.
USER app:app

CMD ["/app/prismaviz-api"]
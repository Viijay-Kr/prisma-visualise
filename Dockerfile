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

###### Build Web front end

FROM node:latest as vitebuilder

WORKDIR /app

COPY ./prismaviz-web .
ENV VITE_PRISMA_API_URL=""
RUN npm instal
RUN npm run build

####################################################################################################
## Final image
####################################################################################################
FROM scratch
# Import from builder.
VOLUME [ "/tmp" ]
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/prismaviz-api ./
# COPY --from=builder /app/.env ./

COPY --from=vitebuilder /app/dist ./dist

# Use an unprivileged user.
# USER app:app

# RUN chmod=a=rwx -r files* /tmp
ENV ROCKET_ASSETS_DIR="./dist"
CMD ["/app/prismaviz-api"]
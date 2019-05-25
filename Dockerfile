FROM clux/muslrust AS builder

RUN mkdir -p /src
WORKDIR /src
COPY . /src

RUN cargo build --release
RUN strip target/x86_64-unknown-linux-musl/release/check
RUN cp target/x86_64-unknown-linux-musl/release/check check
RUN strip target/x86_64-unknown-linux-musl/release/in
RUN cp target/x86_64-unknown-linux-musl/release/in in
RUN strip target/x86_64-unknown-linux-musl/release/out
RUN cp target/x86_64-unknown-linux-musl/release/out out


FROM alpine:3.8 as resource
RUN apk update && apk add ca-certificates

COPY --from=builder /src/check /opt/resource/check
COPY --from=builder /src/in /opt/resource/in
COPY --from=builder /src/out /opt/resource/out

RUN chmod +x /opt/resource/*

FROM resource

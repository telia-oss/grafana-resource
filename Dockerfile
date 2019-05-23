FROM clux/muslrust AS builder

RUN mkdir -p /src
WORKDIR /src
COPY . /src

RUN cargo build --release
RUN strip target/x86_64-unknown-linux-musl/release/grafana-resource
RUN cp target/x86_64-unknown-linux-musl/release/grafana-resource main


FROM alpine:3.8 as resource
RUN apk update && apk add ca-certificates
COPY --from=builder /src/main  /opt/resource/main
RUN ln -s /opt/resource/main /opt/resource/check
RUN ln -s /opt/resource/main /opt/resource/in
RUN ln -s /opt/resource/main /opt/resource/out
RUN chmod +x /opt/resource/*

FROM resource

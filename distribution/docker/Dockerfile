# Builder Image
# hadolint ignore=DL3007
FROM alpine:latest as builder
ARG RELEASE_TAG
# hadolint ignore=DL3018
RUN apk --no-cache add curl
RUN mkdir /build
RUN echo curl --silent -L https://github.com/lukaspustina/flatcat/releases/download/${RELEASE_TAG}/fcat-linux-musl-x86_64 -o /build/fcat
RUN curl --silent -L https://github.com/lukaspustina/flatcat/releases/download/${RELEASE_TAG}/fcat-linux-musl-x86_64 -o /build/fcat
RUN chmod +x /build/fcat

# Final Image
# hadolint ignore=DL3007
FROM alpine:latest
COPY --from=builder /build/fcat /usr/local/bin/fcat
CMD ["echo", "USAGE: docker run -ti flatcat:<image tag> fcat <arguments and options>, e.g., docker run -ti flatcat:latest fcat Cargo.toml"]

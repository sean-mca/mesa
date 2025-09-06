FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev sqlite-static openssl-dev openssl-libs-static pkgconf git libpq-dev protobuf-dev

ENV SYSROOT=/dummy

ENV SQLITE3_STATIC=1 SQLITE3_LIB_DIR=/usr/lib/
ENV LIBPQ_STATIC=1

ENV PATH="/usr/local/bin:${PATH}"

WORKDIR /wd
COPY . /wd
COPY ../protos/ /wd
RUN cargo build --bins --release

FROM scratch
ARG version=unknown
ARG release=unreleased

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /wd/target/release/coordinator /

EXPOSE 3000
EXPOSE 50001
CMD ["./coordinator"]

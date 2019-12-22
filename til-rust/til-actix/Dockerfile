FROM rust:1-stretch AS builder

RUN curl http://www.musl-libc.org/releases/musl-1.1.24.tar.gz | tar -xvzf - && \
    cd musl-1.1.24 && \
    ./configure && make install

ENV PATH=/usr/local/musl/bin/:$PATH

COPY src /app/src
COPY Cargo.toml /app/

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build -v --target x86_64-unknown-linux-musl

FROM alpine

COPY --from=builder /app/target/x86_64-unknown-linux-musl/debug/til-actix /app/

CMD [ "/app/til-actix" ]

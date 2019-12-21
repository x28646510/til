FROM rustlang/rust:nightly-alpine AS builder

COPY src /app
COPY Cargo.toml /app/

WORKDIR /app

RUN cargo build -v

FROM alpine

COPY --from=builder /app/til-actix /app/

CMD [ "/app/til-actix" ]

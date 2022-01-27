####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

WORKDIR /venom_monkey

COPY ./ .

RUN cargo build -p server --release

####################################################################################################
## Final image
####################################################################################################
FROM debian:buster-slim

# Create unprivileged user
ENV USER=venom_monkey
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /venom_monkey

# Copy our build
COPY --from=builder /venom_monkey/target/release/server ./

# Use an unprivileged user
USER venom_monkey:venom_monkey

CMD ["/venom_monkey/server"]

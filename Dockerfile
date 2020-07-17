FROM rust as build

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
 
RUN cargo build
# --release

RUN mkdir -p /build-out

# RUN cp target/release/rust-gae-gcb /build-out/
RUN cp target/debug/rust-gae-gcb /build-out/

FROM ubuntu@sha256:5f4bdc3467537cbbe563e80db2c3ec95d548a9145d64453b06939c4592d67b6d

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/rust-gae-gcb /

EXPOSE 8080

CMD /rust-gae-gcb


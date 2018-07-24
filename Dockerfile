# We need to use the Rust build image, because
# we need the Rust compile and Cargo tooling
FROM rust:1.27.0 as build
RUN USER=root cargo new --bin rust-bookstore
WORKDIR /rust-bookstore
COPY ./Cargo.toml ./Cargo.toml

# build deps and remove fake source
RUN cargo build --release
RUN rm -rf ./src ./target
COPY ./src ./src
#build actual sources
RUN cargo build --release

FROM debian:jessie-slim
#Don't run as root
RUN groupadd -g 999 bookstore \
    && useradd -r -u 999 -g bookstore bookstore \
    && mkdir /rust-bookstore \
    && chown -R bookstore:bookstore /rust-bookstore
USER bookstore
WORKDIR /rust-bookstore

COPY --from=build /rust-bookstore/target/release/bookstore .

EXPOSE 8080
ENTRYPOINT [ "./bookstore" ]
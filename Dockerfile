FROM rust:1.27.0
EXPOSE 8080
WORKDIR /usr/src/bookstore
COPY . .
RUN cargo install --force --path .
CMD /usr/local/cargo/bin/bookstore
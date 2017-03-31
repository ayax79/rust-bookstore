FROM fnichol/rust:1.15.1
EXPOSE 8080
COPY . /bookstore/
WORKDIR /bookstore
RUN ["cargo", "clean"]
RUN ["cargo", "build"]
ENTRYPOINT ["cargo", "run"]

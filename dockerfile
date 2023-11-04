FROM rust:1.71.0
WORKDIR $HOME/webserver
COPY . .
RUN cargo build --release
CMD [ "target/release/webserver" ]
EXPOSE 7878
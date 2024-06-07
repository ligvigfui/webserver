FROM rust
WORKDIR $HOME/webserver
COPY . .

RUN apt-get update
RUN apt-get install -y --no-install-recommends dialog
RUN apt-get install -y --no-install-recommends openssh-server
RUN echo "root:Docker!" | chpasswd
RUN cargo build --release
RUN chmod u+x ./entrypoint.sh
COPY sshd_config /etc/ssh/

EXPOSE 7878 2222

ENTRYPOINT [ "./entrypoint.sh" ]
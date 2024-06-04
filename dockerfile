FROM rust:1.71.0
WORKDIR $HOME/webserver
COPY . .

# new script
COPY entrypoint.sh ./

# Start and enable SSH
RUN apt-get update \
    && apt-get install -y --no-install-recommends dialog \
    && apt-get install -y --no-install-recommends openssh-server \
    && echo "root:Docker!" | chpasswd \
    && chmod u+x ./entrypoint.sh
COPY sshd_config /etc/ssh/

EXPOSE 7878 2222

ENTRYPOINT [ "./entrypoint.sh" ]
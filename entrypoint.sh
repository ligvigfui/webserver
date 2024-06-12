#!/bin/sh
set -e
service ssh start
exec ./target/release/webserver
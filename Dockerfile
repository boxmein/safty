FROM ubuntu:latest
COPY target/release/safty /safty
ENTRYPOINT ["/safty"]

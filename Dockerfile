FROM ubuntu:25.10
FROM rust:1.93

RUN groupadd -r fm26 && useradd -m -r -g fm26 fm26

# Install dependencies
RUN apt update && apt upgrade -y
RUN apt install wget openjdk-21-jre-headless -y
RUN apt clean

USER fm26
ENV ARTIFACT_ROOT=/home/fm26
WORKDIR $ARTIFACT_ROOT

# RustyKeY dependencies (either fetched from GitHub or copied over)
# RUN mkdir rustc-wrapper && cd rustc-wrapper && \
#	wget "https://github.com/Drodt/KeY-rustc-wrapper/archive/refs/tags/0.1.0.tar.gz" && \
#	tar -xzf 0.1.0.tar.gz && rm 0.1.0.tar.gz && mv KeY-rustc-wrapper-0.1.0 wrapper && rm -rf wrapper/rml && \
#	wget "https://github.com/Drodt/rml/archive/refs/tags/v0.1.0.tar.gz" && \
#	tar -xzf v0.1.0.tar.gz && rm v0.1.0.tar.gz && mv rml-0.1.0 wrapper/rml
COPY --chown=fm26:fm26 rustc-wrapper rustc-wrapper
RUN cd rustc-wrapper/wrapper && cargo install --path crates/cargo-key

COPY rusty-key-0.1.0-exe.jar /home/fm26
COPY --chown=fm26:fm26 examples examples

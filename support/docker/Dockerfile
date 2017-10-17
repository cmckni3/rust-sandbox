FROM rust:1-stretch

LABEL maintainer="cmckni3@gmail.com"
LABEL Name=sandbox Version=0.0.1

ENV APP_DIR /src/app

ENV USER root

RUN mkdir -p ${APP_DIR}

WORKDIR ${APP_DIR}

COPY . ${APP_DIR}

RUN cargo build --release --quiet

EXPOSE 7878

COPY support/docker/entrypoint.sh /usr/local/bin/
CMD [ "entrypoint.sh" ]

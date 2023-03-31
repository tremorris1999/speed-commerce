FROM rust:1.67
WORKDIR /usr/src/speed-commerce-api
COPY . .
RUN cargo build
CMD ["/bin/bash", "./prod.sh"]

#TODO Find a way to optimize this
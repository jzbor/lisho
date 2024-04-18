FROM rust:latest as builder
WORKDIR /usr/src/lisho
COPY . .
RUN cargo install --path .

EXPOSE 8080
CMD ["lisho", "/mappings.txt", "0.0.0.0:8080"]



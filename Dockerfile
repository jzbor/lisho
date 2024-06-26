FROM rust:alpine
WORKDIR /usr/src/lisho
COPY . .
RUN cargo install --path .

EXPOSE 8080
CMD ["lisho", "/etc/lisho/mappings.txt", "0.0.0.0:8080"]



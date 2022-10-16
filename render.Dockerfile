FROM rust:slim

WORKDIR /opt/dyrocki/
COPY . .
RUN cargo build --release \
    && cp ./target/release/dyrocki . \
    && cargo clean
CMD ["./dyrocki", "server", "-s", "--withreverseproxy", "--host", "dyrochki-1k5s.onrender.com"]
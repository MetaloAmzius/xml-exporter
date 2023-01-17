FROM rust:1-buster
    MAINTAINER Ignas Lapėnas <ignas@lapenas.dev>

COPY Cargo.lock ./xml-exporter/
COPY Cargo.toml ./xml-exporter/
COPY src/ ./xml-exporter/src/
RUN cd xml-exporter && cargo build --release

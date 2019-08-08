ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

#RUN cargo install diesel_cli --no-default-features --features sqlite
#RUN sudo apt update && sudo apt install -y libsqlite3-dev zlibc zlib1g zlib1g-dev

# Add our source code.
ADD Cargo.toml .

# Create a dummy project for populating the cache
#RUN mkdir -p cache/src && echo "//AUTO GENERATED DUMMY FILE" > cache/src/lib.rs && cp Cargo.toml cache/src/
#RUN cd cache && cargo build --release

ADD diesel.toml .

ADD ./migrations/ ./migrations/
ADD ./src/ ./src/
ADD ./static/ ./static/

#RUN diesel migration run


# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build

# Now, we need to build our _real_ Docker container.
FROM alpine:latest
RUN apk --no-cache add ca-certificates sqlite-libs
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/CUBIC_Config \
    /usr/local/bin/

CMD /usr/local/bin/CUBIC_Config
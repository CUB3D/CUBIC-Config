FROM rust:latest

RUN apt-get update && apt-get install -y libmariadbclient-dev-compat zlibc zlib1g zlib1g-dev

RUN wget https://cdn.cub3d.pw/auth/public.pem && openssl rsa -pubin -inform PEM -in public.pem -outform DER -out public.der

# Add our source code.
ADD Cargo.toml .

ADD diesel.toml .

ADD ./migrations/ ./migrations/
ADD ./src/ ./src/
ADD ./static/ ./static/
ADD ./templates ./templates/

# Fix permissions on source code.
#RUN chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

COPY ./.env .

CMD cargo run --release

#CMD /home/rust/src/target/x86_64-unknown-linux-musl/release/CUBIC_Config

## Now, we need to build our _real_ Docker container.
#FROM debian:stable-slim
##RUN apk --no-cache add ca-certificates mariadb-connector-c-dev
#RUN apt-get update && apt-get install -y libmysqlclient-dev zlibc zlib1g zlib1g-dev
#
#COPY --from=builder \
#    /home/rust/src/target/x86_64-unknown-linux-musl/release/CUBIC_Config \
#    /usr/local/bin/
#
#CMD /usr/local/bin/CUBIC_Config

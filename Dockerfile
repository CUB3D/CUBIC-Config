FROM rust:latest

RUN apt-get update && apt-get upgrade && apt-get install -y libmariadbclient-dev-compat zlibc zlib1g zlib1g-dev

RUN wget https://cdn.cub3d.pw/auth/public.pem && openssl rsa -pubin -inform PEM -in public.pem -outform DER -out public.der

# Add our source code.
ADD Cargo.toml .

ADD diesel.toml .

ADD ./migrations/ ./migrations/
ADD ./src/ ./src/
ADD ./static/ ./static/
ADD ./templates ./templates/

# Build our application.
RUN cargo build --release

COPY ./.env .

CMD cargo run --release

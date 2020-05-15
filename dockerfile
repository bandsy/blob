# select build image
FROM rust:1.43.1-alpine as build

# create a new empty shell project
RUN USER=root cargo new --bin blob
WORKDIR /blob

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/blob*
RUN cargo build --release

# our final base
FROM rust:1.43.1-alpine

# copy the build artifact from the build stage
COPY --from=build /blob/target/release/blob .

# set the startup command to run your binary
CMD ["./blob"]

################################################################################
# Base image
################################################################################

FROM alpine as rust

# Install build tools
RUN apk add --update --no-cache rust cargo

ENV PATH=/root/.cargo/bin:$PATH

################################################################################
# Dependencies
################################################################################

FROM rust as dependencies

WORKDIR /build

# Create new fake project ($USER is needed by `cargo new`)
RUN USER=root cargo new safe-io

WORKDIR /build/safe-io

# Create the fake libraries
RUN USER=root cargo new safe-io-lib --lib
RUN USER=root cargo new safe-io-cli --bin

# Copy real app dependencies
COPY Cargo.* ./
COPY safe-io-lib/Cargo.* ./safe-io-lib/
COPY safe-io-cli/Cargo.* ./safe-io-cli/

# Build fake project with real dependencies
RUN cargo build --release

# Remove the fake app build artifacts
#
# NOTE If your application name contains `-` (`foo-bar` for example)
# then the correct command to remove build artifacts looks like:
#
# RUN rm -rf target/release/foo-bar target/release/deps/foo_bar-*
#                              ^                           ^
RUN rm -rf target/release/safe-io target/release/deps/safe_io-*
RUN rm -rf target/release/libsafe_io_lib* target/release/deps/libsafe_io_lib*

################################################################################
# Builder
################################################################################

FROM rust as builder

# We do not want to download deps, update registry, ... again
COPY --from=dependencies /root/.cargo /root/.cargo

WORKDIR /build/safe-io

# Copy everything, not just source code
COPY . .

# Update already built deps from dependencies image
COPY --from=dependencies /build/safe-io/target target

# Run tests
# TODO: I'm not sure this is using the dependencies already built in the
# previous stage
RUN cargo test

# Build real app
RUN cargo build --release

################################################################################
# Final image
################################################################################

FROM scratch

# Copy binary from builder image
COPY --from=builder /build/safe-io/target/release/safe-io .

ENTRYPOINT ["/safe-io"]

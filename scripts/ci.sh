#!/usr/bin/env bash
set -e

export RUST_BACKTRACE=1
cargo fmt --verbose --all -- --check
cargo clippy -- -D clippy::all -D warnings \
    -A renamed-and-removed-lints #this option is workaround (see https://issues.apache.org/jira/browse/THRIFT-4764)
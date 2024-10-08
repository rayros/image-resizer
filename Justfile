server:
  cargo run --features cli -- server

command-server:
  cargo run --features cli -- command-server

test:
  cargo test --all-features

docker-checks:
  docker build -t respicta . --target checks

coverage: coverage-build coverage-lcov coverage-html

coverage-build:
  mkdir -p coverage
  rm -rf target_coverage
  CARGO_TARGET_DIR="target_coverage" CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target_coverage/coverage/%p-%m.profraw' cargo test --all-features

coverage-lcov:
  grcov target_coverage/coverage \
  --llvm \
  --binary-path ./target_coverage/debug/deps/ \
  -s . \
  --branch \
  --ignore-not-existing \
  --excl-start '^(pub(\((crate|super)\))? )?mod tests' \
  --excl-stop '^}' \
  --ignore="target_coverage/*" \
  --ignore="*/tests/*" \
  --ignore="src/main.rs" \
  -t lcov \
  -o coverage/lcov.info

coverage-html:
  rm -rf coverage/html
  grcov target_coverage/coverage \
  --llvm \
  --binary-path ./target_coverage/debug/deps/ \
  -s . \
  --branch \
  --ignore-not-existing \
  --excl-start '^(pub(\((crate|super)\))? )?mod tests' \
  --excl-stop '^}' \
  --ignore="target_coverage/*" \
  --ignore="*/tests/*" \
  --ignore="src/main.rs" \
  -t html \
  -o coverage/html

docker-test:
  docker build -t respicta . --target test

memprofile:
  RUSTFLAGS="-g" cargo build --release \
  && heaptrack ./target/release/respicta server 
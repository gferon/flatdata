#!/usr/bin/env bash
#
# Usage:
#     ./coverage
#
# Run kcov on the tests, and merge the results.
#
# Environment variables:
# TRAVIS_JOB_ID - id for coveralls, defaults to none
# KCOV - path to kcov, defaults to 'kcov'

[ -n "$TRAVIS_JOB_ID" ] && COVERALLS_ID="--coveralls-id=$TRAVIS_JOB_ID"
[ -z "$KCOV" ] && KCOV=kcov

# Rebuild tests with dead code included, and get a list of the filenames.
export RUSTFLAGS="-C link-dead-code"
TEST_FILES=$(cargo test 2>&1 >/dev/null | awk '/^     Running target\/debug\// { print $2 }')

KCOV_OPTS="--verify --exclude-pattern=/.cargo --include-path $(pwd)"
OUT_DIR=target/kcov

for f in $TEST_FILES; do
    "$KCOV" $KCOV_OPTS "$OUT_DIR" $f
done
"$KCOV" --merge $KCOV_OPTS $COVERALLS_ID "$OUT_DIR" "$OUT_DIR"

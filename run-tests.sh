#!/bin/bash

echo "Running tests"

cargo test -- --test-threads=1

echo "Testing complete"
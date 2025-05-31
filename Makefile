#!/bin/sh

bin ?= main

all: debug

debug:
	nix-shell --run "cargo run --bin $(bin)"

run-release:
	nix-shell --run "cargo run --release --bin $(bin)"

realse:
	nix-shell --run "cargo build --release --bin main"

git-action-test:
	cargo run --bin git-action-test

clean:
	cargo clean


#!/bin/sh

bin ?= main

all: debug

debug:
	nix-shell --run "cargo run --bin $(bin)"

run-release:
	nix-shell --run "cargo run --release --bin $(bin)"

realse:
	nix-shell --run "cargo build --release --bin main"

clean:
	cargo clean


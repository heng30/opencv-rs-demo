#!/bin/sh

bin ?= main

all: debug

debug:
	cargo run --bin $(bin)

debug-nix:
	nix-shell --run "cargo run --bin $(bin)"

run-release:
	cargo run --release --bin $(bin)

realse:
	cargo build --release --bin $(bin)

git-action-test:
	cargo run --bin git-action-test

nix-develop:
	nix develop

clean:
	cargo clean

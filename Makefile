#!/bin/sh

bin ?= main

all: bins

bins:
	cargo build --bins

debug:
	cargo run --bin $(bin)

debug-nix:
	nix-shell --run "cargo run --bin $(bin)"

realse:
	cargo build --release --bin $(bin)

nix-shell:
	nix-shell

clean:
	cargo clean

# Example from
# https://tech.davis-hansson.com/p/make/
# Also consider reference at: http://www.gnu.org/software/make/manual/

## Initial setup
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eux -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

## ------------------------- Main part of the build file
MODE ?=

# Default - top level rule is what gets ran when you run just `make`
build:
> cargo build $(MODE)
> cargo build -p tsp $(MODE)
> wasm-pack build --target nodejs
> #npx tsc --noEmit
.PHONY: build

rust:
> cargo run -p tsp $(MODE)
.PHONY: rust

wasm:
> npx ts-node -T ts
.PHONY: wasm

ts:
> npx ts-node -T ts/tsp.ts
.PHONY: ts


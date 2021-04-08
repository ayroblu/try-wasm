# Example from
# https://tech.davis-hansson.com/p/make/
# Also consider reference at: http://www.gnu.org/software/make/manual/

## Initial setup
SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

ifeq ($(origin .RECIPEPREFIX), undefined)
  $(error This Make does not support .RECIPEPREFIX. Please use GNU Make 4.0 or later)
endif
.RECIPEPREFIX = >

## ------------------------- Main part of the build file

# Default - top level rule is what gets ran when you run just `make`
build:
> yarn asc assembly-script/assembly/index.ts --target debug
> rm -rf dist
> cp -a public dist
> mkdir dist/js
> cp -a build/untouched/* dist/js/
.PHONY: build


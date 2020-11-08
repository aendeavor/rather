SHELL := /bin/bash
.SHELLFLAGS := -eu -o pipefail -c

.DELETE_ON_ERROR:

MAKEFLAGS += --warn-undefined-variables
MAKEFLAGS += --no-builtin-rules

.PHONY: all run

CC := gcc
FILE := main

all: run

build:
	@ $(CC) -Wall -pedantic -O3 -o $(FILE) $(FILE).c debug.c

run: build
	@ sudo ./$(FILE)


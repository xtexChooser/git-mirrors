SHELL = bash
.SHELLFLAGS += -e
.ONESHELL:

MAKE_JOBSERVER_FLAGS = -j4
MAKE_FLAGS = --silent --no-builtin-rules --output-sync=target

TOUCH ?= touch
MKDIR ?= mkdir
PRINTF ?= printf

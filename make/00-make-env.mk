SHELL = bash
.SHELLFLAGS += -e
.ONESHELL:
DROP_STDOUT := >/dev/null
DROP_STDOUT_ERR := &>/dev/null

MAKE_JOBSERVER_FLAGS = -j4
MAKE_FLAGS = --silent --no-builtin-rules --output-sync=target

TOUCH ?= touch
MKDIR ?= mkdir
PRINTF ?= printf
RM ?= rm
CHOWN ?= chown
CHMOD ?= chmod
MV ?= mv
CP ?= cp
SED ?= sed

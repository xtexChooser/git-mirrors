.DEFAULT_GOAL := default
default: build test

.PHONY: force
force: ;

comma:= ,
empty:=
space:= $(empty) $(empty)

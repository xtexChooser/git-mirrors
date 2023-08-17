NAME		= xtex-os
VERSION		= 1.0

OUT			?= out
out			:= ${OUT}
SRC			?= .
src			:= ${SRC}
VPATH		+= $(src)
export NAME VERSION OUT SRC

include scripts/Makefile

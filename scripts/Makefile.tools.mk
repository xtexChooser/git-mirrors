.PHONY: fmt

fmt:
	$(call action,"FMT  ")
	clang-format -i $$(find arch/ core/ -type f \( -iname \*.c -o -iname \*.h -o -iname \*.cpp -o -iname \*.hpp \))

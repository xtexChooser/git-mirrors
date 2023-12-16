.PHONY: stable fmt

stable:
	git merge --stat --no-edi --into-name stable main
	git push origin main stable

fmt:
	prettier --write '.'

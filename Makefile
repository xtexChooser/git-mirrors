.PHONY: stable fmt

stable:
	git switch stable
	git merge --stat --no-edit main
	git switch main
	git push origin main stable

fmt:
	prettier --write '.'

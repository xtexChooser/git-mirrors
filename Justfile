alias r := review

review *ARGS:
	@scripts/review {{ARGS}}

install-hooks:
	rm -rf .git/hooks
	ln -sf ../scripts/hooks .git/hooks

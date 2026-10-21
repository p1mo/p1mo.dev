IGNORE = --ignore temp/** --ignore target/**
EXTS = -e rs,hbs,html,css,ts,js


$(eval ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS)))


.PHONY: wsl
wsl:
	wsl -e bash -c "cargo packager --release"





.PHONY: client
client:
	cd app && bun run build && cd ..



.PHONY: system
system:
	cargo run -p p1mo_dev



.PHONY: execute
execute: client system



.PHONY: dev
dev:
	watchexec $(IGNORE) $(EXTS) -r make execute $(ARGS)

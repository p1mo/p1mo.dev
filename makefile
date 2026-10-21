IGNORE 	= --ignore temp/** --ignore target/**
EXTS 	= -e rs,hbs,html,css,ts,js



$(eval ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS)))



.PHONY: client-dev
client-dev:
	cd app && bun run build && cd ..

.PHONY: system-dev
system-dev:
	cargo run -p p1mo_dev --bin p1mo_dev

.PHONY: dev
dev: client-dev system-dev



.PHONY: watch
watch:
	watchexec $(IGNORE) $(EXTS) -r make dev $(ARGS)



.PHONY: client-release
client-release:
	cd app && bun install && bun run build && cd ..

.PHONY: build-release
build-release:
	cargo build -p p1mo_dev --bin p1mo_dev --release

.PHONY: release
release: client-release build-release

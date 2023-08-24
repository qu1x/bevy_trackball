EXAMPLES := exponential_map gliding_clamp constellation_clamp egui

.PHONY: help
help:
	@printf "make install  Installs required tools.\n"
	@printf "make wasm -j  Builds optimized WASM examples.\n"
	@printf "make serve    Runs local HTTP server.\n"
	@printf "make clean    Removes ./target directory.\n"

.PHONY: install
install:
	command -v apt && sudo apt install binaryen || true
	command -v dnf && sudo dnf install binaryen || true
	command -v yum && sudo yum install binaryen || true
	command -v pacman && sudo pacman -S --needed binaryen || true
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli basic-http-server

.PHONY: wasm
wasm: $(EXAMPLES:%=target/%.wasm)

target:
	mkdir -p target
	ln -sf $(word 1,$(EXAMPLES)).html target/index.html

.PHONY: build
build: | target
	@for example in $(EXAMPLES); do \
		printf "    \033[1;32mChecking\033[0m $$example(example)\n"; \
		features=$$(cargo metadata --no-deps --format-version 1 | \
			jq --raw-output ".packages[] | select(.name==\"bevy_trackball\") | .targets[] | \
				select(.name==\"$$example\") | .\"required-features\" | join(\",\")"); \
		cargo build --profile wasm-release --target wasm32-unknown-unknown \
			--features $${features:-,} --example $$example; \
		jq -Rr @html ../../examples/$$example.rs > target/$$example.code; \
		sed -e 's/{EXAMPLE}/'$$example'/g' -e '/{SOURCE}/{r target/'$$example.code -e 'd}' \
			example.html > target/$$example.html; \
	done

../../target/wasm32-unknown-unknown/wasm-release/examples/%.wasm: build
	@true

target/%_bg.wasm: ../../target/wasm32-unknown-unknown/wasm-release/examples/%.wasm
	@printf "     \033[1;32mBinding\033[0m $*(example)\n"; \
	wasm-bindgen --target web --out-dir target --out-name $* $<

target/%.wasm: target/%_bg.wasm
	@printf "  \033[1;32mOptimizing\033[0m $*(example)\n"; \
	mv $< $@
	wasm-opt -Oz --output $< $@
	touch $@

.PHONY: serve
serve: wasm
	@printf "     \033[1;32mServing\033[0m examples\n"; \
	basic-http-server target

.PHONY: clean
clean:
	rm -rf -- target

.SECONDARY:
Makefile:;

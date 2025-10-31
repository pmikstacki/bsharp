set shell := ["bash", "-cu"]

setup:
    rustup target add wasm32-unknown-unknown
    cargo install wasm-pack
    cargo install --version 0.4.36 mdbook
    cargo install mdbook-mermaid --locked
    cargo install mdbook-linkcheck --locked
    cargo install mdbook-admonish --locked
    cargo install mdbook-pagetoc --locked
    cargo install mdbook-gitinfo --locked
    bun install --cwd site

docs:
    mdbook build docs -d site/public/docs/book

docs-admonish-install:
    mdbook-admonish install .

wasm-build:
    bun run --cwd site wasm:build

astro-build:
    bun run --cwd site build

site-build:
    just docs
    just wasm-build
    just astro-build

dev:
    just site-build
    bun run --cwd site dev

preview:
    bun run --cwd site preview

clean:
    rm -rf site/public/docs site/public/wasm site/dist

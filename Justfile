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
    mdbook build docs -d ../site/public/docs

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

# Run the same steps and checks as the GitHub Actions workflow locally
ci-local:
	set -euxo pipefail
	echo "[ci-local] Cleaning previous dist..."
	rm -rf site/dist || true
	echo "[ci-local] Building docs to site/public/docs..."
	just docs
	echo "[ci-local] Building site (wasm + astro)..."
	bun run --cwd site build
	echo "[ci-local] Verifying docs index exists in build output..."
	[ -f site/dist/docs/html/index.html ] || (echo "Missing site/dist/docs/html/index.html after build" >&2; exit 1)
	echo "[ci-local] Checking that no dev-only /src/scripts paths leaked..."
	! grep -R "/src/scripts/" -n -- "site/dist" || (echo "Found dev-only /src/scripts/ paths in build" >&2; exit 1)
	echo "[ci-local] Checking that no .ts module URLs remain in final HTML..."
	! grep -R 'type="module".*\.ts' -n -- "site/dist" || (echo "Found .ts module URLs in final HTML" >&2; exit 1)
	echo "[ci-local] CI-local checks passed."

$ErrorActionPreference = "Stop"

wasm-pack build --target web --release --out-dir pkg

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

python -m http.server 8000

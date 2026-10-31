$ErrorActionPreference = "Stop"

wasm-pack build --target web --release --out-dir pkg

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Start-Process "http://localhost:8000"

python -m http.server 8000

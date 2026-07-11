#!/usr/bin/env python3
from http.server import HTTPServer, SimpleHTTPRequestHandler, test
from pathlib import Path
import sys

PKG_DIR = Path(__file__).resolve().parent / 'pkg'

class RequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(PKG_DIR), **kwargs)

    def end_headers(self):
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        SimpleHTTPRequestHandler.end_headers(self)

if __name__ == '__main__':
    port=int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    print(f"Serving {PKG_DIR} at http://localhost:{port}/")
    test(RequestHandler, HTTPServer, port=port, bind='127.0.0.1')

#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import http.server
from http.server import HTTPServer, BaseHTTPRequestHandler
import socketserver

PORT = 8080

class CustomHttpRequestHandler(http.server.SimpleHTTPRequestHandler):
	def do_GET(self):
		if self.path == '/':
			self.path = 'index_dist.html'
		return http.server.SimpleHTTPRequestHandler.do_GET(self)

handler = CustomHttpRequestHandler

handler.extensions_map = {
	'.html': 'text/html',
	'.css': 'text/css',
	'.js': 'application/javascript',
	'.svg': 'image/svg+xml',
	'.wasm': 'application/wasm',
	'': 'application/octet-stream', # Default
}

socketserver.TCPServer.allow_reuse_address = True
httpd = socketserver.TCPServer(("", PORT), handler)

print("serving at port", PORT)
httpd.serve_forever()

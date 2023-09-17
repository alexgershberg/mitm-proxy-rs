#!/bin/sh

openssl req -x509 -newkey rsa:4096 -keyout mitm-proxy-rs.key -out mitm-proxy-rs.cer -sha256 -days 128 -nodes

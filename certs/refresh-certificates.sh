#!/bin/sh


# took this one from: https://github.com/rustls/hyper-rustls/blob/main/examples/refresh-certificates.sh
# I really need to understand how to make my own certificates...

set -xe

openssl req -nodes \
          -x509 \
          -days 100 \
          -newkey rsa:4096 \
          -keyout ca.key \
          -out ca.cert \
          -sha256 \
          -batch \
          -subj "/CN=ponytown RSA CA"

openssl req -nodes \
          -newkey rsa:3072 \
          -keyout inter.key \
          -out inter.req \
          -sha256 \
          -batch \
          -subj "/CN=ponytown RSA level 2 intermediate"

openssl req -nodes \
          -newkey rsa:2048 \
          -keyout end.key \
          -out end.req \
          -sha256 \
          -batch \
          -subj "/CN=testserver.com"

openssl rsa \
          -in end.key \
          -out sample.rsa

openssl x509 -req \
            -in inter.req \
            -out inter.cert \
            -CA ca.cert \
            -CAkey ca.key \
            -sha256 \
            -days 100 \
            -set_serial 123 \
            -extensions v3_inter -extfile openssl.cnf

openssl x509 -req \
            -in end.req \
            -out end.cert \
            -CA inter.cert \
            -CAkey inter.key \
            -sha256 \
            -days 100 \
            -set_serial 456 \
            -extensions v3_end -extfile openssl.cnf

cat end.cert inter.cert ca.cert > sample.pem
rm *.key *.cert *.req

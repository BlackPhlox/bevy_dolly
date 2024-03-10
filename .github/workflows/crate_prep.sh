#!/bin/sh

cp -r dolly dolly_crate
rm dolly_crate/.git
rm -rf dolly
mv dolly_crate dolly

read -p "Press any key to resume ..."

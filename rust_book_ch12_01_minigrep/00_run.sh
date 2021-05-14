#!/usr/bin/env bash

echo "case sensitive run..."
echo "--------------------------------------"
cargo run to poem.txt


echo "case insensitive run..."
echo "--------------------------------------"
CASE_INSENSITIVE=1 cargo run to poem.txt

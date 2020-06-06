#!/bin/bash
set -e
cargo run > demo.ppm
convert demo.ppm demo.png
feh demo.png

#!/bin/sh

docker run -i -v ${PWD}:/host rust:1.58 sh << COMMANDS                                     
cd /host; cargo build --release
COMMANDS


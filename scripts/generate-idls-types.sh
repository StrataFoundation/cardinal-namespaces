#!/usr/bin/env bash

# This script generates the IDL JSONs without buildling the full packages.

rm -rf src/idl/
mkdir -p src/idl/

for PROGRAM in $(find programs/ -maxdepth 3 -name lib.rs); do
    PROGRAM_NAME=$(dirname $PROGRAM | xargs dirname | xargs basename | tr '-' '_')
    echo "Parsing IDL for $PROGRAM_NAME"
    anchor-20 idl parse --file $PROGRAM --out-ts src/idl/$PROGRAM_NAME.ts || {
        echo "Could not parse IDL"
        exit 1
    }
done

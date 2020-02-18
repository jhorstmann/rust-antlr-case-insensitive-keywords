#!/bin/sh

java -jar antlr4-4.8-2-SNAPSHOT-complete.jar -Dlanguage=Rust -no-visitor -no-listener -o src/gen Query.g4
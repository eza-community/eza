#!/bin/bash

# Generate test data for the program

if [ $# -ne 2 ]; then
    echo "Usage: $0 <test name> <test argument>"
    exit 1
fi

# Clean up previous test data

if [ -f tests/cmd/"$1".toml ]; then
    rm tests/cmd/"$1".toml
fi

if [ -f tests/cmd/"$1".stdout ]; then
    rm tests/cmd/"$1".stdout
fi

if [ -f tests/cmd/"$1".stderr ]; then
    rm tests/cmd/"$1".stderr
fi

# Generate test data

touch tests/cmd/"$1".toml

echo 'bin.name = "eza"' >> tests/cmd/"$1".toml
echo 'args = "'"$2"'"' >> tests/cmd/"$1".toml

# Generate expected output

if [ -f target/debug/eza ]; then
    target/debug/eza "$2" > tests/cmd/"$1".stdout 2> tests/cmd/"$1".stderr
    returncode=$?
    if [ $returncode -ne 0 ]; then
        echo -e 'status.code = '$returncode'' >> tests/cmd/"$1".toml
        exit 0
    fi
else
    echo "Please build the program first"
    exit 1
fi

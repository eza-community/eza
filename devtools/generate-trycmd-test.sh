#!/usr/bin/env bash

# Generate test data for the program

if [ $# -le 1 ]; then
    echo "Usage: $0 <test name> <test argument>"
    exit 1
fi

test_name=$1
shift

# Clean up previous test data

if [ -f tests/cmd/"$test_name".toml ]; then
    rm tests/cmd/"$test_name".toml
fi

if [ -f tests/cmd/"$test_name".stdout ]; then
    rm tests/cmd/"$test_name".stdout
fi

if [ -f tests/cmd/"$test_name".stderr ]; then
    rm tests/cmd/"$test_name".stderr
fi

# Generate test data

touch tests/cmd/"$test_name".toml

echo 'bin.name = "eza"' >> tests/cmd/"$test_name".toml
echo 'args = "'"$*"'"' >> tests/cmd/"$test_name".toml

# Generate expected output

if [ -f target/debug/eza ]; then
    target/debug/eza "$@" > tests/cmd/"$test_name".stdout 2> tests/cmd/"$test_name".stderr
    returncode=$?
    if [ $returncode -ne 0 ]; then
        echo -e 'status.code = '$returncode'' >> tests/cmd/"$test_name".toml
        exit 0
    fi
else
    echo "Please build the program first"
    exit 1
fi

#!/usr/bin/env python

# Helper to display stdout / stderr in the terminal
# so that colours are rendered properly

import tomllib
import sys



if len(sys.argv) < 2:
    print(f'usage: {sys.argv[0]} [FILE]')
    exit(1)

with open(sys.argv[1], 'rb') as f:
    data = tomllib.load(f)

    if data['stdout']:
        print(f'STDOUT:\n{data['stdout']}')
    else:
        print('STDOUT: (empty)')

    if data['stderr']:
        print(f'STDERR:\n{data['stderr']}')
    else:
        print('STDERR: (empty)')

#!/usr/bin/env fish

set TEST_DIR tests
set TAPES $TEST_DIR/tapes
set REFERENCES $TEST_DIR/references
set TEMP $TEST_DIR/tmp

set EZA_GREEN 0D0
set EZA_RED D00
set EZA_YELLOW DD0

function main

    # Fixes command line output
    echo ""

    # TODO: automatic reference deletion
    for file in $TAPES/*

        set filename (basename $file .tape)

        command fish $TEST_DIR/vhs-util.fish $filename &

    end

    wait 

end

main

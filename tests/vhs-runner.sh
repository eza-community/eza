#!/usr/bin/env fish

set TEST_DIR tests
set TAPES "$TEST_DIR/tapes"
set REFERENCES "$TEST_DIR\/references"
set TEMP "$TEST_DIR\/tmp"

function run_test -d "Run VHS tests" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS "[+] $NAME: Success"
    set FAILURE "[+] $NAME: Failure"

    echo "[*] Testing $NAME..."

    cat $TAPES/$NAME_TAPE | sed "s/outfile/$TEMP\/$NAME.txt/" | sed s/-l// | vhs
    # diff -q validated.ascii validated.txt && echo $SUCCESS || echo $FAILURE
    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

run_test main

#!/usr/bin/env fish

set TEST_DIR tests
set TAPES $TEST_DIR/tapes
set REFERENCES $TEST_DIR/references
set TEMP $TEST_DIR/tmp

function run_test -d "Run VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS "[+] $NAME: Success"
    set FAILURE "[+] $NAME: Failure"

    echo "[*] Testing $NAME..."

    set GEN_DIR $TEMP
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs >/dev/null
    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

function gen_test -d "Generate VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS "[+] $NAME: Success"
    set FAILURE "[+] $NAME: Failure"

    echo "[*] Generating $NAME..."

    set GEN_DIR $REFERENCES
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs >/dev/null
    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

function main

    # gen_test main

    run_test main

end

main

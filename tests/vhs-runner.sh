#!/usr/bin/env fish

set TEST_DIR tests
set TAPES $TEST_DIR/tapes
set REFERENCES $TEST_DIR/references
set TEMP $TEST_DIR/tmp

function run_test -d "Run VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS "[+] $NAME: Success"
    set FAILURE "[+] $NAME: Failure"

    set GEN_DIR $TEMP
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    echo "[*] Testing $NAME..."

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs >/dev/null

    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

function gen_test -d "Generate VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS "[+] $NAME: Success"
    set FAILURE "[+] $NAME: Failure"

    set GEN_DIR $REFERENCES
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    # The idea behind this is that it makes it easier for users of this system
    # to change the reference. They should now only have to delete the old
    # reference, and a new one will be generated.
    if builtin test -f $GEN_FILE
        echo "[*] $GEN_FILE exists, skipping generating it"
        return
    end

    echo "[*] Generating $NAME..."

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs >/dev/null

    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

function main
    # TODO: automatic reference deletion
    for file in $TAPES/*

        set filename (basename $file .tape)

        gen_test $filename
        run_test $filename
    end
end

main

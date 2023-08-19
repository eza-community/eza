#!/usr/bin/env fish

set TEST_DIR tests
set TAPES $TEST_DIR/tapes
set REFERENCES $TEST_DIR/references
set TEMP $TEST_DIR/tmp

function print_msg -a ARG -a OP -a NAME -a MSG
    set_color reset
    echo -n "[$OP] "
    set_color reset
    set_color -b $ARG
    set_color 000
    echo -n "[ $NAME ]:"
    set_color reset
    echo " $MSG"
    set_color reset
end

function run_test -d "Run VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS (print_msg "0D0" "+" "$NAME" "Success")
    set FAILURE (print_msg "D00" "-" "$NAME" "Failure")

    set GEN_DIR $TEMP
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    echo "[*] $NAME: Testing..."
    print_msg DD0 "*" $NAME "Testing..."

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs &>/dev/null

    cmp -s -- $REFERENCES/$NAME.txt $TEMP/$NAME.txt && echo $SUCCESS || echo $FAILURE
end

function gen_test -d "Generate VHS test" -a NAME

    set NAME_TAPE "$NAME.tape"

    set SUCCESS (set_color yellow; echo -n "[+] $NAME:"; set_color reset; echo " Success")
    set FAILURE "[+] $NAME: Failure"

    set GEN_DIR $REFERENCES
    set GEN_FILE $GEN_DIR/$NAME.txt
    set GEN_FILE_ESCAPE (echo $GEN_FILE | sed "s/\//\\\\\//g")

    # The idea behind this is that it makes it easier for users of this system
    # to change the reference. They should now only have to delete the old
    # reference, and a new one will be generated.
    if builtin test -f $GEN_FILE
        echo "[+] $NAME: $GEN_FILE exists, skipping gen"
        return
    end

    echo "[*] $NAME: Testing..."

    cat $TAPES/$NAME_TAPE | sed s/outfile/$GEN_FILE_ESCAPE/ | sed s/-l// | vhs &>/dev/null

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

exit_status=0
if [ "$EZA_TESTS_IN_DOCKER" = "1" ]; then
    uid=$(stat -c '%u' tests/)
    gid=$(stat -c '%g' tests/)

    # chown files to docker’s user so that trycmd can write the test assertions
    sudo chown -R $(whoami): tests/cmd

    cargo --locked test -- --test cli_tests --test-threads 1
    if [ "$?" != "0" ]; then exit_status=1; fi
    cargo --locked test --no-default-features -- --test no_git --test-threads 1
    if [ "$?" != "0" ]; then exit_status=1; fi

    # then chown that back to the host user
    sudo chown -R $uid:$gid tests/cmd
else
    cargo --locked test -- --test cli_tests --skip cli_tests_linux --test-threads 1
    if [ "$?" != "0" ]; then exit_status=1; fi
    cargo --locked test --no-default-features -- --test no_git --skip cli_tests_linux --test-threads 1
    if [ "$?" != "0" ]; then exit_status=1; fi
fi

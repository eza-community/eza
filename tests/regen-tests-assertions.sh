uid=$(stat -c '%u' tests/cmd)
gid=$(stat -c '%g' tests/cmd)

export TRYCMD=overwrite

sudo chown -R $(whoami): tests/cmd

cargo --locked test -- --test cli_tests --test-threads 1
cargo --locked test --no-default-features -- --test cli_tests_any_no_git --test-threads 1

sudo chown -R $uid:$gid tests/cmd

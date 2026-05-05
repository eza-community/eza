uid=$(stat -c '%u' tests/)
gid=$(stat -c '%g' tests/)

export TRYCMD=overwrite

sudo chown -R $(whoami): tests/cmd

cargo --locked test -- --test cli_tests --test-threads 1
cargo --locked test --no-default-features -- --test no_git --test-threads 1

sudo chown -R $uid:$gid tests/cmd

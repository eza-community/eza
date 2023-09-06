Output outfile

Set Shell fish
Sleep 2s

Type "cd tests/itest"
Enter
Sleep 1s

Type "cargo run -q -- -l --no-user --no-time --no-filesize"
Enter
Sleep 3s

Type "cargo run -q -- -l --no-user --no-time --no-filesize"
Enter
Sleep 3s

Type "cargo run -q -- -l --grid --icons --grid --no-user --no-time"
Enter
Sleep 3s

Ctrl+D


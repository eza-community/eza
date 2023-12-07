Output docs/images/demo.gif
Set Width 1536
Set Height 512
Set Shell fish
Set FontFamily "FantasqueSansM Nerd Font Mono"
Set FontSize 13

Type "# eza is a modern, maintained replacement for ls."
Enter
Type "# "
Enter
Type "# It has too many features to cover in a gif, but we'll"
Enter
Type "# try to show off some cool stuff :3"
Enter
Type "# "
Enter
Sleep 4s
Enter

Type "# Let's start with the basics."
Enter
Type "eza src/"
Sleep 2s
Enter
Sleep 4s

Type "# ...but add some icons."
Enter
Type "eza -l --icons src/"
Sleep 2s
Enter
Sleep 4s

Type "# ...and how about a grid view."
Enter
Type "eza -l --icons --grid"
Sleep 2s
Enter
Sleep 4s

Type "# ...with headers."
Enter
Type "eza -l --icons --grid --header"
Sleep 2s
Enter
Sleep 4s

Type "# ...and perhaps some octal permissions."
Enter
Type "eza -l --icons --grid --header --octal-permissions"
Sleep 2s
Enter
Sleep 4s

Type "# ...or what about blocksize."
Enter
Type "eza -l --icons --grid --header --blocksize"
Sleep 2s
Enter
Sleep 4s

Type "# ...or hyperlinks."
Enter
Type "eza -l --icons --grid --header --hyperlink"
Sleep 2s
Enter
Sleep 4s
Type "# Admittedly, those aren't really very visual in a"
Enter
Type "# gif like this >_<"
Enter

Type "# ...what about a custom time style?"
Enter
Type 'eza -l --icons --grid --header --time-style "+%Y/%m/%d - week %W"'
Sleep 2s
Enter
Sleep 4s

Type "# ...or a bit of git status."
Enter
Type 'eza -l --icons --grid --header --git'
Sleep 2s
Enter
Sleep 4s

Type "# ...or a lot of git."
Enter
Type 'eza -l --icons --grid --header --git --git-repos'
Sleep 2s
Enter
Sleep 4s

Type "# Of course, this was only a glimpse off all "
Enter
Type "# the features that eza has to offer."
Enter
Type "# "
Enter
Type "# For more info, checkout the README.md or check"
Enter
Type "# out the github repo or the website at https://eza.rocks"
Enter
Type "# "
Enter
Type "# Thanks for watching <3"
Enter
Sleep 4s

# Type 'eza -l --icons --grid --blocksize -l --hyperlink --grid --git-repos -x --time-style "+%Y, %Y, %d%d%d - %a %u" --git --header --icons -Z -@ -o --group --smart-group --no-quotes'
# Sleep 500ms
# Enter
# Sleep 2s

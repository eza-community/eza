#!/usr/bin/env bash

if [ -z "$1" ]; then
    echo "Usage: $0 <output_dir>";
    exit 1;
fi

rm "$1" -rf;
mkdir -p "$1";
cd "$1" || exit;

# generate files of various age
# TODO: some are commented out due to undeterministic behavior, see:
# https://github.com/eza-community/eza/issues/574

touch --date="13 month ago"  ./13_month
#touch --date="11 month ago"  ./11_month
#touch --date="7 month ago"   ./07_month
#touch --date="5 month ago"   ./05_month
touch --date="now"           ./now
#touch --date="next hour"     ./next_hour
#touch --date="next month"    ./next_month
#touch --date="next year"     ./next_year

#!/usr/bin/env zsh
echo -e "Test: \e[1;31mENTRYPOINT\e[0m"
cargo +nightly test --color always -- --nocapture

# echo -e "Test: \e[1;31mLIB [BTREE]\e[0m"
# cargo +nightly test --color always --lib -p btree -- --nocapture
#
# echo -e "Test: \e[1;31mLIB [UTILS]\e[0m"
# cargo +nightly test --color always --lib -p utils -- --nocapture
#
# echo -e "Test: \e[1;31mLIB [MACROS]\e[0m"
# cargo +nightly test --color always --lib -p macro -- --nocapture
#
# echo -e "Test: \e[1;31mUNIT TEST\e[0m"
# cargo +nightly test --color always --lib --all -- --nocapture
#
# echo -e "Test: \e[1;31mLIB [BTREE]\e[0m"
# cargo +nightly test --color always --lib -p btree -- --nocapture

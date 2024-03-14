# justfile containing shortcuts for cryptopals :D
check part:
    cargo check --bin {{part}}
test part:
    cargo nextest run -v --bin {{part}}
testall:
    cargo nextest run -v
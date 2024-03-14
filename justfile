# justfile containing shortcuts for cryptopals :D
check part:
    cargo check --bin {{part}}
new part:
    cp src/template src/bin/{{part}}.rs
test part:
    cargo nextest run -v --bin {{part}}
testall:
    cargo nextest run -v
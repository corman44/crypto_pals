### justfile containing shortcuts for cryptopals :D

bench libs:
    cargo bench --bench bench_libs >> bench_libs.txt
check part:
    cargo check --bin {{part}}
lint part:
    cargo clippy -p {{part}}
new part:
    cp src/template src/bin/{{part}}.rs
test part:
    cargo nextest run -v --bin {{part}}
testall:
    cargo nextest run -v
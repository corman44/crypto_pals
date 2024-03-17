use crypto_pals::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn criterion_single_xor(c: &mut Criterion) {
    let bytes: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    c.bench_with_input(BenchmarkId::new("single_xor", 16), &bytes, |f, b| 
        f.iter(|| 
            single_byte_xor(b.to_vec(), 75)
        )
    );
}

pub fn criterion_score_single_byte(c: &mut Criterion) {
    let bytes: Vec<u8> = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
    c.bench_with_input(BenchmarkId::new("score_single_bytes", 16), &bytes, |f, b| 
        f.iter(|| 
            score_single_byte(b.to_vec())
        )
    );
}

criterion_group!(benches, criterion_single_xor, criterion_score_single_byte);
criterion_main!(benches);

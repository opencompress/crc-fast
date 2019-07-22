use criterion::{criterion_group, criterion_main, Criterion , black_box};
use adler32::RollingAdler32;

fn criterion_benchmark(c: &mut Criterion) {
    let mut adler32 = RollingAdler32::new());
    c.bench_function("adler32", |b| b.iter(|| RollingAdler32.update(black_box(b"hello checksum"))));
}

criterion_group!(bench, criterion_benchmark);
criterion_main!(bench);



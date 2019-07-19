use criterion::{criterion_group, criterion_main, Criterion , black_box};
use crc_any::CRC;

fn criterion_benchmark(c: &mut Criterion) {
    let mut crc_any_crc16 = CRC::crc16();
    c.bench_function("crc_any::crc16", |b| b.iter(|| crc_any_crc16.digest(black_box(b"hello checksum"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

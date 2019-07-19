#[macro_use]
extern crate criterion;

extern crate crc_any;

use crc_any::CRC;
use criterion::Criterion;
use criterion::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let mut crc_any_crc16 = CRC::crc16();
    c.bench_function("crc_any::crc16", |b| b.iter(|| crc_any_crc16.digest(black_box(b"hello checksum"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

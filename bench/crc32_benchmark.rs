use criterion::{criterion_group, criterion_main, Criterion , black_box};
use crc_any::CRC;
use crc::crc32;


fn crc_crc32_IEEE_benchmark(c: &mut Criterion) {
    let mut digest = crc32::Digest::new(crc32::IEEE);
    c.bench_function("crc::crc32_IEEE", |b| b.iter(|| digest.write(black_box(b"hello checksum"))));
}

fn crc_any_crc32_IEEE_benchmark(c: &mut Criterion) {
    let mut digest = CRC::crc32ieee();
    c.bench_function("crc::crc32_IEEE", |b| b.iter(|| digest.digest(black_box(b"hello checksum"))));
}


criterion_group!(benches, crc_crc32_IEEE_benchmark, crc_any_crc32_IEEE_benchmark);
criterion_main!(benches);

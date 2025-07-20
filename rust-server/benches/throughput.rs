use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use quantis_server::utils::RingBuffer;

fn benchmark_ring_buffer_write(c: &mut Criterion) {
    let buffer = RingBuffer::new(16 * 1024 * 1024); // 16MB
    let data = vec![0xAA; 4096]; // 4KB of data

    let mut group = c.benchmark_group("ring_buffer_write");
    group.throughput(Throughput::Bytes(data.len() as u64));
    
    group.bench_function("write_4kb", |b| {
        b.iter(|| {
            black_box(buffer.write(&data));
        })
    });
    
    group.finish();
}

fn benchmark_ring_buffer_read(c: &mut Criterion) {
    let buffer = RingBuffer::new(16 * 1024 * 1024);
    // Pre-fill buffer
    let data = vec![0xAA; 1024 * 1024]; // 1MB
    buffer.write(&data);

    let mut group = c.benchmark_group("ring_buffer_read");
    
    for size in [32, 256, 1024, 4096].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_function(format!("read_{}_bytes", size), |b| {
            b.iter(|| {
                black_box(buffer.read(*size));
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_ring_buffer_write, benchmark_ring_buffer_read);
criterion_main!(benches);
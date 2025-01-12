use criterion::{criterion_group, criterion_main, Criterion};
use rtsan_standalone::*;

#[nonblocking]
pub fn my_nonblocking() {}

#[blocking]
pub fn my_blocking() {}

#[no_sanitize_realtime]
pub fn my_no_sanitize() {}

#[nonblocking]
pub fn my_scoped_disabler() {
    scoped_disabler!({});
}

pub fn rtsan_bench(c: &mut Criterion) {
    ensure_initialized();

    c.bench_function("nonblocking", |b| {
        b.iter(|| {
            my_nonblocking();
        })
    });

    c.bench_function("blocking", |b| {
        b.iter(|| {
            my_blocking();
        })
    });

    c.bench_function("no-sanitize", |b| {
        b.iter(|| {
            my_no_sanitize();
        })
    });

    c.bench_function("scoped-disabler", |b| {
        b.iter(|| {
            my_scoped_disabler();
        })
    });
}

criterion_group!(benches, rtsan_bench);
criterion_main!(benches);

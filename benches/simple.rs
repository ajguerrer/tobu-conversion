use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use protobuf_conversion::heavy;
use protobuf_conversion::light;
use protobuf_conversion::reflect::Reflect;

fn fill_heavy_simple(m: &mut heavy::simple::Simple) {
    m.simple_bool = true;
}

fn fill_light_simple(m: &mut light::simple::Simple) {
    *m.simple_bool_mut() = true;
}

fn new(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple/new");

    group.bench_function("heavy", |b| {
        b.iter_with_large_drop(heavy::simple::Simple::new)
    });

    group.bench_function("light", |b| {
        b.iter_with_large_drop(light::simple::Simple::new)
    });

    group.finish();
}

fn access(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple/access");

    let mut m = heavy::simple::Simple::new();
    fill_heavy_simple(&mut m);
    group.bench_function("heavy", |b| {
        b.iter_batched_ref(|| &m, |m| m.simple_bool, BatchSize::SmallInput)
    });

    let mut m = light::simple::Simple::new();
    fill_light_simple(&mut m);
    group.bench_function("light", |b| {
        b.iter_batched_ref(|| &m, |m| m.simple_bool(), BatchSize::SmallInput)
    });

    group.finish();
}

fn mutate(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple/mutate");

    group.bench_function("heavy", |b| {
        b.iter_batched(
            heavy::simple::Simple::new,
            |mut m| fill_heavy_simple(&mut m),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("light", |b| {
        b.iter_batched(
            light::simple::Simple::new,
            |mut m| fill_light_simple(&mut m),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn reflect(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple/reflect");

    group.bench_function("heavy", |b| {
        b.iter_batched(
            heavy::simple::Simple::new,
            |m| m.reflect(),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("light", |b| {
        b.iter_batched(
            light::simple::Simple::new,
            |m| m.reflect(),
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn absorb(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple/absorb");

    let m = heavy::simple::Simple::new();
    let r = m.reflect();
    group.bench_function("heavy", |b| {
        b.iter_batched(|| r.clone(), |r| r.absorb(), BatchSize::SmallInput)
    });

    let m = light::simple::Simple::new();
    let r = m.reflect();
    group.bench_function("light", |b| {
        b.iter_batched(|| r.clone(), |r| r.absorb(), BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, new, access, mutate, reflect, absorb);
criterion_main!(benches);

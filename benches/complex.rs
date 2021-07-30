use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use protobuf_conversion::heavy;
use protobuf_conversion::light;
use protobuf_conversion::reflect::Reflect;

fn fill_heavy_complex(m: &mut heavy::complex::Complex) {
    m.optional_enum = Some(heavy::complex::ComplexEnum::Ten);

    m.repeated_bytes = vec![b"hello".to_vec(), b"world".to_vec()];

    let mut mn = heavy::complex::ComplexNested::new();
    mn.optional_string = Some("hello".to_string());
    m.map_message.insert(1, mn);

    let mut mn = heavy::complex::ComplexNested::new();
    mn.optional_string = Some("world".to_string());
    m.map_message.insert(10, mn);
}

fn fill_light_complex(m: &mut light::complex::Complex) {
    *m.optional_enum_mut() = light::complex::ComplexEnum::Ten;

    *m.repeated_bytes_mut() = vec![b"hello".to_vec(), b"world".to_vec()];

    let mut mn = light::complex::ComplexNested::new();
    *mn.optional_string_mut() = "hello".to_string();
    m.map_message_mut().insert(1, mn);

    let mut mn = light::complex::ComplexNested::new();
    *mn.optional_string_mut() = "world".to_string();
    m.map_message_mut().insert(10, mn);
}

fn new(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex/new");

    group.bench_function("heavy", |b| {
        b.iter_with_large_drop(heavy::complex::Complex::new)
    });

    group.bench_function("light", |b| {
        b.iter_with_large_drop(light::complex::Complex::new)
    });

    group.finish();
}

fn access(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex/access");

    let mut m = heavy::complex::Complex::new();
    fill_heavy_complex(&mut m);
    group.bench_function("heavy", |b| {
        b.iter_batched_ref(
            || &m,
            |m| (&m.optional_enum, &m.repeated_bytes, &m.map_message),
            BatchSize::SmallInput,
        )
    });

    let mut m = light::complex::Complex::new();
    fill_light_complex(&mut m);
    group.bench_function("light", |b| {
        b.iter_batched_ref(
            || &m,
            |m| (m.optional_enum(), m.repeated_bytes(), m.map_message()),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn mutate(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex/mutate");

    group.bench_function("heavy", |b| {
        b.iter_batched(
            heavy::complex::Complex::new,
            |mut m| fill_heavy_complex(&mut m),
            BatchSize::SmallInput,
        )
    });

    group.bench_function("light", |b| {
        b.iter_batched(
            light::complex::Complex::new,
            |mut m| fill_light_complex(&mut m),
            BatchSize::SmallInput,
        )
    });

    group.finish();
}

fn reflect(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex/reflect");

    let mut m = heavy::complex::Complex::new();
    fill_heavy_complex(&mut m);
    group.bench_function("heavy", |b| {
        b.iter_batched(|| m.clone(), |m| m.reflect(), BatchSize::SmallInput)
    });

    let mut m = light::complex::Complex::new();
    fill_light_complex(&mut m);
    group.bench_function("light", |b| {
        b.iter_batched(|| m.clone(), |m| m.reflect(), BatchSize::SmallInput)
    });

    group.finish();
}

fn absorb(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex/absorb");

    let mut m = heavy::complex::Complex::new();
    fill_heavy_complex(&mut m);
    let r = m.reflect();
    group.bench_function("heavy", |b| {
        b.iter_batched(|| r.clone(), |r| r.absorb(), BatchSize::SmallInput)
    });

    let mut m = light::complex::Complex::new();
    fill_light_complex(&mut m);
    let r = m.reflect();
    group.bench_function("light", |b| {
        b.iter_batched(|| r.clone(), |r| r.absorb(), BatchSize::SmallInput)
    });

    group.finish();
}

criterion_group!(benches, new, access, mutate, reflect, absorb);
criterion_main!(benches);

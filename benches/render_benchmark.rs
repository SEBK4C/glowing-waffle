use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glowing_waffle::{
    animation::Animation,
    renderer::{ColorMode, Renderer},
    waffle::{Size, Waffle},
};

fn waffle_creation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Waffle Creation");
    
    group.bench_function("small waffle", |b| {
        b.iter(|| Waffle::new(black_box(Size::Small)))
    });
    
    group.bench_function("medium waffle", |b| {
        b.iter(|| Waffle::new(black_box(Size::Medium)))
    });
    
    group.bench_function("large waffle", |b| {
        b.iter(|| Waffle::new(black_box(Size::Large)))
    });
    
    group.finish();
}

fn waffle_rotation_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Waffle Rotation");
    
    let mut small_waffle = Waffle::new(Size::Small);
    let mut medium_waffle = Waffle::new(Size::Medium);
    let mut large_waffle = Waffle::new(Size::Large);
    
    group.bench_function("rotate small waffle", |b| {
        b.iter(|| {
            black_box(&mut small_waffle).rotate(0.01, 0.02, 0.003);
        })
    });
    
    group.bench_function("rotate medium waffle", |b| {
        b.iter(|| {
            black_box(&mut medium_waffle).rotate(0.01, 0.02, 0.003);
        })
    });
    
    group.bench_function("rotate large waffle", |b| {
        b.iter(|| {
            black_box(&mut large_waffle).rotate(0.01, 0.02, 0.003);
        })
    });
    
    group.finish();
}

fn animation_update_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Animation Update");
    
    let mut animation = Animation::new();
    let mut small_waffle = Waffle::new(Size::Small);
    let mut medium_waffle = Waffle::new(Size::Medium);
    let mut large_waffle = Waffle::new(Size::Large);
    
    group.bench_function("animate small waffle", |b| {
        b.iter(|| {
            black_box(&mut animation).update(black_box(&mut small_waffle));
        })
    });
    
    group.bench_function("animate medium waffle", |b| {
        b.iter(|| {
            black_box(&mut animation).update(black_box(&mut medium_waffle));
        })
    });
    
    group.bench_function("animate large waffle", |b| {
        b.iter(|| {
            black_box(&mut animation).update(black_box(&mut large_waffle));
        })
    });
    
    group.finish();
}

// Note: Cannot benchmark rendering in an automated fashion since it requires terminal interaction
// These benchmarks focus on the computation aspects only

criterion_group!(
    benches,
    waffle_creation_benchmark,
    waffle_rotation_benchmark,
    animation_update_benchmark
);
criterion_main!(benches);

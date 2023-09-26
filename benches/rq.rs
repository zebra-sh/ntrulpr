use criterion::{criterion_group, criterion_main, Criterion};
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};

fn encoder_benchmark(cb: &mut Criterion) {
    let mut rng = NTRURandom::new();
    let coeffs = rng.short_random().unwrap();
    let rq = Rq::from(coeffs);
    let r3 = R3::from(rng.random_small().unwrap());

    cb.bench_function("rq_recip_1", |b| {
        b.iter(|| {
            rq.recip::<1>().unwrap();
        });
    });
    cb.bench_function("rq_recip_3", |b| {
        b.iter(|| {
            rq.recip::<3>().unwrap();
        });
    });

    cb.bench_function("rq_mult_int_to_3", |b| {
        b.iter(|| {
            rq.mult_int(3);
        });
    });

    cb.bench_function("rq_mult_to_r3", |b| {
        b.iter(|| {
            rq.mult_r3(&r3);
        });
    });
}

criterion_group!(benches, encoder_benchmark);
criterion_main!(benches);

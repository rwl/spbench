use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spbench::*;
use std::thread::available_parallelism;

#[derive(Clone)]
struct Input<S: rlu::Scalar> {
    n_cpu: usize,
    lu: rlu::LU<S>,
    rhs: Vec<S>,
}

fn jac_par_solve_benchmark(c: &mut Criterion) {
    let a = case_activsg2000_jac();
    // let a = case_activsg10k_jac();
    let n = a.0;
    let x = (0..n)
        .map(|i| 1.0 + i as f64 / n as f64)
        .collect::<Vec<f64>>();
    par_solve_benchmark(c, a, &x);
}

fn par_solve_benchmark<S: rlu::Scalar + Sync + Send>(
    c: &mut Criterion,
    d: (usize, Vec<usize>, Vec<usize>, Vec<S>),
    x: &[S],
) {
    let (n, a_p, a_i, a_x) = d;

    let n_rhs = n / 1;

    let control = amd::Control::default();

    let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
    assert_eq!(info.status, amd::Status::OK);
    // amd::info(&info);

    let options = rlu::Options::default();
    let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &options).unwrap();

    let rhs = x.repeat(n_rhs);

    let available_parallelism_approx = available_parallelism().unwrap().get();

    let inputs = (1..=available_parallelism_approx)
        .map(|n_cpu| Input {
            n_cpu,
            lu: lu.clone(),
            rhs: rhs.clone(),
        })
        .collect::<Vec<Input<S>>>();

    let mut group = c.benchmark_group("par_solve");
    group.sample_size(10);

    for input in inputs.iter() {
        group.throughput(Throughput::Elements(input.n_cpu as u64));
        group.bench_with_input(BenchmarkId::from_parameter(input.n_cpu), input, |b, d| {
            b.iter(|| {
                let mut rhs = d.rhs.clone();

                rayon::ThreadPoolBuilder::new()
                    .num_threads(d.n_cpu)
                    .build()
                    .unwrap()
                    .install(|| {
                        rlu::par_solve(black_box(&d.lu), black_box(&mut rhs), black_box(false))
                            .unwrap();
                    });
            });
        });
    }
    group.finish();
}

criterion_group!(benches, jac_par_solve_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spbench::*;

#[derive(Clone)]
struct Input {
    n: usize,
    a_p: Vec<usize>,
    a_i: Vec<usize>,
    a_x: Vec<f64>,
}

impl Input {
    fn new(n: usize, a_p: &[usize], a_i: &[usize], a_x: &[f64]) -> Self {
        Self {
            n,
            a_p: a_p.to_vec(),
            a_i: a_i.to_vec(),
            a_x: a_x.to_vec(),
        }
    }
}

fn benchmark_inputs(c: &mut Criterion, group_name: &str, inputs: &[Input]) {
    let mut group = c.benchmark_group(group_name);
    group.sample_size(10);

    for input in inputs.iter() {
        group.throughput(Throughput::Elements(input.n as u64));
        group.bench_with_input(BenchmarkId::new("rlu::factor", input.n), input, |b, d| {
            b.iter(|| {
                let control = amd::Control::default();
                let (p, _p_inv, _info) = amd::order(d.n, &d.a_p, &d.a_i, &control).unwrap();
                let options = rlu::Options::default();
                let lu = rlu::factor(d.n, &d.a_i, &d.a_p, &d.a_x, Some(&p), &options).unwrap();
                black_box(lu);
            });
        });
        group.bench_with_input(BenchmarkId::new("klu::factor", input.n), input, |b, d| {
            b.iter(|| {
                let common = klu::defaults();
                let mut symbolic = klu::analyze(d.n, &d.a_p, &d.a_i, &common).unwrap();
                let mut a_x = d.a_x.clone();
                let numeric =
                    klu::factor(&d.a_p, &d.a_i, &mut a_x, &mut symbolic, &common).unwrap();
                black_box(numeric);
            });
        });
    }
    group.finish();
}

pub fn bbus_factor_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        case_activsg2000_bbus(!trans),
        case_activsg10k_bbus(!trans),
        case_activsg25k_bbus(!trans),
        case_activsg70k_bbus(!trans),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2, &d.3))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "factor(bbus)", &inputs);
}

// pub fn ybus_factor_benchmark(c: &mut Criterion) {
//     let trans = false;
//     let inputs = [
//         case_activsg2000_ybus(!trans),
//         case_activsg10k_ybus(!trans),
//         // case_activsg25k_ybus(!trans),
//         // case_activsg70k_ybus(!trans),
//     ]
//     .iter()
//     .map(|d| Input::new(d.0, &d.1, &d.2, &d.3))
//     .collect::<Vec<Input>>();
//
//     benchmark_inputs(c, "factor(ybus)", &inputs);
// }

pub fn jac_factor_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        case_activsg2000_jac(!trans),
        // case_activsg10k_jac(!trans),
        // case_activsg25k_jac(!trans),
        // case_activsg70k_jac(!trans),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2, &d.3))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "factor(jac)", &inputs);
}

criterion_group!(
    benches,
    bbus_factor_benchmark,
    // ybus_factor_benchmark,
    // jac_factor_benchmark
);
criterion_main!(benches);

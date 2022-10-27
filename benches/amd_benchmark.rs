use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use spbench::*;

#[derive(Clone)]
struct Input {
    n: usize,
    a_p: Vec<usize>,
    a_i: Vec<usize>,
    control: amd::Control,
}

impl Input {
    fn new(n: usize, a_p: &[usize], a_i: &[usize]) -> Self {
        Self {
            n,
            a_p: a_p.to_vec(),
            a_i: a_i.to_vec(),
            control: amd::Control::default(),
        }
    }
}

fn benchmark_inputs(c: &mut Criterion, group_name: &str, inputs: &[Input]) {
    let mut group = c.benchmark_group(group_name);

    for input in inputs.iter() {
        group.throughput(Throughput::Elements(input.n as u64));
        group.bench_with_input(BenchmarkId::from_parameter(input.n), input, |b, d| {
            b.iter(|| {
                amd::order(
                    black_box(d.n),
                    black_box(&d.a_p),
                    black_box(&d.a_i),
                    black_box(&d.control),
                )
            });
        });
    }
    group.finish();
}

fn amd_bbus_benchmark(c: &mut Criterion) {
    let inputs = [
        case_activsg2000_bbus(),
        case_activsg10k_bbus(),
        case_activsg25k_bbus(),
        case_activsg70k_bbus(),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Bbus", &inputs);
}

fn amd_ybus_benchmark(c: &mut Criterion) {
    let inputs = [
        case_activsg2000_ybus(),
        case_activsg10k_ybus(),
        case_activsg25k_ybus(),
        case_activsg70k_ybus(),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Ybus", &inputs);
}

fn amd_jac_benchmark(c: &mut Criterion) {
    let inputs = [
        case_activsg2000_jac(),
        case_activsg10k_jac(),
        case_activsg25k_jac(),
        case_activsg70k_jac(),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Jac", &inputs);
}

criterion_group!(
    benches,
    amd_bbus_benchmark,
    amd_ybus_benchmark,
    amd_jac_benchmark
);
criterion_main!(benches);

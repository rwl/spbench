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

pub fn bbus_order_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        case_activsg2000_bbus(!trans),
        // case_activsg10k_bbus(!trans),
        // case_activsg25k_bbus(!trans),
        // case_activsg70k_bbus(!trans),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Bbus", &inputs);
}

pub fn ybus_order_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        case_activsg2000_ybus(!trans),
        // case_activsg10k_ybus(!trans),
        // case_activsg25k_ybus(!trans),
        // case_activsg70k_ybus(!trans),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Ybus", &inputs);
}

pub fn jac_order_benchmark(c: &mut Criterion) {
    let trans = false;
    let inputs = [
        case_activsg2000_jac(!trans),
        // case_activsg10k_jac(!trans),
        // case_activsg25k_jac(!trans),
        // case_activsg70k_jac(!trans),
    ]
    .iter()
    .map(|d| Input::new(d.0, &d.1, &d.2))
    .collect::<Vec<Input>>();

    benchmark_inputs(c, "AMD Jac", &inputs);
}

criterion_group!(
    benches,
    // bbus_order_benchmark,
    ybus_order_benchmark,
    // jac_order_benchmark
);
criterion_main!(benches);

use cpuprofiler::PROFILER;
use spbench::*;
use std::time::Instant;

fn main() {
    let trans = false;

    let t0 = Instant::now();
    let (n, a_p, a_i, a_x) = case_activsg10k_bbus(!trans);

    let control = amd::Control::default();

    let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
    assert_eq!(info.status, amd::Status::OK);
    // amd::info(&info);

    let options = rlu::Options::default();

    PROFILER.lock().unwrap().start("./spbench.profile").unwrap();
    let _lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &options).unwrap();
    PROFILER.lock().unwrap().stop().unwrap();

    println!("factor: {:?}", t0.elapsed());
    // let t0 = Instant::now();
    //
    // let mut b = vec![1.0; n * (n / 2)];
    //
    // rlu::solve(&lu, &mut b, trans).unwrap();
    // // rlu::par_solve(&lu, &mut b, false).unwrap();
    //
    // println!("solve: {:?}", t0.elapsed());
}

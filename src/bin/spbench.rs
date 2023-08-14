// use cpuprofiler::PROFILER;
use spbench::*;
use std::time::Instant;

fn main() {
    let trans = false;

    // let (n, a_p, a_i, a_x) = case_activsg10k_bbus(!trans);
    // let (n, a_p, a_i, a_x) = case_activsg25k_bbus(!trans);
    let (n, a_p, a_i, a_x) = case_activsg70k_bbus(!trans);

    let p = {
        let t0 = Instant::now();
        let control = amd::Control::default();

        let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
        assert_eq!(info.status, amd::Status::OK);
        // amd::info(&info);
        println!("order: {:?}", t0.elapsed());
        p
    };

    run_lufact(n, &a_p, &a_i, &a_x, &p, trans);
    run_rlu(n, &a_p, &a_i, &a_x, &p, trans);
    run_klu(n, &a_p, &a_i, &a_x, &p, trans);
}

fn run_lufact(n: usize, a_p: &[usize], a_i: &[usize], a_x: &[f64], p: &[usize], trans: bool) {
    let mut gp = lufact::GP::default();
    gp.col_perm = Some(p.iter().map(|i| *i as i32).collect());

    let mut a_desc = lufact::CSC {
        m: n as i32,
        n: n as i32,
        nnz: a_x.len() as i32,
        base: 0,
        colptr: a_p.iter().map(|p| *p as i32).collect(),
        rowind: a_i.iter().map(|i| *i as i32).collect(),
    };

    let t0 = Instant::now();
    let mut lu = lufact::dgstrf(&gp, n as i32, n as i32, &a_x, &mut a_desc).unwrap();
    println!("dgstrf: {:?}", t0.elapsed());

    let mut b = make_b(n);

    {
        let t0 = Instant::now();
        let _ierr = lufact::dgstrs(
            &gp,
            if trans { 'T' } else { 'N' },
            n as i32,
            1,
            &mut lu,
            1,
            1,
            &mut b,
            1,
            1, /*, -1*/
        );
        println!("dgstrs: {:?}", t0.elapsed());
    }
}

fn run_klu(n: usize, a_p: &[usize], a_i: &[usize], a_x0: &[f64], _p: &[usize], _trans: bool) {
    let mut common = klu::defaults();

    let t0 = Instant::now();
    let mut symbolic = klu::analyze(n, &a_p, &a_i, &mut common).unwrap();
    println!("symbolic: {:?}", t0.elapsed());

    let mut a_x = a_x0.to_vec();

    let t0 = Instant::now();
    let mut numeric = klu::factor(&a_p, &a_i, &mut a_x, &mut symbolic, &common).unwrap();
    println!("numeric: {:?}", t0.elapsed());

    let mut b = make_b(n);

    let t0 = Instant::now();
    klu::solve(&mut symbolic, &mut numeric, n, 1, &mut b, &mut common).unwrap();
    println!("klusolve: {:?}", t0.elapsed());
}

fn run_rlu(n: usize, a_p: &[usize], a_i: &[usize], a_x: &[f64], p: &[usize], trans: bool) {
    let t0 = Instant::now();

    // PROFILER.lock().unwrap().start("./spbench.profile").unwrap();
    let options = rlu::Options::default();
    let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &options).unwrap();
    // PROFILER.lock().unwrap().stop().unwrap();

    println!("factor: {:?}", t0.elapsed());

    let mut b = make_b(n);

    {
        let t0 = Instant::now();

        rlu::solve(&lu, &mut b, trans).unwrap();
        // rlu::par_solve(&lu, &mut b, false).unwrap();

        println!("solve: {:?}", t0.elapsed());
    }
}

fn make_b(n: usize) -> Vec<f64> {
    // vec![1.0; n * (n / 2)]

    (0..n)
        .map(|i| 1.0 + i as f64 / n as f64)
        .collect::<Vec<f64>>()
}

use crate::mtx::*;
use approx::assert_abs_diff_eq;
use num_complex::Complex64;
use num_traits::{ToPrimitive, Zero};
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

const EPSILON: f64 = 1e-11;

#[test]
fn test_bbus() {
    let trans = false;
    for d in [
        case_activsg2000_bbus(!trans),
        // case_activsg10k_bbus(!trans),
        // case_activsg25k_bbus(!trans),
        // case_activsg70k_bbus(!trans),
    ] {
        let n = d.0;
        let x = Vec::from(
            [
                (0..n)
                    .map(|i| 1.0 + i as f64 / n as f64)
                    .collect::<Vec<f64>>(),
                (0..n)
                    .map(|i| -1.0 - i as f64 / n as f64)
                    .collect::<Vec<f64>>(),
            ]
            .concat(),
        );

        let b = test_rlu(d, &x, trans);

        for i in 0..n {
            assert_abs_diff_eq!(b[i], x[i], epsilon = EPSILON);
        }
    }
}

#[test]
fn test_ybus() {
    let trans = false;
    for d in [
        case_activsg2000_ybus(!trans),
        case_activsg10k_ybus(!trans),
        case_activsg25k_ybus(!trans),
        // case_activsg70k_ybus(),
    ] {
        let n = d.0;
        let x = Vec::from(
            [
                (0..n)
                    .map(|i| Complex64::new(1.0 + i as f64 / n as f64, 0.0))
                    .collect::<Vec<Complex64>>(),
                (0..n)
                    .map(|i| Complex64::new(1.0 + i as f64 / n as f64, 1.0 + i as f64 / n as f64))
                    .collect::<Vec<Complex64>>(),
                (0..n)
                    .map(|i| Complex64::new(0.0, 1.0 + i as f64 / n as f64))
                    .collect::<Vec<Complex64>>(),
                (0..n)
                    .map(|i| Complex64::new(-1.0 - i as f64 / n as f64, 0.0))
                    .collect::<Vec<Complex64>>(),
                (0..n)
                    .map(|i| Complex64::new(-1.0 - i as f64 / n as f64, -1.0 - i as f64 / n as f64))
                    .collect::<Vec<Complex64>>(),
                (0..n)
                    .map(|i| Complex64::new(0.0, -1.0 - i as f64 / n as f64))
                    .collect::<Vec<Complex64>>(),
            ]
            .concat(),
        );
        let b = test_rlu(d, &x, trans);
        for i in 0..n {
            assert_abs_diff_eq!(b[i].re, x[i].re, epsilon = EPSILON);
            assert_abs_diff_eq!(b[i].im, x[i].im, epsilon = EPSILON);
        }
    }
}

#[test]
fn test_jac() {
    let trans = false;
    for d in [
        case_activsg2000_jac(!trans),
        // case_activsg10k_jac(!trans),
        // case_activsg25k_jac(!trans),
        // case_activsg70k_jac(!trans),
    ] {
        let n = d.0;
        let x = Vec::from(
            [
                (0..n)
                    .map(|i| 1.0 + i as f64 / n as f64)
                    .collect::<Vec<f64>>(),
                (0..n)
                    .map(|i| -1.0 - i as f64 / n as f64)
                    .collect::<Vec<f64>>(),
            ]
            .concat(),
        );
        let b = test_rlu(d, &x, trans);
        for i in 0..n {
            assert_abs_diff_eq!(b[i], x[i], epsilon = EPSILON);
        }
    }
}

fn test_rlu<S: AddAssign + rlu::Scalar + Send + Sync + Debug>(
    d: (usize, Vec<usize>, Vec<usize>, Vec<S>),
    x: &[S],
    trans: bool,
) -> Vec<S> {
    let (n, a_p, a_i, a_x) = d;

    let control = amd::Control::default();

    let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
    assert_eq!(info.status, amd::Status::OK);
    // amd::info(&info);

    let options = rlu::Options::default();

    let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &options).unwrap();

    let mut b = Vec::<S>::with_capacity(x.len());
    for x_i in x.chunks_exact(n) {
        let b_i = if trans {
            csr_mat_vec(n, &a_i, &a_p, &a_x, x_i)
        } else {
            csc_mat_vec(n, &a_i, &a_p, &a_x, x_i)
        };

        // println!("{:?}", x_i.to_vec());
        // println!("{:?}", b_i.to_vec());

        b.extend(b_i);
    }

    rlu::solve(&lu, &mut b, trans).unwrap();
    // rlu::par_solve(&lu, &mut b, false).unwrap();

    b
}

pub fn csc_mat_vec<I: Copy + ToPrimitive, S: Copy + Zero + Mul<Output = S> + AddAssign>(
    n: usize,
    a_i: &[I],
    a_p: &[I],
    a_x: &[S],
    x: &[S],
) -> Vec<S> {
    let mut y = vec![S::zero(); n];
    for j in 0..n {
        let start = a_p[j].to_usize().unwrap();
        let end = a_p[j + 1].to_usize().unwrap();

        for ii in start..end {
            let i = a_i[ii].to_usize().unwrap();
            y[i] += a_x[ii] * x[j];
        }
    }
    y
}

pub fn csr_mat_vec<I: Copy + ToPrimitive, S: Copy + Zero + Mul<Output = S> + AddAssign>(
    n: usize,
    a_i: &[I],
    a_p: &[I],
    a_x: &[S],
    x: &[S],
) -> Vec<S> {
    let mut y = vec![S::zero(); n];
    for i in 0..n {
        let start = a_p[i].to_usize().unwrap();
        let end = a_p[i + 1].to_usize().unwrap();

        for jj in start..end {
            let j = a_i[jj].to_usize().unwrap();
            y[i] += a_x[jj] * x[j];
        }
    }
    y
}

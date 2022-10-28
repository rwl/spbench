use crate::mtx::*;
use approx::assert_abs_diff_eq;
use num_complex::Complex64;
use num_traits::Zero;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

const EPSILON: f64 = 1e-11;

#[test]
fn test_ybus() {
    // for d in [
    //     case_activsg2000_bbus(),
    //     // case_activsg10k_bbus(),
    //     // case_activsg25k_bbus(),
    //     // case_activsg70k_bbus(),
    // ] {
    //     let n = d.0;
    //     // let x = vec![vec![1.0; n], vec![-1.0; n]];
    //     let x = vec![vec![1.0; n]];
    //     let b = test_rlu(d, &x);
    //     for i in 0..x.len() {
    //         for j in 0..n {
    //             assert_abs_diff_eq!(b[i][j], x[i][j], epsilon = EPSILON);
    //         }
    //     }
    // }

    for d in [
        case_activsg2000_ybus(),
        case_activsg10k_ybus(),
        case_activsg25k_ybus(),
        // case_activsg70k_ybus(),
    ] {
        let n = d.0;
        let x = vec![
            vec![Complex64::new(1.0, 0.0); n],
            vec![Complex64::new(1.0, 1.0); n],
            vec![Complex64::new(0.0, 1.0); n],
            vec![Complex64::new(-1.0, 0.0); n],
            vec![Complex64::new(-1.0, -1.0); n],
            vec![Complex64::new(0.0, -1.0); n],
        ];
        let b = test_rlu(d, &x);
        for i in 0..x.len() {
            for j in 0..n {
                assert_abs_diff_eq!(b[i][j].re, x[i][j].re, epsilon = EPSILON);
                assert_abs_diff_eq!(b[i][j].im, x[i][j].im, epsilon = EPSILON);
            }
        }
    }

    for d in [
        case_activsg2000_jac(),
        case_activsg10k_jac(),
        case_activsg25k_jac(),
        // case_activsg70k_jac(),
    ] {
        let n = d.0;
        let x = vec![vec![1.0; n], vec![-1.0; n]];
        let b = test_rlu(d, &x);
        for i in 0..x.len() {
            for j in 0..n {
                assert_abs_diff_eq!(b[i][j], x[i][j], epsilon = EPSILON);
            }
        }
    }
}

fn test_rlu<S: AddAssign + rlu::Scalar + Debug>(
    d: (usize, Vec<usize>, Vec<usize>, Vec<S>),
    x: &[Vec<S>],
) -> Vec<Vec<S>> {
    let (n, a_p, a_i, a_x) = d;

    let control = amd::Control::default();

    let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
    assert_eq!(info.status, amd::Status::OK);
    // amd::info(&info);

    let options = rlu::Options::default();
    let lu = rlu::factor(n, &a_i, &a_p, &a_x, Some(&p), &options).unwrap();

    let mut b: Vec<Vec<S>> = vec![];
    for x_i in x {
        let b_i = mat_vec(n, &a_i, &a_p, &a_x, x_i);
        b.push(b_i);
    }

    let mut rhs: Vec<&mut [S]> = vec![];
    for b_i in &mut b {
        rhs.push(b_i);
    }

    rlu::solve(&lu, &mut rhs, false).unwrap();

    b
}

fn mat_vec<S: Copy + Zero + Mul<Output = S> + AddAssign>(
    n: usize,
    a_i: &[usize],
    a_p: &[usize],
    a_x: &[S],
    x: &[S],
) -> Vec<S> {
    let mut y = vec![S::zero(); n];
    for j in 0..n {
        let start = a_p[j];
        let end = a_p[j + 1];

        for ii in start..end {
            let i = a_i[ii];
            y[i] += a_x[ii] * x[j];
        }
    }
    y
}

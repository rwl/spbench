use crate::klu;
use crate::mtx::*;
use crate::test::solve_test::{csc_mat_vec, csr_mat_vec};
use approx::assert_abs_diff_eq;

const EPSILON: f64 = 1e-9;

#[test]
fn test_bbus() {
    let trans = false;
    for d in [
        // case_activsg2000_bbus(!trans),
        // case_activsg10k_bbus(!trans),
        case_activsg25k_bbus(!trans),
        // case_activsg70k_bbus(!trans),
    ] {
        let n = d.0;
        let x = Vec::from(
            [
                (0..n)
                    .map(|i| 1.0 + i as f64 / n as f64)
                    .collect::<Vec<f64>>(),
                // (0..n)
                //     .map(|i| -1.0 - i as f64 / n as f64)
                //     .collect::<Vec<f64>>(),
            ]
            .concat(),
        );

        let b = test_klu(d, &x, trans);

        for i in 0..n {
            assert_abs_diff_eq!(b[i], x[i], epsilon = EPSILON);
        }
    }
}

fn test_klu(d: (usize, Vec<usize>, Vec<usize>, Vec<f64>), x: &[f64], trans: bool) -> Vec<f64> {
    let (n, a_p, a_i, mut a_x) = d;

    let mut common = klu::defaults();
    let mut symbolic = klu::analyze(n, &a_p, &a_i, &mut common).unwrap();
    let mut numeric = klu::factor(&a_p, &a_i, &mut a_x, &mut symbolic, &common).unwrap();

    let mut b = Vec::<f64>::with_capacity(x.len());
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

    klu::solve(&mut symbolic, &mut numeric, n, 1, &mut b, &mut common).unwrap();

    b
}

#[test]
fn test_klu5() {
    let n = 5;
    let a_p = vec![0, 2, 5, 9, 10, 12];
    let a_i = vec![0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4];
    let mut a_x = vec![
        2.0, 3.0, //
        3.0, -1.0, 4.0, //
        4.0, -3.0, 1.0, 2.0, //
        2.0, //
        6.0, 1.0, //
    ];
    let mut b = vec![8.0, 45.0, -3.0, 3.0, 19.0];

    let mut common = klu::defaults();
    let mut symbolic = klu::analyze(n, &a_p, &a_i, &mut common).unwrap();
    let mut numeric = klu::factor(&a_p, &a_i, &mut a_x, &mut symbolic, &common).unwrap();

    klu::solve(&mut symbolic, &mut numeric, 5, 1, &mut b, &mut common).unwrap();

    println!("{:?}", b.to_vec());
}

use crate::mtx::*;
use crate::test::solve_test::{csc_mat_vec, csr_mat_vec};
use approx::assert_abs_diff_eq;

const EPSILON: f64 = 1e-11;

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

        let b = test_lufact(d, &x, trans);

        for i in 0..n {
            assert_abs_diff_eq!(b[i], x[i], epsilon = EPSILON);
        }
    }
}

fn test_lufact(d: (usize, Vec<usize>, Vec<usize>, Vec<f64>), x: &[f64], trans: bool) -> Vec<f64> {
    let (n, a_p, a_i, a_x) = d;

    let control = amd::Control::default();

    let (p, _p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();
    assert_eq!(info.status, amd::Status::OK);

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

    let mut lu = lufact::dgstrf(&gp, n as i32, n as i32, &a_x, &mut a_desc).unwrap();

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

    // let mut x = b.clone();
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
    // let rv = blu.solve_dense(&b.clone(), &mut b, );
    // rv.unwrap();
    // assert_eq!(rv, Status::OK);

    b
}

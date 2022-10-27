use num_complex::Complex64;
use sprs::io::read_matrix_market;
use sprs::num_kinds::PrimitiveKind;
use sprs::num_matrixmarket::{MatrixMarketConjugate, MatrixMarketRead};
use std::ops::{Add, Neg};
use std::path::PathBuf;

pub fn case_activsg2000_bbus() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg2000_Bbus")
}

pub fn case_activsg2000_ybus() -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg2000_Ybus")
}

pub fn case_activsg2000_jac() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg2000_Jac")
}

pub fn case_activsg10k_bbus() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg10k_Bbus")
}

pub fn case_activsg10k_ybus() -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg10k_Ybus")
}

pub fn case_activsg10k_jac() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg10k_Jac")
}

pub fn case_activsg25k_bbus() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg25k_Bbus")
}

pub fn case_activsg25k_ybus() -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg25k_Ybus")
}

pub fn case_activsg25k_jac() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg25k_Jac")
}

pub fn case_activsg70k_bbus() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg70k_Bbus")
}

pub fn case_activsg70k_ybus() -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg70k_Ybus")
}

pub fn case_activsg70k_jac() -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg70k_Jac")
}

fn read_matpower_data<
    S: Clone
        + PrimitiveKind
        + MatrixMarketRead
        + MatrixMarketConjugate
        + Neg<Output = S>
        + Add<Output = S>,
>(
    name: &str,
) -> (usize, Vec<usize>, Vec<usize>, Vec<S>) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("data/matpower");
    d.push(name.to_string() + ".mtx");

    let tri = read_matrix_market(d.to_str().unwrap()).unwrap();

    let csc = tri.to_csc();

    let n = csc.cols();
    let a_p = csc.indptr().into_raw_storage().to_vec();
    let a_i = csc.indices().to_vec();
    let data = csc.data().to_vec();

    (n, a_p, a_i, data)
}

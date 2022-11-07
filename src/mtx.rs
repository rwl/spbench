use num_complex::Complex64;
use sprs::io::read_matrix_market;
use sprs::num_kinds::PrimitiveKind;
use sprs::num_matrixmarket::{MatrixMarketConjugate, MatrixMarketRead};
use std::ops::{Add, Neg};
use std::path::PathBuf;

pub fn case_activsg2000_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg2000_Bbus", csc)
}

pub fn case_activsg2000_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg2000_Ybus", csc)
}

pub fn case_activsg2000_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg2000_Jac", csc)
}

pub fn case_activsg10k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg10k_Bbus", csc)
}

pub fn case_activsg10k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg10k_Ybus", csc)
}

pub fn case_activsg10k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg10k_Jac", csc)
}

pub fn case_activsg25k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg25k_Bbus", csc)
}

pub fn case_activsg25k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg25k_Ybus", csc)
}

pub fn case_activsg25k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg25k_Jac", csc)
}

pub fn case_activsg70k_bbus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg70k_Bbus", csc)
}

pub fn case_activsg70k_ybus(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<Complex64>) {
    read_matpower_data("case_ACTIVSg70k_Ybus", csc)
}

pub fn case_activsg70k_jac(csc: bool) -> (usize, Vec<usize>, Vec<usize>, Vec<f64>) {
    read_matpower_data("case_ACTIVSg70k_Jac", csc)
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
    csc: bool,
) -> (usize, Vec<usize>, Vec<usize>, Vec<S>) {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("data/matpower");
    d.push(name.to_string() + ".mtx");

    let tri = read_matrix_market(d.to_str().unwrap()).unwrap();

    let a = if csc { tri.to_csc() } else { tri.to_csr() };

    let n = a.cols();
    let a_p = a.indptr().into_raw_storage().to_vec();
    let a_i = a.indices().to_vec();
    let data = a.data().to_vec();

    (n, a_p, a_i, data)
}

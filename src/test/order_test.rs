use crate::mtx::*;
use amd::Status;

#[test]
fn test_amd() {
    let (n, a_p, a_i, _) = case_activsg2000_ybus(true);

    let control = amd::Control::default();

    let (p, p_inv, info) = amd::order::<usize>(n, &a_p, &a_i, &control).unwrap();

    assert_eq!(p.len(), n);
    assert_eq!(p_inv.len(), n);
    assert_eq!(info.status, Status::OK);
}

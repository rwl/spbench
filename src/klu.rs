use klu_sys::{
    klu_analyze, klu_defaults, klu_factor, klu_free_numeric, klu_free_symbolic, klu_solve,
    KluCommon, KluNumeric, KluSymbolic,
};
use std::alloc::Layout;
use std::ptr::NonNull;

pub struct Common {
    ctx: Box<KluCommon>,
}

impl Common {
    fn as_ffi(&self) -> *mut KluCommon {
        self.ctx.as_ref() as *const KluCommon as *mut KluCommon
    }
}

pub struct Symbolic {
    common: *mut KluCommon,
    ctx: NonNull<KluSymbolic>,
}

impl Drop for Symbolic {
    fn drop(&mut self) {
        if unsafe { klu_free_symbolic(&mut self.ctx.as_ptr(), self.common) } == 0 {
            unreachable!("free symbolic failed")
        }
    }
}

pub struct Numeric {
    common: *mut KluCommon,
    ctx: NonNull<KluNumeric>,
}

impl Drop for Numeric {
    fn drop(&mut self) {
        if unsafe { klu_free_numeric(&mut self.ctx.as_ptr(), self.common) } == 0 {
            unreachable!("free numeric failed")
        }
    }
}

pub fn defaults() -> Common {
    unsafe {
        let raw = std::alloc::alloc(Layout::new::<KluCommon>()) as *mut KluCommon;
        klu_defaults(raw);
        Common {
            ctx: Box::from_raw(raw),
        }
    }
}

pub fn analyze(
    n: usize,
    a_p: &[usize],
    a_i: &[usize],
    common: &Common,
) -> Result<Symbolic, String> {
    let a_p32: Vec<i32> = a_p.iter().map(|&v| v as i32).collect();
    let a_i32: Vec<i32> = a_i.iter().map(|&v| v as i32).collect();
    let symbolic =
        unsafe { klu_analyze(n as i32, a_p32.as_ptr(), a_i32.as_ptr(), common.as_ffi()) };
    if symbolic.is_null() {
        Err("klu_analyze failed".to_string())
    } else {
        Ok(Symbolic {
            common: common.as_ffi(),
            ctx: NonNull::new(symbolic).unwrap(),
        })
    }
}

pub fn factor(
    a_p: &[usize],
    a_i: &[usize],
    a_x: &mut [f64],
    symbolic: &Symbolic,
    common: &Common,
) -> Result<Numeric, String> {
    let a_p32: Vec<i32> = a_p.iter().map(|&v| v as i32).collect();
    let a_i32: Vec<i32> = a_i.iter().map(|&v| v as i32).collect();
    let numeric = unsafe {
        klu_factor(
            a_p32.as_ptr(),
            a_i32.as_ptr(),
            a_x.as_mut_ptr(),
            symbolic.ctx.as_ptr(),
            common.as_ffi(),
        )
    };
    if numeric.is_null() {
        Err("klu_factor failed".to_string())
    } else {
        Ok(Numeric {
            common: common.as_ffi(),
            ctx: NonNull::new(numeric).unwrap(),
        })
    }
}

pub fn solve(
    symbolic: &mut Symbolic,
    numeric: &mut Numeric,
    ldim: usize,
    nrhs: usize,
    b: &mut [f64],
    common: &mut Common,
) -> Result<(), i32> {
    let rv = unsafe {
        klu_solve(
            symbolic.ctx.as_ptr(),
            numeric.ctx.as_ptr(),
            ldim as i32,
            nrhs as i32,
            b.as_mut_ptr(),
            common.as_ffi(),
        )
    };
    if rv == 0 {
        Err(rv)
    } else {
        Ok(())
    }
}

// pub fn free_symbolic(symbolic: &mut &mut KluSymbolic, common: &mut KluCommon) -> Result<(), i32> {
//     let rv = unsafe { klu_free_symbolic(symbolic, common) };
//     if rv == 0 {
//         Err(rv)
//     } else {
//         Ok(())
//     }
// }
//
// pub fn free_numeric(numeric: &mut &mut KluNumeric, common: &mut KluCommon) -> Result<(), i32> {
//     let rv = unsafe { klu_free_numeric(numeric, common) };
//     if rv == 0 {
//         Err(rv)
//     } else {
//         Ok(())
//     }
// }

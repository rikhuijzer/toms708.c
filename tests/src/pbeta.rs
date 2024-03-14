/*
 *  This Rust file is based on a C file from the R project with the following header:
 *
 *  Mathlib : A C Library of Special Functions
 *  Copyright (C) 2022 The R Core Team
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program; if not, a copy is available at
 *  https://www.R-project.org/Licenses/
 *
 *  SYNOPSIS
 *
 * #include <Rmath.h>
 *
 * double pbeta_raw(double x, double a, double b, int lower_tail, int log_p)
 * double pbeta	   (double x, double a, double b, int lower_tail, int log_p)
 *
 *  DESCRIPTION
 *
 *	Returns distribution function of the beta distribution.
 *	( = The incomplete beta ratio I_x(p,q) ).
 *
 *  NOTES
 *
 *      As from R 2.3.0, a wrapper for TOMS708
 *      as from R 2.6.0, 'log_p' partially improved over log(p..)
 */

use crate::toms708::bratio;
use crate::toms708::r_d__0;
use crate::toms708::r_d__1;

const M_LN2: f64 = std::f64::consts::LN_2;

pub fn r_dt_0(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        unsafe { r_d__0(log_p) }
    } else {
        unsafe { r_d__1(log_p) }
    }
}

pub fn r_dt_1(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        unsafe { r_d__1(log_p) }
    } else {
        unsafe { r_d__0(log_p) }
    }
}

fn pbeta_raw(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    if a == 0.0 || b == 0.0 || !a.is_finite() || !b.is_finite() {
        if a == 0.0 && b == 0.0 {
            return if log_p { -M_LN2 } else { 0.5 };
        }
        if a == 0.0 || a / b == 0.0 {
            return r_dt_1(lower_tail, log_p);
        }
        if b == 0.0 || b / a == 0.0 {
            return r_dt_0(lower_tail, log_p);
        }
        if x < 0.5 {
            return r_dt_0(lower_tail, log_p);
        } else {
            return r_dt_1(lower_tail, log_p);
        }
    }
    if x >= 1.0 {
        return r_dt_1(lower_tail, log_p);
    }

    let x1 = 0.5 - x + 0.5;
    let mut w = 0.0;
    let mut wc = 0.0;
    let mut ierr = 0;
    unsafe {
        bratio(a, b, x, x1, &mut w, &mut wc, &mut ierr, log_p);
    }

    if ierr != 0 && ierr != 11 && ierr != 14 {
        println!("pbeta_raw({x}, a={a}, b={b}, lower_tail={lower_tail}, log_p={log_p}) -> bratio() gave error code{ierr}");
    }
    if lower_tail {
        w
    } else {
        wc
    }
}

pub fn pbeta(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    if x.is_nan() || a.is_nan() || b.is_nan() {
        return x + a + b;
    }
    if a < 0.0 || b < 0.0 {
        return f64::NAN;
    }
    if x <= 0.0 {
        return r_dt_0(lower_tail, log_p);
    }
    pbeta_raw(x, a, b, lower_tail, log_p)
}

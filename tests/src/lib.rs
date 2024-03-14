mod pbeta;

// Export to get rid of a dead code warning.
pub use pbeta::pbeta;

mod toms708 {
    extern "C" {
        pub fn r_d__0(log_p: bool) -> f64;
        pub fn r_d__1(log_p: bool) -> f64;
        pub fn bratio(a: f64, b: f64, x: f64, y: f64, w: *mut f64,
            w1: *mut f64, ierr: *mut i32, log_p: bool) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_abs_diff_eq;

    fn bratio(a: f64, b: f64, x: f64, y: f64, log_p: bool) -> (f64, f64, i32) {
        let mut w = 0.0;
        let mut w1 = 0.0;
        let mut ierr = 0;
        unsafe {
            toms708::bratio(a, b, x, y, &mut w, &mut w1, &mut ierr, log_p);
        }
        (w, w1, ierr)
    }

    #[test]
    // This test is based on `toms708_test.f` subroutine `TEST01`.
    // Test BRATIO computes the Beta ratio function.
    fn subroutine_test01() {
        let a: f64 = 5.3;
        let b: f64 = 10.1;

        for i in 1..=50 {
            let x: f64 = (i as f64) / 100.0;
            let y: f64 = 0.5 + (0.5 - x);
            let log_p = false;

            let (_w, _w1, ierr) = bratio(a, b, x, y, log_p);

            assert!(ierr == 0);
        }
    }

    // Thanks to the `pbeta` definition in `pbeta.rs`, we can compare the
    // outcome of `bratio` against the outcome of `pbeta` in R.
    // Expected values are from R version 4.3.2.
    //
    // `pbeta.rs` is mostly handling edge cases and then calling `bratio`.
    // Cases which end up in `bratio` are where 0 < a < Inf and 0 < b < Inf.
    #[test]
    fn test_pbeta() {
        let lower_tail = true;
        let log_p = false;
        let received = pbeta(0.5, 1.0, 1.0, lower_tail, log_p);
        // R> pbeta(0.5, 1.0, 1.0, lower.tail = TRUE, log.p = FALSE)
        let expected = 0.5;
        assert_eq!(received, expected);

        assert_abs_diff_eq!(
            pbeta(256.0/1024.0, 3.0, 2200.0, false, true),
            // R> sprintf("%.13f", pbeta(256/1024, 3, 2200, lower.tail=FALSE, log.p=TRUE))
            -620.9697808693397,
            epsilon = 1e-13
        );

        assert_abs_diff_eq!(
            pbeta(512.0/1024.0, 3.0, 2200.0, false, true),
            // R> sprintf("%.10f", pbeta(512/1024, 3, 2200, lower.tail=FALSE, log.p=TRUE))
            -1511.6085416972,
            epsilon = 1e-10
        );
    }
}

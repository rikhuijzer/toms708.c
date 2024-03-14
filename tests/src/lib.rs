mod pbeta;

// Export to get rid of a dead code warning.
pub use pbeta::pbeta;

mod toms708 {
    extern "C" {
        pub fn r_d__0(log_p: bool) -> f64;
        pub fn r_d__1(log_p: bool) -> f64;
        pub fn bratio(
            a: f64,
            b: f64,
            x: f64,
            y: f64,
            w: *mut f64,
            w1: *mut f64,
            ierr: *mut i32,
            log_p: bool,
        ) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use approx::assert_abs_diff_eq;
    use statrs::statistics::Statistics;

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

        let epsilon = 1e-12;

        // Based on a test in `d-p-q-r-tst-2.R` from the R source code.
        for x in [0.01, 0.10, 0.25, 0.40, 0.55, 0.71, 0.98] {
            assert_abs_diff_eq!(
                pbeta(x, 0.8, 2.0, false, true),
                pbeta(1.0 - x, 2.0, 0.8, true, true),
                epsilon = epsilon
            )
        }

        // One case from the loop above.
        assert_abs_diff_eq!(
            pbeta(0.10, 0.8, 2.0, false, true),
            // R> sprintf("%.13f", pbeta(0.10, 0.8, 2.0, lower.tail=FALSE, log.p=TRUE))
            -0.3182809860569,
            epsilon = epsilon
        );

        // Based on a test in `d-p-q-r-tst-2.R` from the R source code.
        assert_abs_diff_eq!(
            pbeta(256.0 / 1024.0, 3.0, 2200.0, false, true),
            // R> sprintf("%.13f", pbeta(256/1024, 3, 2200, lower.tail=FALSE, log.p=TRUE))
            -620.9697808693397,
            epsilon = epsilon
        );

        assert_abs_diff_eq!(
            pbeta(512.0 / 1024.0, 3.0, 2200.0, false, true),
            // R> sprintf("%.13f", pbeta(512/1024, 3, 2200, lower.tail=FALSE, log.p=TRUE))
            -1511.6085416971891,
            epsilon = epsilon
        );

        assert_abs_diff_eq!(
            pbeta(768.0 / 1024.0, 3.0, 2200.0, false, true),
            // R> sprintf("%.13f", pbeta(768/1024, 3, 2200, lower.tail=FALSE, log.p=TRUE))
            -3035.7220144978146,
            epsilon = epsilon
        );

        assert!(pbeta(1024.0 / 1024.0, 3.0, 2200.0, false, true).is_infinite());

        fn diff(x: Vec<f64>) -> Vec<f64> {
            let mut result = Vec::new();
            let n = x.len();
            for i in 0..n - 1 {
                result.push(x[i + 1] - x[i]);
            }
            result
        }

        fn log(x: Vec<f64>) -> Vec<f64> {
            x.iter().map(|x| x.ln()).collect()
        }

        // Based on a test in `d-p-q-r-tst-2.R` at line 330 from the R source code.
        // pbeta(x, a, b, log=TRUE) for small x and a is ~ log-linear.
        let x = (10..=200).map(|n| 2.0_f64.powf(-n as f64));
        for a in [1e-8, 1e-12, 16e-16, 4e-16] {
            for b in [0.6, 1.0, 2.0, 10.0] {
                let xs = x.clone().map(|x| pbeta(x, a, b, true, true));
                let dp = diff(xs.collect());
                let sd = dp.clone().population_std_dev();
                let m = dp.mean();
                assert!(sd / m < 0.0007);
            }
        }

        // Based on a test in `d-p-q-r-tst-2.R` at line 401 from the R source code.
        // pbeta(x, <small a>,<small b>, .., log):
        let a = (1..=25).map(|i| 2_f64.powi(-(90 + i)));
        let b = 2_f64.powi(-60);
        let pb = a.clone().map(|a| pbeta(0.5, a, b, true, true));
        let ldp = diff(log(diff(pb.collect())));
        println!("{:?}", ldp);
        for l in ldp {
            let val = l - (1_f64 / 2_f64).ln();
            assert!(val.abs() < 1e-9)
        }
    }
}

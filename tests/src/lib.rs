#[cfg(test)]
mod tests {
    mod toms708 {
        extern "C" {
            pub fn bratio(a: f64, b: f64, x: f64, y: f64, w: *mut f64,
                w1: *mut f64, ierr: *mut i32, log_p: bool) -> i32;
        }
    }

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
    // This is `subroutine test02` from `toms708_test.f`.
    fn test_bratio() {
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
}

use std::f64::consts::PI;

use num_complex::Complex;

const TAU: f64 = 2.0 * PI;

pub fn batman_fn(t: f64) -> Complex<f64> {
    let t = t * 16.0 - 8.0;
    let t_abs = t.abs();
    let x = if t != 0.0 {
        (t_abs / t)
            * (0.3 * t_abs + 0.2 * (t_abs - 1.0).abs() + 2.2 * (t_abs - 2.0).abs()
                - 2.7 * (t_abs - 3.0).abs()
                - 3.0 * (t_abs - 5.0).abs()
                + 3.0 * (t_abs - 7.0).abs()
                + 5.0 * ((PI / 4.0) * ((t_abs - 3.0).abs() - (t_abs - 4.0).abs() + 1.0)).sin()
                + 1.25 * ((t_abs - 4.0).abs() - (t_abs - 5.0).abs() - 1.0).powi(3)
                - 5.3
                    * (((PI / 2.0) + (47.0_f64 / 53.0).asin())
                        * (((t_abs - 7.0).abs() - (t_abs - 8.0).abs() - 1.0) / 2.0))
                        .cos()
                + 2.8)
    } else {
        0.0
    };

    let y = (3.0 / 2.0) * (t_abs - 1.0).abs()
        - (3.0 / 2.0) * (t_abs - 2.0).abs()
        - (29.0 / 4.0) * (t_abs - 4.0).abs()
        + (29.0 / 4.0) * (t_abs - 5.0).abs()
        + (7.0 / 16.0) * ((t_abs - 2.0).abs() - (t_abs - 3.0).abs() - 1.0).powi(4)
        + 4.5 * ((PI / 4.0) * ((t_abs - 3.0).abs() - (t_abs - 4.0).abs() - 1.0)).sin()
        - (3.0 * (2.0_f64).sqrt() / 5.0)
            * ((t_abs - 5.0).abs() - (t_abs - 7.0).abs()).abs().powf(2.5)
        + 6.4
            * (((PI / 2.0) + (47.0_f64 / 53.0).asin())
                * ((t_abs - 7.0).abs() - (t_abs - 8.0).abs() + 1.0)
                / 2.0
                + (56.0_f64 / 64.0).asin())
            .sin()
        + 4.95;

    Complex::new(x, y)
}

pub fn heart_fn(t: f64) -> Complex<f64> {
    let t = t * TAU;
    Complex::new(
        16.0 * t.sin().powi(3),
        13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
    )
}

pub fn star_fn(t: f64) -> Complex<f64> {
    let t = t * TAU;
    let c = 0.7;
    let s = 0.15;
    let m = 1.0 - c * ((2.5 * t).cos().powi(2)).powf(s);
    Complex::new(m * t.cos(), m * t.sin())
}

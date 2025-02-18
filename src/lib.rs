use std::f64::consts::PI;

use eframe::egui::Color32;
use egui_plot::{Arrows, Line, LineStyle, PlotPoints};
use num_complex::Complex;

pub mod app;
pub mod functions;

const TAU: f64 = 2.0 * PI;

fn compute_fourier_coeffs(f: fn(f64) -> Complex<f64>, n: usize) -> Vec<Complex<f64>> {
    let n = n as i32;
    let mut coeffs = vec![];
    for k in -n..=n {
        // loop over all harmonics
        let mut coeff = Complex::new(0.0, 0.0);

        let a = 1000;
        let dt = 1.0 / a as f64;
        for i in 0..a {
            // approximate integral from 0 to 1 of f(t) * e^(-2 * pi * i * k * t)
            let t = i as f64 * dt;
            coeff += f(t) * Complex::from_polar(1.0, -TAU * k as f64 * t);
        }
        coeff *= dt;

        coeffs.push(coeff);
    }
    coeffs
}

fn input_function_line<'a>(f: fn(f64) -> Complex<f64>, n: usize) -> Line<'a> {
    let dn = 1.0 / n as f64;
    let points: PlotPoints = (0..=n)
        .map(|i| {
            let t = i as f64 * dn;
            let f = f(t);
            [f.re, f.im]
        })
        .collect();
    Line::new(points)
        .color(Color32::BLUE)
        .width(1.0)
        .style(LineStyle::dashed_loose())
}

fn fourier_function_line<'a>(coeffs: &Vec<Complex<f64>>, n: usize) -> Line<'a> {
    let dn = 1.0 / n as f64;
    let num_coeffs = (coeffs.len() - 1) / 2;
    let points: PlotPoints = (0..=n)
        .map(|i| {
            let t = i as f64 * dn;
            let mut f = Complex::new(0.0, 0.0);
            for (k, coeff) in coeffs.iter().enumerate() {
                f += coeff
                    * Complex::from_polar(1.0, TAU * (k as i32 - num_coeffs as i32) as f64 * t);
            }
            [f.re, f.im]
        })
        .collect();
    Line::new(points).color(Color32::GREEN).width(3.0)
}

fn animate_fourier_function<'a>(
    points: &mut Vec<[f64; 2]>,
    coeffs: &Vec<Complex<f64>>,
    t: f64,
) -> Line<'a> {
    let num_coeffs = (coeffs.len() - 1) / 2;
    let mut f = Complex::new(0.0, 0.0);
    for (k, coeff) in coeffs.iter().enumerate() {
        f += coeff * Complex::from_polar(1.0, TAU * (k as i32 - num_coeffs as i32) as f64 * t);
    }
    points.push([f.re, f.im]);
    Line::new(PlotPoints::new(points.clone()))
        .color(Color32::RED)
        .width(3.0)
}

fn vectors<'a>(coeffs: &Vec<Complex<f64>>, t: f64) -> Arrows<'a> {
    let c = ((coeffs.len() - 1) / 2) as i32;
    let first_arrow = coeffs[c as usize];
    let mut arrows = vec![[0.0, 0.0], [first_arrow.re, first_arrow.im]];
    for k in 1..=c {
        let previous_tip = arrows.last().unwrap();

        let v1 = coeffs[(c + k) as usize] * Complex::from_polar(1.0, TAU * k as f64 * t);
        let v2 = coeffs[(c - k) as usize] * Complex::from_polar(1.0, TAU * -k as f64 * t);

        let v1_tip = [previous_tip[0] + v1.re, previous_tip[1] + v1.im];
        let v2_tip = [v1_tip[0] + v2.re, v1_tip[1] + v2.im];

        arrows.push(v1_tip);
        arrows.push(v2_tip);
    }
    let origins = arrows[0..arrows.len() - 1].to_vec();
    let tips = arrows[1..].to_vec();
    Arrows::new(origins, tips).tip_length(3.0)
}

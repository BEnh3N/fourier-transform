use std::f64::consts::PI;

use eframe::egui::{self, Key};
use egui_plot::Plot;
use num_complex::Complex;

use crate::{animate_fourier_function, compute_fourier_coeffs, vectors};

pub fn create_app(num_coeffs: i32, function: fn(f64) -> Complex<f64>) {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Fourier Transform",
        native_options,
        Box::new(|cc| Ok(Box::new(AppState::new(cc, num_coeffs, function)))),
    )
    .unwrap();
}

struct AppState {
    num_coeffs: i32,
    time: f64,
    function: fn(f64) -> Complex<f64>,
    coeffs: Vec<Complex<f64>>,
    points: Vec<[f64; 2]>,
}

impl AppState {
    fn new(
        _cc: &eframe::CreationContext<'_>,
        num_coeffs: i32,
        function: fn(f64) -> Complex<f64>,
    ) -> AppState {
        // let num_coeffs = 10;
        // let function = batman_fn;

        let coeffs = compute_fourier_coeffs(function, num_coeffs as usize);
        let points = vec![];

        AppState {
            num_coeffs,
            time: 0.0,
            function,
            coeffs,
            points,
        }
    }
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.monospace(self.time.to_string());

            let t: f64 = self.time / 10.0;

            // let input_function = input_function_line(self.function, 1024);
            // let fourier_function = fourier_function_line(&self.coeffs, 1024);
            let fourier_animation = animate_fourier_function(&mut self.points, &self.coeffs, t);

            let vectors = vectors(&self.coeffs, t);

            let plot = Plot::new("my_plot").data_aspect(1.0).show_axes(false);
            plot.show(ui, |plot_ui| {
                // plot_ui.line(input_function);
                // plot_ui.line(fourier_function);
                plot_ui.line(fourier_animation);

                plot_ui.arrows(vectors);
            });

            // animation
            ctx.request_repaint();
            self.time += ui.input(|i| i.stable_dt) as f64;
            if self.time > 10.0 {
                self.time = 0.0;
                self.points.clear();
            }

            if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                self.num_coeffs += 1;
                self.coeffs = compute_fourier_coeffs(self.function, self.num_coeffs as usize);
                self.points = (0..(t * 1000.0) as i32)
                    .map(|i| {
                        let t = i as f64 * 0.01;
                        let mut f = Complex::new(0.0, 0.0);
                        for (k, coeff) in self.coeffs.iter().enumerate() {
                            f += coeff
                                * Complex::from_polar(
                                    1.0,
                                    2.0 * PI * (k as i32 - self.num_coeffs) as f64 * (t / 10.),
                                );
                        }
                        [f.re, f.im]
                    })
                    .collect();
                println!("num_coeffs: {}", self.num_coeffs);
            }
            if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                self.num_coeffs -= 1;
                self.coeffs = compute_fourier_coeffs(self.function, self.num_coeffs as usize);
                self.points = (0..(t * 1000.0) as i32)
                    .map(|i| {
                        let t = i as f64 * 0.01;
                        let mut f = Complex::new(0.0, 0.0);
                        for (k, coeff) in self.coeffs.iter().enumerate() {
                            f += coeff
                                * Complex::from_polar(
                                    1.0,
                                    2.0 * PI * (k as i32 - self.num_coeffs) as f64 * (t / 10.),
                                );
                        }
                        [f.re, f.im]
                    })
                    .collect();
                println!("num_coeffs: {}", self.num_coeffs);
            }
        });
    }
}

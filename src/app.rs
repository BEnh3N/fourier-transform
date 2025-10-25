use eframe::egui::{self, Key};
use egui_plot::Plot;
use num_complex::Complex;

use crate::{
    animate_fourier_function, compute_fourier_coeffs, fourier_function_line, generate_points,
    input_function_line, vectors,
};

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
            let t: f64 = self.time;
            ui.monospace(format!("{:.05}", t));

            let input_function = input_function_line(self.function, 1024);
            let fourier_function = fourier_function_line(&self.coeffs, 1024);
            let fourier_animation = animate_fourier_function(&mut self.points, &self.coeffs, t);

            let vectors = vectors(&self.coeffs, t);

            Plot::new("fourier_plot")
                .data_aspect(1.0)
                .show_axes(false)
                .show(ui, |plot_ui| {
                    plot_ui.line(input_function);
                    plot_ui.line(fourier_function);
                    plot_ui.line(fourier_animation);

                    plot_ui.arrows(vectors);
                });

            // animation
            ctx.request_repaint();
            if self.time > 1.0 {
                self.time = 0.0;
                self.points.clear();
            }

            let num_coeffs = self.num_coeffs;
            if ui.input(|i| i.key_pressed(Key::ArrowUp)) {
                self.num_coeffs += 1;
            }
            if ui.input(|i| i.key_pressed(Key::ArrowDown)) {
                self.num_coeffs -= 1;
            }
            if self.num_coeffs != num_coeffs {
                self.coeffs = compute_fourier_coeffs(self.function, self.num_coeffs as usize);
                self.points = generate_points(t, self.num_coeffs, &self.coeffs);
                println!("num_coeffs: {}", self.num_coeffs);
            }

            self.time += ui.input(|i| i.stable_dt) as f64 * 0.1;
        });
    }
}

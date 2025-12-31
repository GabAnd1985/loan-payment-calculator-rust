
use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Loan Payment Calculator",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}

#[derive(Default)]
struct App {
    tna: String,
    plazo: String,
    monto: String,
    resultado: Option<f64>,
    error: Option<String>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calculadora de cuota (Sistema Francés)");
            ui.add_space(10.0);

            ui.label("TNA (%)");
            ui.text_edit_singleline(&mut self.tna);

            ui.label("Plazo (meses)");
            ui.text_edit_singleline(&mut self.plazo);

            ui.label("Monto del préstamo");
            ui.text_edit_singleline(&mut self.monto);

            ui.add_space(10.0);

            if ui.button("Calcular").clicked() {
                self.error = None;
                self.resultado = None;

                // Parse + validaciones
                let tna = match parse_f64(&self.tna) {
                    Ok(v) if v >= 0.0 => v,
                    Ok(_) => {
                        self.error = Some("La TNA debe ser mayor o igual a cero.".to_string());
                        return;
                    }
                    Err(e) => {
                        self.error = Some(format!("TNA inválida: {e}"));
                        return;
                    }
                };

                let plazo = match self.plazo.trim().parse::<i32>() {
                    Ok(v) if v > 0 => v,
                    Ok(_) => {
                        self.error = Some("El plazo debe ser mayor a cero.".to_string());
                        return;
                    }
                    Err(_) => {
                        self.error = Some("Plazo inválido. Ingrese un entero.".to_string());
                        return;
                    }
                };

                let monto = match parse_f64(&self.monto) {
                    Ok(v) if v > 0.0 => v,
                    Ok(_) => {
                        self.error = Some("El monto debe ser mayor a cero.".to_string());
                        return;
                    }
                    Err(e) => {
                        self.error = Some(format!("Monto inválido: {e}"));
                        return;
                    }
                };

                let cuota = monto_cuota(tna, plazo, monto);
                self.resultado = Some(cuota);
            }

            if ui.button("Limpiar").clicked() {
                self.tna.clear();
                self.plazo.clear();
                self.monto.clear();
                self.resultado = None;
                self.error = None;
            }

            ui.add_space(10.0);

            if let Some(err) = &self.error {
                ui.colored_label(egui::Color32::RED, err);
            }

            if let Some(cuota) = self.resultado {
                ui.separator();
                ui.heading(format!("Cuota mensual: {:.2}", cuota));
            }

            ui.add_space(6.0);
            ui.small("Tip: podés usar coma o punto para decimales.");
        });
    }
}

// --- Tu lógica (reusada) ---
fn monto_cuota(tna: f64, plazo: i32, monto: f64) -> f64 {
    let r = tna / 1200.0;

    if r == 0.0 {
        return monto / plazo as f64;
    }

    let pow = (1.0 + r).powi(plazo);
    let num = pow * r;
    let den = pow - 1.0;

    monto * (num / den)
}

// --- Helpers ---
fn parse_f64(s: &str) -> Result<f64, &'static str> {
    let cleaned = s.trim().replace(',', ".");
    cleaned.parse::<f64>().map_err(|_| "Ingrese un número válido")
}
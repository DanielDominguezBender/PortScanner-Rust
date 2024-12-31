// Código de la ha herramienta de escaneo de redes en Rust

use eframe::egui;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

fn main() -> eframe::Result<()> {
    // Inicia la aplicación GUI
    eframe::run_native(
        "Escáner de Puertos - Rust",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(PortScannerApp::default())),
    )
}

struct PortScannerApp {
    target: String,          // Dirección IP o dominio
    start_port: String,      // Puerto inicial (como String para la interfaz)
    end_port: String,        // Puerto final
    results: String,         // Resultados del escaneo
}

impl Default for PortScannerApp {
    fn default() -> Self {
        Self {
            target: "127.0.0.1".to_string(),
            start_port: "1".to_string(),
            end_port: "1024".to_string(),
            results: "".to_string(),
        }
    }
}

impl eframe::App for PortScannerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Escáner de Puertos en Rust");
            ui.separator();

            // Campo de entrada para el objetivo
            ui.label("Dirección IP o dominio:");
            ui.text_edit_singleline(&mut self.target);

            // Campos para el rango de puertos
            ui.label("Puerto inicial:");
            ui.text_edit_singleline(&mut self.start_port);

            ui.label("Puerto final:");
            ui.text_edit_singleline(&mut self.end_port);

            // Botón para ejecutar el escaneo
            if ui.button("Iniciar Escaneo").clicked() {
                self.results = scan_ports(
                    &self.target,
                    self.start_port.parse().unwrap_or(1),
                    self.end_port.parse().unwrap_or(1024),
                );
            }

            // Mostrar resultados
            ui.separator();
            ui.label("Resultados:");
            ui.text_edit_multiline(&mut self.results);
        });
    }
}

// Función que realiza el escaneo de puertos
fn scan_ports(target: &str, start: u16, end: u16) -> String {
    let rt = Runtime::new().unwrap();
    let mut results = String::new();

    rt.block_on(async {
        for port in start..=end {
            let address = format!("{}:{}", target, port);
            if is_port_open(&address).await {
                results.push_str(&format!("Puerto {} está abierto\n", port));
            }
        }
    });

    if results.is_empty() {
        results.push_str("No se encontraron puertos abiertos.");
    }

    results
}

// Comprueba si un puerto está abierto
async fn is_port_open(address: &str) -> bool {
    let socket: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    match TcpStream::connect(socket).await {
        Ok(_) => true, // Puerto abierto
        Err(_) => false, // Puerto cerrado
    }
}

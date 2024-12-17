# PortScanner-Rust

## **Introducción**

La idea con éste proyecto era vercómo poder usar de manera más real el uso de Rust, o sea, de que manera se pueden implementar / crear herramientas usando éste lenguaje de programación, más allá del típico ejercicio de ``suma me dos dígitos y que la funcion devuelva un entero```.

En éste ejemplo, he querido crear un escáner de puertos similar al ```nmap``` pero que también incluya un simple front-end.

A continuación desgloso el programa por partes.

## **1. Dependencias y `Cargo.toml`**

Primero, añadimo las librerías que necesitamos:

- **`tokio`**: Proporciona un runtime asíncrono, ideal para manejar múltiples conexiones de red sin bloquear el programa.
- **`eframe`**: Un framework basado en **`egui`** que nos permite crear una interfaz gráfica fácil de usar.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
eframe = "0.23"
```

---

## **2. La función `main`**

Esta es la función principal que inicia la aplicación GUI.

```rust
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Escáner de Puertos - Rust",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(PortScannerApp::default())),
    )
}
```

- **`eframe::run_native`**: Llama a la función principal de `eframe` para iniciar la interfaz gráfica.
- **"Escáner de Puertos - Rust"**: Es el título de la ventana de la aplicación.
- **`PortScannerApp::default()`**: Crea una instancia de nuestra estructura `PortScannerApp` que maneja el estado y la lógica de la GUI.

---

## **3. Estructura `PortScannerApp`**

Esta estructura almacena el **estado de la aplicación**, es decir, los datos que el usuario introduce y los resultados del escaneo.

```rust
struct PortScannerApp {
    target: String,          // IP o dominio
    start_port: String,      // Puerto inicial (formato String por la interfaz)
    end_port: String,        // Puerto final
    results: String,         // Resultados del escaneo
}
```

Aquí:

- **`target`**: Dirección IP o dominio introducido por el usuario.
- **`start_port`** y **`end_port`**: Rango de puertos introducido.
- **`results`**: Cadena donde se almacenan y muestran los resultados del escaneo.

---

## **4. Implementación por defecto**

Cuando la aplicación inicia, `PortScannerApp::default()` define valores iniciales para los campos.

```rust
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
```

- **Valores predeterminados**:
    - IP: `127.0.0.1` (localhost).
    - Puertos: Del `1` al `1024`.

---

## **5. Interfaz gráfica con `update`**

La lógica de la interfaz se encuentra en el método `update`.

```rust
impl eframe::App for PortScannerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Escáner de Puertos en Rust");
            ui.separator();

            // Entrada de IP o dominio
            ui.label("Dirección IP o dominio:");
            ui.text_edit_singleline(&mut self.target);

            // Entrada de rango de puertos
            ui.label("Puerto inicial:");
            ui.text_edit_singleline(&mut self.start_port);
            ui.label("Puerto final:");
            ui.text_edit_singleline(&mut self.end_port);

            // Botón para iniciar el escaneo
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
```

## **Explicación línea por línea:**

1. **`egui::CentralPanel::default()`**:
    
    - Define el área central de la ventana donde se colocarán los elementos.
2. **Título y separador**:
    
    - `ui.heading`: Muestra el título de la aplicación.
    - `ui.separator`: Dibuja una línea divisoria.
3. **Entrada de texto**:
    
    - **`ui.label`**: Añade un texto descriptivo.
    - **`ui.text_edit_singleline`**: Campo de texto de una línea para el usuario (IP y puertos).
4. **Botón "Iniciar Escaneo"**:
    
    - **`if ui.button("Iniciar Escaneo").clicked()`**: Detecta si el botón ha sido presionado.
    - **`self.results = scan_ports(...)`**: Llama a la función `scan_ports` y guarda los resultados en `self.results`.
5. **Mostrar resultados**:
    
    - **`ui.text_edit_multiline`**: Área de texto multilínea para mostrar los puertos abiertos.

---

## **6. La función `scan_ports`**

Esta función realiza el escaneo de puertos.

```rust
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
```

## **Desglose:**

1. **`Runtime::new()`**: Crea un runtime asíncrono de Tokio.
2. **Bucle `for port in start..=end`**:
    - Itera desde el puerto inicial hasta el puerto final.
    - Forma la dirección `target:port` y verifica si está abierta con `is_port_open`.
3. **`is_port_open`**: Comprueba si una conexión `TCP` al puerto es exitosa.
4. **Resultados**:
    - Si el puerto está abierto, se agrega a `results`.
    - Si no hay puertos abiertos, devuelve un mensaje predeterminado.

---

## **7. Comprobación de puertos con `is_port_open`**

Esta función intenta conectar a un puerto específico.

```rust
async fn is_port_open(address: &str) -> bool {
    let socket: SocketAddr = match address.parse() {
        Ok(addr) => addr,
        Err(_) => return false,
    };

    match TcpStream::connect(socket).await {
        Ok(_) => true,  // Puerto abierto
        Err(_) => false, // Puerto cerrado
    }
}
```

- **`TcpStream::connect`**: Intenta establecer una conexión `TCP`.
    - Si tiene éxito → puerto abierto.
    - Si falla → puerto cerrado.

---

## **Resumen**

1. **Interfaz gráfica**:
    - Permite introducir el objetivo (IP/dominio) y el rango de puertos.
    - Botón para iniciar el escaneo.
2. **Lógica del escaneo**:
    - Ejecuta un bucle asíncrono para comprobar los puertos dentro del rango.
    - Muestra los puertos abiertos.
3. **Uso de Tokio**:
    - Permite manejar conexiones de red sin bloquear la aplicación.

---

## **Ejecución del programa en Rust**

1. En un terminal, ejecutar el siguiente comando:
```bash
cargo run
```

Aparecerá una ventana interactiva donde poder probar escanear de puertos con un **front-end** simple pero efectivo.

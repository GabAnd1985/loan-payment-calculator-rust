
use std::io::{self, Write};

struct Prestamo {
    tasa: f64,
    plazo: i32,
    monto: f64,
}


impl Prestamo {
    fn cuota_sistema_frances(&self) -> f64 {
        if self.tasa== 0.0{
            return self.monto / self.plazo as f64;
        }

    let num= ((1.0 + self.tasa/1200.0).powi(self.plazo)) * (self.tasa/1200.0);

    let den= (1.0 + self.tasa/1200.0).powi(self.plazo) - 1.0;

    self.monto * (num / den)

    }

}


fn main() {

    loop {

        let prestamo = Prestamo {
            tasa: input_usuario_tasa(),
            plazo: input_usuario_plazo(),
            monto: input_usuario_deuda(),
        };

        let cuota= prestamo.cuota_sistema_frances();

        println!("\nMonto de cuota: {:.2} Unidades Valor mensuales", cuota);

        match preguntar_continuar() {
            Decision::Continuar => continue,
            Decision::Salir => break,
        }
    }
    
    pausa()
    
}

//Consultamos si debemos continuar

enum Decision {
    Continuar,
    Salir,
}


fn preguntar_continuar() -> Decision {
    loop {
        let mut s = String::new();
        print!("\n¿Desea calcular otra cuota? (s/n): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut s).unwrap();
        match s.trim().to_lowercase().as_str() {
            "s" | "si" | "sí" => return Decision::Continuar,
            "n" | "no" => return Decision::Salir,
            _ => println!("Respuesta inválida. Ingrese 's' o 'n'."),
        }
    }
}

//Pedimos input de usuario para cerrar la ventana

fn pausa() {
    println!("\nPresione Enter para salir...");
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);   
}


// Toma input de tasa de un usuario y lo convierte a decimal

fn input_usuario_tasa() -> f64 {

    loop {

        let mut num = String::new();

        print!("\nIngrese la TNA (%): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().replace(",", ".").parse::<f64>() {
            Ok(valor) => {
                if valor >= 0.0 {
                    return valor;
                } else {
                    println!("\nLa tasa debe ser mayor o igual a cero")
                }
            }
            Err(_) => println!("\nEntrada inválida tasa. Intente nuevamente."),
        }
    }
}

// Toma input de plazo meses de un usuario y lo convierte a entero

fn input_usuario_plazo() -> i32 {

    loop {

        let mut num = String::new();

        print!("\nIngrese el plazo en meses: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().parse::<i32>() {
            Ok(valor) => {
                if valor > 0 {
                    return valor;
                } else {
                    println!("\nEl plazo debe ser mayor a cero.");
                }
            }
            Err(_) => println!("\nEntrada inválida. Ingrese un número entero."),
        }
    }
}

// Toma input de Deuda de un usuario y lo convierte a decimal

fn input_usuario_deuda() -> f64 {

    loop {

        let mut num = String::new();

        print!("\nIngrese el monto del préstamo: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().replace(",", ".").parse::<f64>() {
            Ok(valor) => {
                if valor > 0.0 {
                    return valor;
                } else {
                    println!{"\nEl monto del préstamo debe ser mayor a cero."}
                }
            }
            Err(_) => println!("\nEntrada inválida monto préstamo. Intente nuevamente."),
        }
    }
}


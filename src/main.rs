
use std::io::{self, Write};

fn main() {

    loop {

        let i= input_usuario_tasa();

        let n= input_usuario_plazo();

        let p= input_usuario_deuda();

        let cuota= monto_cuota(i, n, p);

        println!("\nMonto de cuota: {:.2} Unidades Valor mensuales", cuota);

        if preguntar_continuar()== false {
            break;
        }
    }
    
    pausa()
    
}

//Consultamos si debemos continuar

fn preguntar_continuar() -> bool {
    loop {
        let mut s = String::new();
        print!("\n¿Desea calcular otra cuota? (s/n): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut s).unwrap();
        match s.trim().to_lowercase().as_str() {
            "s" | "si" | "sí" => return true,
            "n" | "no" => return false,
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

//Calcula el monto de la cuota e imprime el valor

fn monto_cuota(tna: f64, plazo: i32, monto: f64) -> f64 {

    if tna== 0.0 {
        return monto / plazo as f64;
    } 

    let num= ((1.0 + tna/1200.0).powi(plazo)) * (tna/1200.0);

    let den= (1.0 + tna/1200.0).powi(plazo) - 1.0;

    monto * (num / den)

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
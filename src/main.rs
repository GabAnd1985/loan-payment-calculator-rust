
use std::io::{self, Write};

fn main() {

    let i= input_usuario_tasa();

    let n= input_usuario_plazo();

    let p= input_usuario_deuda();

    monto_cuota(i, n, p);

    pausa()

}

//Pedimos input de usuario para cerrar la ventana
// Ask for user input to close the window

fn pausa() {
    println!("\nPresione Enter para salir...");
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);   
}

//Calcula el monto de la cuota e imprime el valor
// Calculates the loan payment amount and prints the result

fn monto_cuota(i: f64, n: i32, p: f64) {

    let num= ((1.0 + i/1200.0).powi(n)) * (i/1200.0);

    let den= (1.0 + i/1200.0).powi(n) - 1.0;

    let cuota= p * (num / den);

    println!("\nMonto de cuota: {:.2} Unidades Valor mensuales", cuota);

}

// Toma input de tasa de un usuario y lo convierte a decimal
// Gets the interest rate from the user and converts it to a decimal

fn input_usuario_tasa() -> f64 {

    loop {

        let mut num = String::new();

        print!("\nIngrese la tasa de interes (%): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().replace(",", ".").parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("\nEntrada inválida tasa. Intente nuevamente."),
        }
    }
}

// Toma input de plazo meses de un usuario y lo convierte a entero
// Reads the loan term (months) from the user and converts it to an integer

fn input_usuario_plazo() -> i32 {

    loop {

        let mut num = String::new();

        print!("\nIngrese el plazo en meses: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().parse::<i32>() {
            Ok(valor) => return valor,
            Err(_) => println!("\nEntrada inválida plazo. Intente nuevamente."),
        }
    }
}

// Toma input de Deuda de un usuario y lo convierte a decimal
// Reads the user's loan amount and parses it as a decimal value

fn input_usuario_deuda() -> f64 {

    loop {

        let mut num = String::new();

        print!("\nIngrese el monto del préstamo: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut num).unwrap();

        match num.trim().replace(",", ".").parse::<f64>() {
            Ok(valor) => return valor,
            Err(_) => println!("\nEntrada inválida monto préstamo. Intente nuevamente."),
        }
    }
}
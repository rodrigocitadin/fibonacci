use std::io;

fn main() {
    loop {
        println!("\nDigite um inteiro maior que zero\n");

        let mut n: String = String::new();

        io::stdin().read_line(&mut n).expect("Falha ao ler entrada");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Valor invalido!");
                continue;
            }
        };

        let result: u64 = fibonacci(n);

        println!("\nO {}° termo da sequência de fibonacci é {}", n, result);
    }
}

fn fibonacci(n: u64) -> u64 {
    if n == 1 {
        0
    } else {
        if n == 2 {
            1
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}

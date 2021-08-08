use fatorial::factorial;

fn main() {
    let valores = [5, 6, 9, 10];

    for valor in valores {
        println!("{}! = {}", valor, factorial(valor));
    }
}
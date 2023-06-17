use std::io::Write;
use std::str::FromStr;
use crate::r::gcd;
mod r;



fn main() {
    println!("{}", gcd(14, 15));
    println!("Hello, world!");
    let mut numbers = Vec::new();
    
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        numbers.push(u64::from_str(&arg)
            .expect("Ошибка получения числа"));
    }
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Невалидное число gcd ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m );
    }

    println!("Наибольший общий делитель массива {:?} : {}", &numbers, d);
}



#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
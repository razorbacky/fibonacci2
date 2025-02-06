// AstroNvim 테스트

use std::io;

fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        let temp = b;
        b += a;
        a = temp;
    }
    a
}

fn main() {
    println!("계산할 순서를 입력하세요.");

    let mut int_order = String::new();

    io::stdin().read_line(&mut int_order).expect("Err");

    let int_order: i32 = match int_order.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println! {"잘못된 순서를 입력하여 종료합니다."}
            return;
        }
    };

    for i in 0..int_order {
        println!("순서({i}) = {}", fibonacci(i));
    }
}

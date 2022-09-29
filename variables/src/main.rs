use std::io;

fn main() {
    let mut fahrenheit = String::new();
    println!("華氏の温度を入力してください。");

    io::stdin().read_line(&mut fahrenheit).expect("Error!");

    let fahrenheit: f32 = fahrenheit
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!(
        "華氏{}°は摂氏{}℃",
        fahrenheit,
        fahrenheit_to_celsius(fahrenheit)
    );

    // -----------------------------------------

    'hoge: loop {
        println!("フィボナッチ数列の何番目を出力しますか？");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Error!");

        let n: i32 = n.trim().parse().expect("Index entered was not a number");

        println!("フィボナッチ数列の{}番目は{}です。", n, fibonatch(n));
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    // (32°F - 32) × 5 ÷ 9 = 0°C
    (f - 32.0) * 5.0 / 9.0
}

fn fibonatch(n: i32) -> i32 {
    // フィボナッチ数列のn番目
    if n <= 2 {
        1
    } else {
        fibonatch(n - 1) + fibonatch(n - 2)
    }
}

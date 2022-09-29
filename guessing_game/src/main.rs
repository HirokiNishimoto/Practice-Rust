use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数当てゲームです。");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("予想を入力してください。");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal => {
                println!("正解です！");
                break;
            }
        }
    }
}

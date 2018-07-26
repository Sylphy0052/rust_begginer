// 数当てゲーム

// crate(クレート)Cargo.tomlで定義
extern crate rand;

// io標準ライブラリから読み込む
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // マクロ
    println!("Guess the number!");

    // 乱数生成器のコピー
    // 1~100の数字を生成
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // For the test
    // println!("The secret number is: {}", secret_number);

    // 無限ループ
    loop {
        println!("Please input your guess.");

        // 束縛変数(let)
        // let mutは可変変数(mutable)
        // letは不変変数(imutable)
        //String::new()はStringのスタティックメソッド
        let mut guess = String::new();

        // useしていなければstd::io::stdin...とも書ける
        // &は参照
        // メソッドを呼び出す時改行,スペースを入れてもいい
        // expectはエラーハンドリング
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // 同じ変数名(Shadowing)
        // trimで改行や空白を除去
        // parseで変換
        // matchで返り値のエラーハンドリング
        // Okなら成功, Errなら失敗
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };

        println!("You guessed: {}", guess);

        // 比較
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            },
        }
    }
}

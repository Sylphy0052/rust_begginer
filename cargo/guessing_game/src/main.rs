// 数当てゲーム

// io標準ライブラリから読み込む
use std::io;

fn main() {
    println!("Guess the number!");
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

    println!("You guessed: {}", guess);
}

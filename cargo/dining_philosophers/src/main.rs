use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

// 構造体
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

// 構造体の定義
// new: Associated function(関連関数)
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    // lockの解放はスコープから抜ける時自動で行われる
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

// Mutex: 1つのスレッドしか同時にアクセスできない
// 値を使わないため中身は空タプル
struct Table {
    forks: Vec<Mutex<()>>,
}

fn main() {

    // Arc:atomic reference count(アトミック参照カウント):複数スレッドから共有されるときに使用
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        ]});

    let philosophers = vec![
        Philosopher::new("Judith Bulter", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    // handles: threadを制御する
    // _: PlaceHolder -> 型推論しろということ
    // into_iter: vecの所有権を持つイテレータを生成
    // map: mapを呼び出す．それを引数とし，要素ごとに順番に呼ばれるクロージャを渡す
    // thread::spawn: 引数を取り，クロージャを新しいスレッドで実行する
    // move: 所有権をクロージャに渡す
    // thread::spawnでは末尾に;がない -> これは式であるということを示唆する
    // collect: Vec<>型のコレクション型を生成する
    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    // join: スレッドが完了するまでブロックする
    for h in handles {
        h.join().unwrap();
    }
}

// 独自のマクロを定義
macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    // マクロを使用
    say_hello!(); // "Hello, world!" と表示される
    say_hello!("Alice"); // "Hello, Alice!" と表示される
}


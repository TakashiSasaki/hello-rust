fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1の所有権がs2に移動する
    // println!("{}, world!", s1); // エラー: s1の値はもはや有効ではない

    let s3 = &s2; // s2の借用
    println!("{}, world!", s3); // 成功: s3はs2の参照を借用している
}


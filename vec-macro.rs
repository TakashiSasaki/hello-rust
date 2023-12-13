fn main() {
    // 整数のベクタを作成
    let numbers = vec![1, 2, 3, 4, 5];
    println!("numbers: {:?}", numbers);

    // 各要素を2倍する
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
}


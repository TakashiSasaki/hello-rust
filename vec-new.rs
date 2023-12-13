fn main() {
    // 整数のベクタを手動で作成
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5);
    println!("numbers: {:?}", numbers);

    // 各要素を2倍する
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
}


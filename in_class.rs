pub fn run() {
    let numbers = vec![1, 2, 3];

    let doubled = numbers.clone().into_iter().map(|x| x * 2).collect::<Vec<i32>>();

    let replaced = numbers.into_iter().map(|x| {
        if x > 2 { 0 } else { x }
    }).collect::<Vec<i32>>();

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}
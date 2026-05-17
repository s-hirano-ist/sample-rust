fn main() {
    println!("1+1");
    let char_a = "a";
    let string_b = "b".to_string(); // TODO: String型と&str型の違いについて確認
    let b_a = string_b + char_a;
    dbg!(b_a);

    let mut ans_input = String::new(); // TODO: string::new()
    std::io::stdin().read_line(&mut ans_input).unwrap(); // TODO: :: 系何してる?
    // dbg!(ans_input); // TODO: この文があるとエラー debugの有無でエラー有無はつらたん

    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);
    if ans_input == 1 + 1 {
        println!("正解")
    } else {
        println!("不正解")
    }

    println!("1-4");

    let mut ans_input = String::new(); // TODO: string::new()
    std::io::stdin().read_line(&mut ans_input).unwrap(); // TODO: :: 系何してる?
    // dbg!(ans_input); // TODO: この文があるとエラー debugの有無でエラー有無はつらたん

    let ans_input = ans_input.trim().parse::<i32>().unwrap();
    dbg!(ans_input);
    if ans_input == 1 - 4 {
        println!("正解")
    } else {
        println!("不正解")
    }
}

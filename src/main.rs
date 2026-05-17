use rand::RngExt;

fn main() {
    // step 1
    // let char_a = "a";
    // let string_b: String = "b".to_string(); // TODO: String型と&str型の違いについて確認
    // let b_a = string_b + char_a;
    // dbg!(b_a);

    // step 2
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::rng().random_range(1..=2); // TODO: enum等に変換

        let op1 = rand::rng().random_range(0..10);
        let op2 = rand::rng().random_range(0..10);

        loop {
            match quiz_mode {
                1 => {
                    println!("{}+{} = ??", op1, op2);
                }
                2 => {
                    println!("{}-{} = ??", op1, op2);
                }
                _ => unreachable!(),
            }

            let mut ans_input = String::new(); // TODO: string::new()

            std::io::stdin().read_line(&mut ans_input).unwrap(); // TODO: :: 系何してる?
            // dbg!(ans_input); // TODO: この文があるとエラー debugの有無でエラー有無はつらたん

            let ans_input = ans_input.trim().parse::<i32>().ok();
            match ans_input {
                Some(ans_input) => match quiz_mode {
                    1 => {
                        if ans_input == op1 + op2 {
                            println!("正解");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解")
                        }
                    }
                    2 => {
                        if ans_input == op1 - op2 {
                            println!("正解");
                            num_of_correct += 1;
                            break;
                        } else {
                            println!("不正解")
                        }
                    }
                    _ => unreachable!(),
                },
                None => {
                    println!("数値を入力してください")
                }
            }
        }
    }
}

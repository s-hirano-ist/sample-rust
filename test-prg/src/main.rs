// fn add_values(left: f64, right: f64) -> f64 {
//     left + right
// }

fn main() {
    // println!("Hello, world!");
    // println!("{:?}", add_values(1.0, 2.0));

    let mut foo: Vec<i32> = Vec::new(); // foo

    foo.push(123);

    println!("{}", foo.len());

    let hoge = &foo;
    // hoge.push(456); // TODO: ここ

    println!("{}", hoge.len());

    // let bar = foo;
    // foo.push(456);
    // bar.push(456);

    // println!("{}", foo.len());

    // {
    //     let buzz = bar;
    // }

    // println!("{}", bar.len());

    // let fuga = {
    //     let buzz: Vec<i32> = Vec::new();
    //     &buzz
    // };
}

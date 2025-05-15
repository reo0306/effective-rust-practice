fn main() {
    let amount_to_add = 3;
    let add_n = |y| {
        y + amount_to_add
    };
    let z = add_n(5);
    println!("{}", z);
}

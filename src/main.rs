struct S {
    field: Option<i32>,
}

fn main() {
    let amount_to_add = 3;
    let add_n = |y| {
        y + amount_to_add
    };
    let z = add_n(5);
    println!("{}", z);

    let s = S { field: Some(42) };
    match &s.field {
        Some(i) => println!("field is {i}"),
        None => {}
    }

    if let Some(i) = &s.field {
        println!("field is {i}");
    }
}

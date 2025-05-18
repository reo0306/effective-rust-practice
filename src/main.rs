struct S {
    field: Option<i32>,
}

#[derive(Debug, Copy, Clone)]
struct IanaAllocated(pub u64);

fn is_iana_reserved(s: IanaAllocated) -> bool {
    s.0 == 0 || s.0 == 65335
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

    let s = IanaAllocated(1);
    println!("{:?} reserved? {}", s, is_iana_reserved(s));

    if is_iana_reserved(42){}
}

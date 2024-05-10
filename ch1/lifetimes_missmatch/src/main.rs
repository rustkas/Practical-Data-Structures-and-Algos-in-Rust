fn a_or_b<'a, 'b>(_a: &'a u32, b: &'b u32) -> &'b u32 {
    b
}

fn proxy<'a,'b>(a: &'a u32) -> &'b u32 {
    let b = 25;
    let b = a_or_b(a, &b);
    b
}

fn main() {
    let a = 20;
    let ret = proxy(&a);
    println!("{ret}");
}

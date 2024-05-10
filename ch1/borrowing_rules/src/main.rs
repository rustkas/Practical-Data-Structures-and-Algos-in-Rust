fn get<'a>(arr: &'a [u32], idx: usize) -> &'a u32 {
    &arr[idx]
}

fn main() {
    let mut arr = [1, 2, 3];
    let one = get(&arr, 1);
    #[allow(dropping_references)]
    drop(one);
    let _mut_zero = &mut arr[0];
    
}

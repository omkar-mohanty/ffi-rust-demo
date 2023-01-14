#[no_mangle]
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    println!("In Rust code now");
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

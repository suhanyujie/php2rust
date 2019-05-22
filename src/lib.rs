use std::fmt::{Debug};

fn var_dump<T:Debug>(arg: &T) {
    println!("{:?}", arg);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_var_dump() {
        let x:i32 = 12;
        var_dump(&x);
        assert!(true);
    }
}

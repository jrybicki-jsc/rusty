
pub fn add_one(x:i32) -> i32 {
    return x + 1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let parm = 21;
        let res = add_one(parm);
        assert_eq!(res, 22);
    }
}

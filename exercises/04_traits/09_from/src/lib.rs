// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.


#[derive(Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(u: u32) -> WrappingU32 {
        WrappingU32 { value: u }
    }
}


// impl<u32, WrappingU32> Into<> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }
// This is automatically implemented by the compiler for me i geuss. 


fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intro() {
        let X = WrappingU32 {
            value: 42,
        };
        let newWrapping: WrappingU32 = 42.into();

        assert_eq!(X, newWrapping);
    }
}

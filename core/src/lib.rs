use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
pub struct Memo { pub id:i64, pub content:String, pub public:bool, pub created_at:String }

pub fn add(left: u64, right: u64) -> u64 {
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

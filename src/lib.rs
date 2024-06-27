use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::near_bindgen;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MyContract {
    // Define the contract's state here.
}

#[near_bindgen]
impl MyContract {
    // Add contract methods here.
    pub fn hello(&self) -> String {
        "Hello, NEAR!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let contract = MyContract::default();
        assert_eq!(contract.hello(), "Hello, NEAR!");
    }
}


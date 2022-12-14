//Contador incrementar, decrementar, estado, reset a 0

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contador {
    valor: i8,
}

impl Contador {
    pub fn get_num(&self) -> i8 {
        return self.valor;
    }

    pub fn add_num(&mut self) {
        self.valor += 1;
        let log_message = format!("Value updated to {}", self.valor);

        env::log(log_message.as_bytes());
        after_update();
    }

    pub fn sub_num(&mut self) {
        self.valor -= 1;
        let log_message = format!("Value updated to {}", self.valor);

        env::log(log_message.as_bytes());
        after_update();
    }

    pub fn reset(&mut self) {
        self.valor = 0;
        let log_message = format!("Value updated to {}", self.valor);

        env::log(log_message.as_bytes());
        after_update();
    }
}

fn after_update() {
    env::log("value updated".as_bytes());
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext{
        VMContext{
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0,1,2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0,1,2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn adding_method_test() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 0 };
        contract.add_num();

        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn sub_method_test() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 1 };
        contract.sub_num();

        assert_eq!(0, contract.get_num());
    }

    #[test]
    fn reset_method_test() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contador{ valor: 2 };
        contract.reset();

        assert_eq!(0, contract.get_num());
    }
}
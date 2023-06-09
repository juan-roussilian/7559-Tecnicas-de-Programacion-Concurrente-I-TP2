use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use crate::account::Account;
use crate::accounts_manager::AccountsManager;
use crate::errors::ServerError;

pub struct MemoryAccountsManager {
    accounts: HashMap<usize, Arc<RwLock<Account>>>,
}

impl AccountsManager for MemoryAccountsManager {

    fn new(&self) -> Self {
        MemoryAccountsManager {
            accounts: HashMap::new(),
        }
    }

    fn add_points(&mut self, account_id: usize, points: usize, operation_time: Option<u128>)->Result<(),ServerError> {
        if self.accounts.contains_key(&account_id) {
            if let Some(account) = self.accounts.get(&account_id) {
                if let Ok(account_guard) = account.write() {}
            }
        } else {
            if let Some(new_account) = Account::new(account_id, points) {
                self.accounts
                    .insert(account_id, Arc::new(RwLock::new(new_account)));
            }
        }
        Ok(())
    }

    fn request_points(&self, account_id: usize, points: usize) -> Result<(), ServerError> {
        Ok(())
    }
    fn cancel_requested_points(&self, account_id: usize) -> Result<(), ServerError> {
        Ok(())
    }
    fn substract_points(&self, account_id: usize, points: usize, operation_time: Option<u128>) -> Result<(), ServerError> {
        Ok(())
    }
}

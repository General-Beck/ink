#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod multisig_plain {
    use ink_core::{
        env::{
            self,
            call,
            DefaultEnvTypes,
        },
        storage,
    };

    const MAX_OWNERS: u32 = 50;

    #[derive(scale::Encode, scale::Decode, storage::Flush)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Transaction {
        callee: AccountId,
        data: call::CallData,
        gas_limit: u64,
        transferred_value: Balance,
    }

    #[ink(storage)]
    struct MultisigPlain {
        confirmations: storage::HashMap<(u32, AccountId), ()>,
        transactions: storage::Stash<Transaction>,
        owners: storage::Vec<AccountId>,
        is_owner: storage::HashMap<AccountId, ()>,
        transaction_count: storage::Value<u32>,
        required: storage::Value<u32>,
    }

    impl MultisigPlain {
        #[ink(constructor)]
        fn new(&mut self, owners: storage::Vec<AccountId>, required: u32) {
            for owner in owners.iter() {
                self.is_owner.insert(*owner, ());
            }
            self.owners = owners;
            ensure_requirement(self.owners.len(), required);
            assert!(self.is_owner.len() == self.owners.len());
            self.required.set(required);
            self.transaction_count.set(0);
        }

        #[ink(message)]
        fn add_owner(&mut self, owner: AccountId) {
            self.ensure_from_wallet();
            self.ensure_no_owner(&owner);
            ensure_requirement(self.owners.len() + 1, *self.required);
            self.is_owner.insert(owner, ());
            self.owners.push(owner);
        }

        #[ink(message)]
        fn remove_owner(&mut self, owner: AccountId) {
            self.ensure_from_wallet();
            self.ensure_owner(&owner);
            let len = self.owners.len() - 1;
            let required = u32::min(len, *self.required.get());
            ensure_requirement(len, required);
            self.owners.swap_remove(self.owner_index(&owner));
            self.is_owner.remove(&owner);
            self.required.set(required);
        }

        #[ink(message)]
        fn replace_owner(&mut self, owner: AccountId, new_owner: AccountId) {
            self.ensure_from_wallet();
            self.ensure_owner(&owner);
            self.ensure_no_owner(&new_owner);
            self.owners.replace(self.owner_index(&owner), || new_owner);
            self.is_owner.remove(&owner);
            self.is_owner.insert(new_owner, ());
        }

        #[ink(message)]
        fn change_requirement(&mut self, requirement: u32) {
            self.ensure_from_wallet();
            ensure_requirement(self.owners.len(), requirement);
            self.required.set(requirement);
        }

        #[ink(message)]
        fn submit_transaction(&mut self, transaction: Transaction) {
            self.ensure_from_owner();
            let id = self.transactions.put(transaction);
            self.internal_confirm(id);
        }

        #[ink(message)]
        fn confirm_transaction(&mut self, id: u32) {
            self.ensure_from_owner();
            self.ensure_transaction_exists(id);
            self.internal_confirm(id);
        }

        #[ink(message)]
        fn execute_transaction(&mut self, id: u32) {
            self.ensure_transaction_exists(id);
            self.ensure_confirmed(id);
            self.internal_confirm(id);
        }

        fn internal_confirm(&mut self, id: u32) {
            self.confirmations
                .insert((id, env::caller::<DefaultEnvTypes>().unwrap()), ());
            if self.is_confirmed(id) {
                self.internal_execute(id);
            }
        }

        fn internal_execute(&mut self, id: u32) {}

        fn owner_index(&self, owner: &AccountId) -> u32 {
            self.owners.iter().position(|x| *x == *owner).unwrap() as u32
        }

        fn is_confirmed(&self, id: u32) -> bool {
            self.owners
                .iter()
                .filter(|owner| self.confirmations.contains_key(&(id, **owner)))
                .count() as u32
                >= *self.required.get()
        }

        fn ensure_confirmed(&self, id: u32) {
            assert!(self.is_confirmed(id));
        }

        fn ensure_transaction_exists(&self, id: u32) {
            self.transactions.get(id).unwrap();
        }

        fn ensure_from_owner(&self) {
            assert!(self
                .is_owner
                .contains_key(&env::caller::<DefaultEnvTypes>().unwrap()));
        }

        fn ensure_from_wallet(&self) {
            assert!(
                env::caller::<DefaultEnvTypes>().unwrap()
                    == env::account_id::<DefaultEnvTypes>().unwrap()
            );
        }

        fn ensure_owner(&self, owner: &AccountId) {
            assert!(self.is_owner.contains_key(owner));
        }

        fn ensure_no_owner(&self, owner: &AccountId) {
            assert!(!self.is_owner.contains_key(owner));
        }
    }

    fn ensure_requirement(owners: u32, required: u32) {
        assert!(0 < required && required <= owners && owners <= MAX_OWNERS);
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
    }
}

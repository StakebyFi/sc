#![no_std]

multiversx_sc::imports!();

#[multiversx_sc::contract]
pub trait StakingContract {
    #[init]
    fn init(&self) {}

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();
        require!(payment_amount > 0, "Must pay more than 0");

        let caller = self.blockchain().get_caller();
        self.staking_position(&caller).update(|current_amount| {
            *current_amount += payment_amount;
        });
        self.staked_addresses().insert(caller);
    }

    #[endpoint]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let stake_mapper = self.staking_position(&caller);

        let caller_stake = stake_mapper.get();
        if caller_stake == 0 {
            return;
        }

        self.staked_addresses().swap_remove(&caller);
        stake_mapper.clear();

        self.send().direct_egld(&caller, &caller_stake);
    }

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
}

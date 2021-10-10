#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use std::ops::{Add, Mul};

    use casper_types::{Key, U256};

    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_decimals(), TestFixture::TOKEN_DECIMALS);
        println!(
            "{} {} {}",
            fixture.token_name(),
            fixture.token_symbol(),
            fixture.token_decimals()
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);
        assert_eq!(fixture.balance_of(Key::from(fixture.joe)), None);
    }

    #[test]
    fn should_transfer() {
        let mut fixture = TestFixture::install_contract();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        let transfer_amount_1 = U256::from(42);
        fixture.transfer(
            Key::from(fixture.bob),
            transfer_amount_1,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1)
        );

        let transfer_amount_2 = U256::from(20);
        fixture.transfer(
            Key::from(fixture.ali),
            transfer_amount_2,
            Sender(fixture.bob),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1 + transfer_amount_2),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1 - transfer_amount_2)
        );
    }

    #[test]
    fn should_create_stake() {
        let mut fixture = TestFixture::install_contract();

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        let stake_amount_1 = U256::from(1);

        fixture.create_stake(Key::from(fixture.ali), stake_amount_1, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - stake_amount_1)
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_1)
        )
    }

    #[test]
    fn should_create_and_remove_stake() {
        let mut fixture = TestFixture::install_contract();

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        let stake_amount_1 = U256::from(1);

        fixture.create_stake(Key::from(fixture.ali), stake_amount_1, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - stake_amount_1)
        );

        fixture.remove_stake(Key::from(fixture.ali), stake_amount_1, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );

        assert_eq!(fixture.stake_of(Key::from(fixture.ali)), Some(U256::zero()))
    }

    #[test]
    fn should_add_to_stake() {
        let mut fixture = TestFixture::install_contract();

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        let stake_amount_1 = U256::from(1);

        fixture.create_stake(Key::from(fixture.ali), stake_amount_1, Sender(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - stake_amount_1)
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_1)
        );

        fixture.create_stake(
            Key::from(fixture.ali),
            stake_amount_1.mul(10),
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - stake_amount_1.mul(11))
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_1.mul(11))
        );
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_fail_to_stake_if_balance_not_enough() {
        let mut fixture = TestFixture::install_contract();

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.create_stake(
            Key::from(fixture.ali),
            TestFixture::token_total_supply().add(1),
            Sender(fixture.ali),
        );
    }

    #[test]
    fn should_stake_full_amount_from_two_accounts() {
        let mut fixture = TestFixture::install_contract();
        let stake_amount_1 = U256::from(1);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );

        fixture.transfer(
            Key::from(fixture.bob),
            TestFixture::token_total_supply() - stake_amount_1.mul(3),
            Sender(fixture.ali),
        );

        fixture.create_stake(
            Key::from(fixture.ali),
            stake_amount_1.mul(3),
            Sender(fixture.ali),
        );

        fixture.create_stake(
            Key::from(fixture.bob),
            TestFixture::token_total_supply() - stake_amount_1.mul(3),
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::zero())
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_1.mul(3))
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.bob)),
            Some(TestFixture::token_total_supply() - stake_amount_1.mul(3))
        );

        assert_eq!(
            fixture.total_stakes(),
            Some(TestFixture::token_total_supply())
        )
    }

    #[test]
    fn should_store_total_stakes() {
        let mut fixture = TestFixture::install_contract();
        let stake_amount_1 = U256::from(1);
        let stake_amount_100 = U256::from(100);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );

        fixture.transfer(
            Key::from(fixture.bob),
            stake_amount_100,
            Sender(fixture.ali),
        );

        fixture.create_stake(Key::from(fixture.ali), stake_amount_1, Sender(fixture.ali));

        fixture.create_stake(
            Key::from(fixture.bob),
            stake_amount_100,
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_1)
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.bob)),
            Some(stake_amount_100)
        );

        assert_eq!(fixture.total_stakes(), Some(U256::from(101)));

        fixture.create_stake(
            Key::from(fixture.ali),
            stake_amount_100,
            Sender(fixture.ali),
        );

        assert_eq!(fixture.total_stakes(), Some(U256::from(201)));
    }

    #[test]
    fn should_distribute_rewards() {
        let mut fixture = TestFixture::install_contract();
        let stake_amount_110 = U256::from(110);

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );

        fixture.transfer(
            Key::from(fixture.bob),
            stake_amount_110,
            Sender(fixture.ali),
        );

        fixture.create_stake(
            Key::from(fixture.bob),
            stake_amount_110,
            Sender(fixture.bob),
        );

        fixture.create_stake(
            Key::from(fixture.ali),
            stake_amount_110.mul(3),
            Sender(fixture.ali),
        );

        assert_eq!(fixture.total_stakes(), Some(stake_amount_110.mul(4)));

        fixture.distribute_rewards(Key::from(fixture.ali), Sender(fixture.ali));

        assert_eq!(fixture.total_stakes(), Some(stake_amount_110.mul(4)));

        assert_eq!(
            fixture.reward_of(Key::from(fixture.ali)),
            Some(U256::from(33))
        );

        assert_eq!(
            fixture.reward_of(Key::from(fixture.bob)),
            Some(U256::from(11))
        );

        assert_eq!(fixture.total_rewards(), Some(U256::from(44)));
    }

    #[test]
    fn should_withdraw_rewards() {
        let mut fixture = TestFixture::install_contract();
        let stake_amount_110 = U256::from(110);

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali));

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );

        fixture.transfer(
            Key::from(fixture.bob),
            stake_amount_110,
            Sender(fixture.ali),
        );

        fixture.create_stake(
            Key::from(fixture.bob),
            stake_amount_110,
            Sender(fixture.bob),
        );

        fixture.create_stake(
            Key::from(fixture.ali),
            stake_amount_110.mul(3),
            Sender(fixture.ali),
        );

        fixture.distribute_rewards(Key::from(fixture.ali), Sender(fixture.ali));

        assert_eq!(
            fixture.reward_of(Key::from(fixture.ali)),
            Some(U256::from(33))
        );

        assert_eq!(
            fixture.reward_of(Key::from(fixture.bob)),
            Some(U256::from(11))
        );

        fixture.withdraw_reward(Key::from(fixture.ali), Sender(fixture.ali));

        assert_eq!(
            fixture.reward_of(Key::from(fixture.ali)),
            Some(U256::from(0))
        );

        assert_eq!(
            fixture.stake_of(Key::from(fixture.ali)),
            Some(stake_amount_110.mul(3))
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(
                initial_ali_balance.unwrap()
            - stake_amount_110.mul(4) // 330 staked, 110 sent to bob
            + U256::from(33)
            ) // the reward
        );
    }

    #[test]
    fn should_transfer_full_amount() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();

        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance,
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(initial_ali_balance)
        );
        println!("ali {}", initial_ali_balance);
        println!(
            "bob received {:?}",
            fixture.balance_of(Key::from(fixture.bob))
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        fixture.transfer(
            Key::from(fixture.ali),
            initial_ali_balance,
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::zero())
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(initial_ali_balance)
        );
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_not_transfer_with_insufficient_balance() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance + U256::one(),
            Sender(fixture.ali),
        );
    }

    #[test]
    fn should_transfer_from() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");
        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Sender(spender),
        );

        assert_eq!(
            fixture.balance_of(Key::from(owner)),
            Some(owner_balance_before - transfer_amount),
            "should decrease balance of the owner"
        );
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount - transfer_amount),
            "should decrease allowance of the spender"
        );
        assert_eq!(
            fixture.balance_of(Key::from(recipient)),
            Some(transfer_amount),
            "recipient should receive tokens"
        );
    }

    #[should_panic(expected = "ApiError::User(65533) [131069]")]
    #[test]
    fn should_not_transfer_from_more_than_approved() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            approve_amount + U256::one(),
            Sender(spender),
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

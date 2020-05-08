#[cfg(test)]
mod erc20;

#[cfg(test)]
mod tests {
    use super::erc20;
    use erc20::{
        account::{ALI, BOB, JOE},
        ERC20Contract, Sender, ERC20_INIT_BALANCE,
    };

    #[test]
    fn test_erc20_deploy() {
        let token = ERC20Contract::deployed();
        assert_eq!(token.balance_of(ALI), ERC20_INIT_BALANCE);
        assert_eq!(token.balance_of(BOB), 0);
        assert_eq!(token.total_supply(), ERC20_INIT_BALANCE);
    }

    #[test]
    fn test_erc20_transfer() {
        let amount = 10;
        let mut token = ERC20Contract::deployed();
        token.transfer(BOB, amount, Sender(ALI));
        assert_eq!(token.balance_of(ALI), ERC20_INIT_BALANCE - amount);
        assert_eq!(token.balance_of(BOB), amount);
    }

    #[test]
    #[should_panic]
    fn test_erc20_transfer_too_much() {
        let amount = ERC20_INIT_BALANCE + 1;
        let mut token = ERC20Contract::deployed();
        token.transfer(BOB,amount,Sender(ALI));
    }


    #[test]
    fn test_erc20_approve() {
        let amount = 10;
        let mut token = ERC20Contract::deployed();
        token.approve(BOB, amount, Sender(ALI));
        assert_eq!(token.balance_of(ALI), ERC20_INIT_BALANCE);
        assert_eq!(token.balance_of(BOB), 0);
        assert_eq!(token.allowance(ALI, BOB), amount);
        assert_eq!(token.allowance(BOB, ALI), 0);
    }

    #[test]
    #[should_panic]
    fn test_approve_too_much() {
        let allowance = 1;
        let amount = 2;
        let  mut token = ERC20Contract::deployed();
        token.approve(BOB,allowance,Sender(ALI));
        token.transfer_from(ALI,JOE,amount,Sender(BOB));
    }


    #[test]
    fn test_erc20_transfer_from() {
        let allowance = 10;
        let amount = 3;
        let mut token = ERC20Contract::deployed();
        token.approve(BOB, allowance, Sender(ALI));
        token.transfer_from(ALI, JOE, amount, Sender(BOB));
        assert_eq!(token.balance_of(ALI), ERC20_INIT_BALANCE - amount);
        assert_eq!(token.balance_of(BOB), 0);
        assert_eq!(token.balance_of(JOE), amount);
        assert_eq!(token.allowance(ALI, BOB), allowance - amount);
    }

    #[test]
    #[should_panic]
    fn test_erc20_transfer_from_too_much() {
        let amount = ERC20_INIT_BALANCE + 1;
        let mut token = ERC20Contract::deployed();
        token.transfer_from(ALI,JOE,amount,Sender(BOB));
    }
}

mod airdrop;
mod keygen;
mod wallet;

mod programs;
mod transfer;

mod enroll;

use crate::programs::wba_prereq::{CompleteArgs, UpdateArgs, WbaPrereqProgram};
#[cfg(test)]

mod tests {
    use solana_sdk;
    #[test]
    fn keygen() {}
    #[test]
    fn airdop() {}
    #[test]
    fn transfer_sol() {}
}

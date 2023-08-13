use ethers_core::types::U64;
#[derive(PartialEq)]
pub enum Route {
    Home,
    Search,
    //Address(Address),
    Block(U64),
    Blocks,
    //Transaction(usize),
    Transactions,
}

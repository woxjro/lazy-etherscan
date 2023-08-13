use ethers_core::types::U64;

#[derive(PartialEq, Clone)]
pub enum Route {
    Home(HomeRoute),
}

#[derive(PartialEq, Clone)]
pub enum HomeRoute {
    Root,
    Search,
    LatestBlocks,
    LatestTransactions,
    Block(U64),
    //Transaction(usize),
}

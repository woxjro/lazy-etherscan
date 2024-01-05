use ethers::core::types::Block;

pub enum SelectableBlockDetailItem {
    Transactions,
    Withdrawls,
    FeeRecipient,
    ParentHash,
}

impl SelectableBlockDetailItem {
    pub fn next<T>(&self, block: &Block<T>) -> Self {
        match self {
            Self::Transactions => {
                if block.withdrawals.is_some() {
                    Self::Withdrawls
                } else {
                    Self::FeeRecipient
                }
            }
            Self::Withdrawls => Self::FeeRecipient,
            Self::FeeRecipient => {
                if block.author.is_some() {
                    Self::ParentHash
                } else {
                    Self::Transactions
                }
            }
            Self::ParentHash => Self::Transactions,
        }
    }

    pub fn previous<T>(&self, block: &Block<T>) -> Self {
        match self {
            Self::Transactions => {
                if block.author.is_some() {
                    Self::ParentHash
                } else {
                    Self::FeeRecipient
                }
            }
            Self::Withdrawls => Self::Transactions,
            Self::FeeRecipient => {
                if block.withdrawals.is_some() {
                    Self::Withdrawls
                } else {
                    Self::Transactions
                }
            }
            Self::ParentHash => Self::FeeRecipient,
        }
    }
}

impl From<usize> for SelectableBlockDetailItem {
    fn from(i: usize) -> Self {
        if i == 0 {
            Self::Transactions
        } else if i == 1 {
            Self::Withdrawls
        } else if i == 2 {
            Self::FeeRecipient
        } else if i == 3 {
            Self::ParentHash
        } else {
            unreachable!()
        }
    }
}

impl From<SelectableBlockDetailItem> for usize {
    fn from(val: SelectableBlockDetailItem) -> Self {
        match val {
            SelectableBlockDetailItem::Transactions => 0,
            SelectableBlockDetailItem::Withdrawls => 1,
            SelectableBlockDetailItem::FeeRecipient => 2,
            SelectableBlockDetailItem::ParentHash => 3,
        }
    }
}

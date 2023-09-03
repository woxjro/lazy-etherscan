use crate::ethers::types::TransactionWithReceipt;

pub enum SelectableTransactionDetailItem {
    From,
    To,
}

impl SelectableTransactionDetailItem {
    pub fn next(&self, transaction: &TransactionWithReceipt) -> Self {
        match self {
            Self::From => {
                if let Some(_) = transaction.transaction.to {
                    Self::To
                } else {
                    Self::From
                }
            }
            Self::To => Self::From,
        }
    }

    pub fn previous(&self, transaction: &TransactionWithReceipt) -> Self {
        match self {
            Self::From => {
                if let Some(_) = transaction.transaction.to {
                    Self::To
                } else {
                    Self::From
                }
            }
            Self::To => Self::From,
        }
    }
}

impl From<usize> for SelectableTransactionDetailItem {
    fn from(i: usize) -> Self {
        if i == 0 {
            Self::From
        } else if i == 1 {
            Self::To
        } else {
            panic!()
        }
    }
}

impl Into<usize> for SelectableTransactionDetailItem {
    fn into(self) -> usize {
        match self {
            Self::From => 0,
            Self::To => 1,
        }
    }
}

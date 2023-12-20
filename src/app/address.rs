use crate::ethers::types::AddressInfo;

#[derive(Copy, Clone)]
pub enum SelectableContractDetailItem {
    ContractSourceCode, //0
    ContractAbi,        //1
}

impl SelectableContractDetailItem {
    pub fn next(&self, address_info: &AddressInfo) -> Self {
        match self {
            Self::ContractAbi => {
                if address_info.contract_source_code.is_some() {
                    Self::ContractSourceCode
                } else {
                    Self::ContractAbi
                }
            }
            Self::ContractSourceCode => {
                if address_info.contract_abi.is_some() {
                    Self::ContractAbi
                } else {
                    Self::ContractSourceCode
                }
            }
        }
    }

    pub fn previous(&self, address_info: &AddressInfo) -> Self {
        match self {
            Self::ContractAbi => {
                if address_info.contract_source_code.is_some() {
                    Self::ContractSourceCode
                } else {
                    Self::ContractAbi
                }
            }
            Self::ContractSourceCode => {
                if address_info.contract_abi.is_some() {
                    Self::ContractAbi
                } else {
                    Self::ContractSourceCode
                }
            }
        }
    }
}

impl Default for SelectableContractDetailItem {
    fn default() -> Self {
        Self::ContractSourceCode
    }
}

impl From<usize> for SelectableContractDetailItem {
    fn from(i: usize) -> Self {
        if i == 0 {
            Self::ContractSourceCode
        } else if i == 1 {
            Self::ContractAbi
        } else {
            unreachable!()
        }
    }
}

impl From<SelectableContractDetailItem> for usize {
    fn from(val: SelectableContractDetailItem) -> Self {
        match val {
            SelectableContractDetailItem::ContractSourceCode => 0,
            SelectableContractDetailItem::ContractAbi => 1,
        }
    }
}

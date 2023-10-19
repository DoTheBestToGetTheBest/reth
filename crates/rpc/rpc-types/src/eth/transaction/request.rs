use crate::eth::transaction::typed::{
    EIP1559TransactionRequest, EIP2930TransactionRequest, LegacyTransactionRequest,
    TransactionKind, TypedTransactionRequest,
};
use alloy_primitives::{Address, Bytes, U128, U256, U64, U8};
use reth_primitives::AccessList;
use serde::{Deserialize, Serialize};
/// Represents _all_ transaction requests received from RPC
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    /// from address
    pub from: Option<Address>,
    /// to address
    pub to: Option<Address>,
    /// legacy, gas Price
    #[serde(default)]
    pub gas_price: Option<U128>,
    /// max base fee per gas sender is willing to pay
    #[serde(default)]
    pub max_fee_per_gas: Option<U128>,
    /// miner tip
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<U128>,
    /// gas
    pub gas: Option<U256>,
    /// value of th tx in wei
    pub value: Option<U256>,
    /// Any additional data sent
    #[serde(alias = "input")]
    pub data: Option<Bytes>,
    /// Transaction nonce
    pub nonce: Option<U64>,
    /// warm storage access pre-payment
    #[serde(default)]
    pub access_list: Option<AccessList>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U8>,
}

// == impl TransactionRequest ==

impl TransactionRequest {
    /// Converts the request into a [`TypedTransactionRequest`]
    ///
    /// Returns None if mutual exclusive fields `gasPrice` and `max_fee_per_gas` are either missing
    /// or both set.
    pub fn into_typed_request(self) -> Option<TypedTransactionRequest> {
        let TransactionRequest {
            to,
            gas_price,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            gas,
            value,
            data,
            nonce,
            mut access_list,
            ..
        } = self;
        match (gas_price, max_fee_per_gas, access_list.take()) {
            // legacy transaction
            (Some(_), None, None) => {
                Some(TypedTransactionRequest::Legacy(LegacyTransactionRequest {
                    nonce: nonce.unwrap_or_default(),
                    gas_price: gas_price.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                    input: data.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    chain_id: None,
                }))
            }
            // EIP2930
            (_, None, Some(access_list)) => {
                Some(TypedTransactionRequest::EIP2930(EIP2930TransactionRequest {
                    nonce: nonce.unwrap_or_default(),
                    gas_price: gas_price.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                    input: data.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    chain_id: 0,
                    access_list,
                }))
            }
            // EIP1559
            (None, Some(_), access_list) | (None, None, access_list @ None) => {
                // Empty fields fall back to the canonical transaction schema.
                Some(TypedTransactionRequest::EIP1559(EIP1559TransactionRequest {
                    nonce: nonce.unwrap_or_default(),
                    max_fee_per_gas: max_fee_per_gas.unwrap_or_default(),
                    max_priority_fee_per_gas: max_priority_fee_per_gas.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    value: value.unwrap_or_default(),
                    input: data.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    chain_id: 0,
                    access_list: access_list.unwrap_or_default(),
                }))
            }
            #[allow(unreachable_code)]
            #[allow(unreachable_patterns)]
            // EIP4844
            (None, Some(_), access_list) | (None, None, access_list @ None) => {
                Some(TypedTransactionRequest::EIP4844(crate::Eip4844TransactionRequest {
                    chain_id: 0,
                    nonce: nonce.unwrap_or_default(),
                    max_priority_fee_per_gas: max_priority_fee_per_gas.unwrap_or_default(),
                    max_fee_per_gas: max_fee_per_gas.unwrap_or_default(),
                    gas_limit: gas.unwrap_or_default(),
                    kind: match to {
                        Some(to) => TransactionKind::Call(to),
                        None => TransactionKind::Create,
                    },
                    value: value.unwrap_or_default(),
                    gas_price: gas_price.unwrap_or_default(),
                    access_list: access_list.unwrap_or_default(),
                    input: data.unwrap_or_default(),
                    blob_versioned_hashes: todo!(),
                    max_fee_per_blob_gas: todo!(),
                    sidecar: todo!(),
                }))
            }
            _ => None,
        }
    }

    // fn signed(transaction: Transaction, signer: B256) -> TransactionSigned {
    //   todo!()
    //}
    /// Sets the gas limit for the transaction.

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas = Some(U256::from(gas_limit));
        self
    }
    /// Sets the nonce for the transaction.

    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(U64::from(nonce));
        self
    }
    /// Increments the nonce for the transaction.

    pub fn inc_nonce(mut self) -> Self {
        if let Some(ref mut nonce_value) = self.nonce {
            *nonce_value += U64::from(1);
        } else {
            self.nonce = Some(U64::from(1));
        }
        self
    }
    /// Decrements the nonce for the transaction.

    pub fn dcr_nonce(mut self) -> Self {
        if let Some(ref mut nonce_value) = self.nonce {
            *nonce_value -= U64::from(1);
        }
        self
    }
    /// Sets the maximum fee per gas for the transaction.

    pub fn max_fee_per_gas(mut self, max_fee_per_gas: u128) -> Self {
        self.max_fee_per_gas = Some(U128::from(max_fee_per_gas));
        self
    }
    /// Sets the maximum priority fee per gas for the transaction.

    pub fn max_priority_fee_per_gas(mut self, max_priority_fee_per_gas: u128) -> Self {
        self.max_priority_fee_per_gas = Some(U128::from(max_priority_fee_per_gas));
        self
    }
    /// Sets the recipient address for the transaction.

    pub fn to(mut self, to: Address) -> Self {
        self.to = Some(to);
        self
    }
    /// Sets the value (amount) for the transaction.

    pub fn value(mut self, value: u128) -> Self {
        self.value = Some(U256::from(value));
        self
    }
    /// Sets the access list for the transaction.

    pub fn access_list(mut self, access_list: AccessList) -> Self {
        self.access_list = Some(access_list);
        self
    }
    /// Sets the input data for the transaction.

    pub fn input(mut self, input: Bytes) -> Self {
        self.data = Some(input);
        self
    }

    /// Sets the transactions type for the transactions.

    pub fn transaction_type(mut self, transaction_type: u8) -> Self {
        self.transaction_type = Some(U8::from(transaction_type));
        self
    }

    // pub fn set_nonce(&mut self, nonce: u64) -> &mut Self {
    //     self.nonce = Some(U64::from(nonce));
    //     self
    // }
    // pub fn set_gas_limit(&mut self, gas_limit: u64) -> &mut Self {
    //     self.gas = Some(U256::from(gas_limit));
    //     self
    // }

    // pub fn set_max_fee_per_gas(&mut self, max_fee_per_gas: u128) -> &mut Self {
    //     self.max_fee_per_gas = Some(U128::from(max_fee_per_gas));
    //     self
    // }

    // pub fn set_max_priority_fee_per_gas(&mut self, max_priority_fee_per_gas: u128) -> &mut Self {
    //     self.max_priority_fee_per_gas = Some(U128::from(max_priority_fee_per_gas));
    //     self
    // }
    // pub fn set_to(&mut self, to: Address) -> &mut Self {
    //     self.to = Some(to);
    //     self
    // }
    // pub fn set_value(&mut self, value: u128) -> &mut Self {
    //     self.value = Some(U256::from(value));
    //     self
    // }

    // pub fn set_access_list(&mut self, access_list: AccessList) -> &mut Self {
    //     self.access_list = Some(access_list);
    //     self
    // }
}
// impl Default for TransactionRequest {
//     fn default() -> Self {
//         Self {
//             from: Default::default(),
//             nonce: 0,
//             gas_price: 0,
//             gas: 0,
//             max_fee_per_gas: 0,
//             max_priority_fee_per_gas: 0,
//             to: Default::default(),
//             value: Default::default(),
//             access_list: Default::default(),
//             data: Default::default(),
//             transaction_type: Default::default(),
//         }
//     }
// }

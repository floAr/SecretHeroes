use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, StdResult, Uint128};

use crate::msg::{Action, Status};

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
pub struct Proposal {
    pub id: u32,
    pub action: Action,
    pub status: Status,
    pub description: Option<String>,
    pub yes: Vec<HumanAddr>,
    pub no: Vec<HumanAddr>,
}

/// actions that a proposal can take
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StoredAction {
    /// add a voter to the authorized list
    AddVoter { address: CanonicalAddr },
    /// remove a voter from the authorized list
    RemoveVoter { address: CanonicalAddr },
    /// withdraw funds from the contract
    Withdraw {
        /// address to send the funds to
        recipient: CanonicalAddr,
        /// amount to send
        amount: Uint128,
    },
}

impl StoredAction {
    pub fn into_humanized<A: Api>(self, api: &A) -> StdResult<Action> {
        let action = match self {
            StoredAction::AddVoter { address } => Action::AddVoter {
                address: api.human_address(&address)?,
            },
            StoredAction::RemoveVoter { address } => Action::RemoveVoter {
                address: api.human_address(&address)?,
            },
            StoredAction::Withdraw { recipient, amount } => Action::Withdraw {
                recipient: api.human_address(&recipient)?,
                amount,
            },
        };
        Ok(action)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoredProposal {
    pub id: u32,
    pub action: StoredAction,
    pub status: Status,
    pub description: Option<String>,
    pub votes: [Vec<CanonicalAddr>; 2],
}

impl StoredProposal {
    pub fn into_humanized<A: Api>(self, api: &A) -> StdResult<Proposal> {
        Ok(Proposal {
            id: self.id,
            action: self.action.into_humanized(api)?,
            status: self.status,
            description: self.description,
            yes: self.votes[0]
                .iter()
                .map(|a| api.human_address(&a))
                .collect::<StdResult<Vec<HumanAddr>>>()?,
            no: self.votes[1]
                .iter()
                .map(|a| api.human_address(&a))
                .collect::<StdResult<Vec<HumanAddr>>>()?,
        })
    }
}

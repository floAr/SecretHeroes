use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{HumanAddr, Uint128};

use crate::proposal::Proposal;

/// Instantiation message
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct InitMsg {
    /// entropy used for prng seed
    pub entropy: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// propose an action
    Propose {
        /// action to perform if proposal passes
        action: Action,
        /// optional description for the propoal
        description: Option<String>,
    },
    /// vote on an existing proposal
    Vote {
        /// proposal id
        id: u32,
        /// your vote
        vote: Vote,
    },
    /// create a viewing key
    CreateViewingKey {
        /// entropy String used in random key generation
        entropy: String,
    },
    /// set viewing key
    SetViewingKey {
        /// desired viewing key
        key: String,
        /// optional message length padding
        padding: Option<String>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    /// response from setting and creating a viewing key
    ViewingKey { key: String },
    /// list of proposals that closed after either voting or creating a new one
    ClosedProposals {
        /// list of proposals that passed as a result of this tx
        passed: Vec<u32>,
        /// list of proposals that failed as a result of this tx
        failed: Vec<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// list all proposals in descending order
    ListAllProposals {
        /// address of the querier
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
        /// optional proposal id to start listing from.  If not specified, it will
        /// start with the most recently created proposal
        start_at: Option<u32>,
        /// number of proposals to display
        limit: Option<u32>,
    },
    /// list only the open proposals
    ListOpenProposals {
        /// address of the querier
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
    },
    /// display a single proposal
    Proposal {
        /// proposal id
        id: u32,
        /// address of the querier
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
    },
    /// display the list of authorized voters
    ListVoters {
        /// address of the querier
        address: HumanAddr,
        /// querier's viewing key
        viewing_key: String,
    },
    /// display a voter's vote on a specific proposal
    Vote {
        /// proposal id
        id: u32,
        /// voter whose vote history is requested
        voter: HumanAddr,
        /// querier's address if different from the voter's address
        viewer: Option<HumanAddr>,
        /// viewing key of the querier
        viewing_key: String,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    /// list of proposals
    ProposalList { proposals: Vec<Proposal> },
    /// proposal info
    ProposalInfo { proposal: Proposal },
    /// list of voters
    VoterList { voters: Vec<HumanAddr> },
    /// history of a vote
    Vote {
        /// a voter's history for a specific proposal.  None if they did not vote
        vote: Option<Vote>,
    },
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Vote {
    Yes,
    No,
}

/// status of a proposal
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Open,
    Passed,
    Failed,
}

/// actions that a proposal can take
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    /// add a voter to the authorized list
    AddVoter { address: HumanAddr },
    /// remove a voter from the authorized list
    RemoveVoter { address: HumanAddr },
    /// withdraw funds from the contract
    Withdraw {
        /// address to send the funds to
        recipient: HumanAddr,
        /// amount to send
        amount: Uint128,
    },
}

// Copyright 2021 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: Apache-2.0

use crate::helpers::calculate_epoch_reward_rate;
use crate::state::State;
use crate::storage::{
    config, layer_distribution, reverse_gateway_delegations, reverse_mix_delegations,
};
use crate::{error::ContractError, queries, transactions};
use config::defaults::NETWORK_MONITOR_ADDRESS;
use cosmwasm_std::{
    entry_point, to_binary, Addr, Decimal, Deps, DepsMut, Env, MessageInfo, QueryResponse,
    Response, Uint128,
};
use mixnet_contract::{
    Delegation, ExecuteMsg, IdentityKey, IdentityKeyRef, InstantiateMsg, MigrateMsg, QueryMsg,
    StateParams,
};

pub const INITIAL_DEFAULT_EPOCH_LENGTH: u32 = 2;

/// Constant specifying minimum of coin required to bond a gateway
pub const INITIAL_GATEWAY_BOND: Uint128 = Uint128(100_000000);

/// Constant specifying minimum of coin required to bond a mixnode
pub const INITIAL_MIXNODE_BOND: Uint128 = Uint128(100_000000);

// percentage annual increase. Given starting value of x, we expect to have 1.1x at the end of the year
pub const INITIAL_MIXNODE_BOND_REWARD_RATE: u64 = 110;
pub const INITIAL_GATEWAY_BOND_REWARD_RATE: u64 = 110;
pub const INITIAL_MIXNODE_DELEGATION_REWARD_RATE: u64 = 110;
pub const INITIAL_GATEWAY_DELEGATION_REWARD_RATE: u64 = 110;

pub const INITIAL_MIXNODE_ACTIVE_SET_SIZE: u32 = 100;

fn default_initial_state(owner: Addr) -> State {
    let mixnode_bond_reward_rate = Decimal::percent(INITIAL_MIXNODE_BOND_REWARD_RATE);
    let gateway_bond_reward_rate = Decimal::percent(INITIAL_GATEWAY_BOND_REWARD_RATE);
    let mixnode_delegation_reward_rate = Decimal::percent(INITIAL_MIXNODE_DELEGATION_REWARD_RATE);
    let gateway_delegation_reward_rate = Decimal::percent(INITIAL_GATEWAY_DELEGATION_REWARD_RATE);

    State {
        owner,
        network_monitor_address: Addr::unchecked(NETWORK_MONITOR_ADDRESS), // we trust our hardcoded value
        params: StateParams {
            epoch_length: INITIAL_DEFAULT_EPOCH_LENGTH,
            minimum_mixnode_bond: INITIAL_MIXNODE_BOND,
            minimum_gateway_bond: INITIAL_GATEWAY_BOND,
            mixnode_bond_reward_rate,
            gateway_bond_reward_rate,
            mixnode_delegation_reward_rate,
            gateway_delegation_reward_rate,
            mixnode_active_set_size: INITIAL_MIXNODE_ACTIVE_SET_SIZE,
        },
        mixnode_epoch_bond_reward: calculate_epoch_reward_rate(
            INITIAL_DEFAULT_EPOCH_LENGTH,
            mixnode_bond_reward_rate,
        ),
        gateway_epoch_bond_reward: calculate_epoch_reward_rate(
            INITIAL_DEFAULT_EPOCH_LENGTH,
            gateway_bond_reward_rate,
        ),
        mixnode_epoch_delegation_reward: calculate_epoch_reward_rate(
            INITIAL_DEFAULT_EPOCH_LENGTH,
            mixnode_delegation_reward_rate,
        ),
        gateway_epoch_delegation_reward: calculate_epoch_reward_rate(
            INITIAL_DEFAULT_EPOCH_LENGTH,
            gateway_delegation_reward_rate,
        ),
    }
}

/// Instantiate the contract.
///
/// `deps` contains Storage, API and Querier
/// `env` contains block, message and contract info
/// `msg` is the contract initialization message, sort of like a constructor call.
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = default_initial_state(info.sender);

    config(deps.storage).save(&state)?;
    layer_distribution(deps.storage).save(&Default::default())?;
    Ok(Response::default())
}

/// Handle an incoming message
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::BondMixnode { mix_node } => transactions::try_add_mixnode(deps, info, mix_node),
        ExecuteMsg::UnbondMixnode {} => transactions::try_remove_mixnode(deps, info),
        ExecuteMsg::BondGateway { gateway } => transactions::try_add_gateway(deps, info, gateway),
        ExecuteMsg::UnbondGateway {} => transactions::try_remove_gateway(deps, info),
        ExecuteMsg::UpdateStateParams(params) => {
            transactions::try_update_state_params(deps, info, params)
        }
        ExecuteMsg::RewardMixnode { identity, uptime } => {
            transactions::try_reward_mixnode(deps, info, identity, uptime)
        }
        ExecuteMsg::RewardGateway { identity, uptime } => {
            transactions::try_reward_gateway(deps, info, identity, uptime)
        }
        ExecuteMsg::DelegateToMixnode { mix_identity } => {
            transactions::try_delegate_to_mixnode(deps, info, mix_identity)
        }
        ExecuteMsg::UndelegateFromMixnode { mix_identity } => {
            transactions::try_remove_delegation_from_mixnode(deps, info, mix_identity)
        }
        ExecuteMsg::DelegateToGateway { gateway_identity } => {
            transactions::try_delegate_to_gateway(deps, info, gateway_identity)
        }
        ExecuteMsg::UndelegateFromGateway { gateway_identity } => {
            transactions::try_remove_delegation_from_gateway(deps, info, gateway_identity)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<QueryResponse, ContractError> {
    let query_res = match msg {
        QueryMsg::GetMixNodes { start_after, limit } => {
            to_binary(&queries::query_mixnodes_paged(deps, start_after, limit)?)
        }
        QueryMsg::GetGateways { limit, start_after } => {
            to_binary(&queries::query_gateways_paged(deps, start_after, limit)?)
        }
        QueryMsg::OwnsMixnode { address } => {
            to_binary(&queries::query_owns_mixnode(deps, address)?)
        }
        QueryMsg::OwnsGateway { address } => {
            to_binary(&queries::query_owns_gateway(deps, address)?)
        }
        QueryMsg::StateParams {} => to_binary(&queries::query_state_params(deps)),
        QueryMsg::LayerDistribution {} => to_binary(&queries::query_layer_distribution(deps)),
        QueryMsg::GetMixDelegations {
            mix_identity,
            start_after,
            limit,
        } => to_binary(&queries::query_mixnode_delegations_paged(
            deps,
            mix_identity,
            start_after,
            limit,
        )?),
        QueryMsg::GetReverseMixDelegations {
            delegation_owner,
            start_after,
            limit,
        } => to_binary(&queries::query_reverse_mixnode_delegations_paged(
            deps,
            delegation_owner,
            start_after,
            limit,
        )?),
        QueryMsg::GetMixDelegation {
            mix_identity,
            address,
        } => to_binary(&queries::query_mixnode_delegation(
            deps,
            mix_identity,
            address,
        )?),
        QueryMsg::GetGatewayDelegations {
            gateway_identity,
            start_after,
            limit,
        } => to_binary(&queries::query_gateway_delegations_paged(
            deps,
            gateway_identity,
            start_after,
            limit,
        )?),
        QueryMsg::GetReverseGatewayDelegations {
            delegation_owner,
            start_after,
            limit,
        } => to_binary(&queries::query_reverse_gateway_delegations_paged(
            deps,
            delegation_owner,
            start_after,
            limit,
        )?),
        QueryMsg::GetGatewayDelegation {
            gateway_identity,
            address,
        } => to_binary(&queries::query_gateway_delegation(
            deps,
            gateway_identity,
            address,
        )?),
    };

    Ok(query_res?)
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    fn get_all_mixnodes_identities(deps: &DepsMut) -> Result<Vec<IdentityKey>, ContractError> {
        let mut mixnode_bonds = Vec::new();
        let mut start_after = None;
        loop {
            let mut paged_response =
                queries::query_mixnodes_paged(deps.as_ref(), start_after, None)?;
            mixnode_bonds.append(&mut paged_response.nodes);

            if let Some(start_after_res) = paged_response.start_next_after {
                start_after = Some(start_after_res)
            } else {
                break;
            }
        }
        let mixnodes = mixnode_bonds
            .into_iter()
            .map(|bond| bond.mix_node.identity_key)
            .collect();

        Ok(mixnodes)
    }

    fn get_all_gateways_identities(deps: &DepsMut) -> Result<Vec<IdentityKey>, ContractError> {
        let mut gateway_bonds = Vec::new();
        let mut start_after = None;
        loop {
            let mut paged_response =
                queries::query_gateways_paged(deps.as_ref(), start_after, None)?;
            gateway_bonds.append(&mut paged_response.nodes);

            if let Some(start_after_res) = paged_response.start_next_after {
                start_after = Some(start_after_res)
            } else {
                break;
            }
        }
        let gateways = gateway_bonds
            .into_iter()
            .map(|bond| bond.gateway.identity_key)
            .collect();

        Ok(gateways)
    }

    fn get_all_mixnode_delegations(
        deps: &DepsMut,
        mix_identity: IdentityKeyRef,
    ) -> Result<Vec<Delegation>, ContractError> {
        let mut delegations = Vec::new();
        let mut start_after = None;
        loop {
            let mut paged_response = queries::query_mixnode_delegations_paged(
                deps.as_ref(),
                mix_identity.into(),
                start_after,
                None,
            )?;
            delegations.append(&mut paged_response.delegations);

            if let Some(start_after_res) = paged_response.start_next_after {
                start_after = Some(start_after_res)
            } else {
                break;
            }
        }

        Ok(delegations)
    }

    fn get_all_gateway_delegations(
        deps: &DepsMut,
        gateway_identity: IdentityKeyRef,
    ) -> Result<Vec<Delegation>, ContractError> {
        let mut delegations = Vec::new();
        let mut start_after = None;
        loop {
            let mut paged_response = queries::query_gateway_delegations_paged(
                deps.as_ref(),
                gateway_identity.into(),
                start_after,
                None,
            )?;
            delegations.append(&mut paged_response.delegations);

            if let Some(start_after_res) = paged_response.start_next_after {
                start_after = Some(start_after_res)
            } else {
                break;
            }
        }

        Ok(delegations)
    }

    let mixnodes_identities = get_all_mixnodes_identities(&deps)?;
    for mix_identity in mixnodes_identities {
        let delegations = get_all_mixnode_delegations(&deps, &mix_identity)?;
        for delegation in delegations {
            reverse_mix_delegations(deps.storage, &delegation.owner())
                .save(mix_identity.as_bytes(), &())?;
        }
    }

    let gateways_identities = get_all_gateways_identities(&deps)?;
    for gateway_identity in gateways_identities {
        let delegations = get_all_gateway_delegations(&deps, &gateway_identity)?;
        for delegation in delegations {
            reverse_gateway_delegations(deps.storage, &delegation.owner())
                .save(gateway_identity.as_bytes(), &())?;
        }
    }

    Ok(Default::default())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::support::tests::helpers::*;
    use config::defaults::DENOM;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use mixnet_contract::PagedMixnodeResponse;

    #[test]
    fn initialize_contract() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // mix_node_bonds should be empty after initialization
        let res = query(
            deps.as_ref(),
            env.clone(),
            QueryMsg::GetMixNodes {
                start_after: None,
                limit: Option::from(2),
            },
        )
        .unwrap();
        let page: PagedMixnodeResponse = from_binary(&res).unwrap();
        assert_eq!(0, page.nodes.len()); // there are no mixnodes in the list when it's just been initialized

        // Contract balance should match what we initialized it as
        assert_eq!(
            coins(0, DENOM),
            query_contract_balance(env.contract.address, deps)
        );
    }
}

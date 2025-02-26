import { FeeDetails, DecCoin, Gateway, MixNode } from '@nymproject/types';
import { TBondGatewayArgs, TBondMixNodeArgs } from 'src/types';
import { invokeWrapper } from './wrapper';

export const simulateBondGateway = async (args: TBondGatewayArgs) =>
  invokeWrapper<FeeDetails>('simulate_bond_gateway', args);

export const simulateUnbondGateway = async (args: any) => invokeWrapper<FeeDetails>('simulate_unbond_gateway', args);

export const simulateBondMixnode = async (args: TBondMixNodeArgs) =>
  invokeWrapper<FeeDetails>('simulate_bond_mixnode', args);

export const simulateUnbondMixnode = async (args: any) => invokeWrapper<FeeDetails>('simulate_unbond_mixnode', args);

export const simulateUpdateMixnode = async (args: any) => invokeWrapper<FeeDetails>('simulate_update_mixnode', args);

export const simulateDelegateToMixnode = async (args: { identity: string; amount: DecCoin }) =>
  invokeWrapper<FeeDetails>('simulate_delegate_to_mixnode', args);

export const simulateUndelegateFromMixnode = async (identity: string) =>
  invokeWrapper<FeeDetails>('simulate_undelegate_from_mixnode', { identity });

export const simulateCompoundDelgatorReward = async (identity: string) =>
  invokeWrapper<FeeDetails>('simulate_compound_delegator_reward', { mixIdentity: identity });

export const simulateClaimDelgatorReward = async (identity: string) =>
  invokeWrapper<FeeDetails>('simulate_claim_delegator_reward', { mixIdentity: identity });

export const simulateVestingClaimDelgatorReward = async (identity: string) =>
  invokeWrapper<FeeDetails>('simulate_vesting_claim_delegator_reward', { mixIdentity: identity });

export const simulateVestingCompoundDelgatorReward = async (identity: string) =>
  invokeWrapper<FeeDetails>('simulate_vesting_compound_delegator_reward', { mixIdentity: identity });

export const simulateVestingUndelegateFromMixnode = async (args: any) =>
  invokeWrapper<FeeDetails>('simulate_vesting_undelegate_from_mixnode', args);

export const simulateVestingBondGateway = async (args: { gateway: Gateway; pledge: DecCoin; ownerSignature: string }) =>
  invokeWrapper<FeeDetails>('simulate_vesting_bond_gateway', args);

export const simulateVestingUnbondGateway = async (args: any) =>
  invokeWrapper<FeeDetails>('simulate_vesting_unbond_gateway', args);

export const simulateVestingDelegateToMixnode = async (args: { identity: string }) =>
  invokeWrapper<FeeDetails>('simulate_vesting_delegate_to_mixnode', args);

export const simulateVestingBondMixnode = async (args: { mixnode: MixNode; pledge: DecCoin; ownerSignature: string }) =>
  invokeWrapper<FeeDetails>('simulate_vesting_bond_mixnode', args);

export const simulateVestingUnbondMixnode = async () => invokeWrapper<FeeDetails>('simulate_vesting_unbond_mixnode');

export const simulateVestingUpdateMixnode = async (args: any) =>
  invokeWrapper<FeeDetails>('simulate_vesting_update_mixnode', args);

export const simulateWithdrawVestedCoins = async (args: any) =>
  invokeWrapper<FeeDetails>('simulate_withdraw_vested_coins', args);

export const simulateSend = async ({ address, amount }: { address: string; amount: DecCoin }) =>
  invokeWrapper<FeeDetails>('simulate_send', { address, amount });

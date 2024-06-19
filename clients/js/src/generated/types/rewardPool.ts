/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  publicKey as publicKeySerializer,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  AccountType,
  AccountTypeArgs,
  RewardVault,
  RewardVaultArgs,
  getAccountTypeSerializer,
  getRewardVaultSerializer,
} from '.';

export type RewardPool = {
  accountType: AccountType;
  bump: number;
  totalShare: bigint;
  vault: RewardVault;
  depositAuthority: PublicKey;
  distributeAuthority: PublicKey;
  fillAuthority: PublicKey;
};

export type RewardPoolArgs = {
  accountType: AccountTypeArgs;
  bump: number;
  totalShare: number | bigint;
  vault: RewardVaultArgs;
  depositAuthority: PublicKey;
  distributeAuthority: PublicKey;
  fillAuthority: PublicKey;
};

export function getRewardPoolSerializer(): Serializer<
  RewardPoolArgs,
  RewardPool
> {
  return struct<RewardPool>(
    [
      ['accountType', getAccountTypeSerializer()],
      ['bump', u8()],
      ['totalShare', u64()],
      ['vault', getRewardVaultSerializer()],
      ['depositAuthority', publicKeySerializer()],
      ['distributeAuthority', publicKeySerializer()],
      ['fillAuthority', publicKeySerializer()],
    ],
    { description: 'RewardPool' }
  ) as Serializer<RewardPoolArgs, RewardPool>;
}
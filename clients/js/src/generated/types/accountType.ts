/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum AccountType {
  Uninitialized,
  RewardPool,
  Mining,
  RewardVault,
}

export type AccountTypeArgs = AccountType;

export function getAccountTypeSerializer(): Serializer<
  AccountTypeArgs,
  AccountType
> {
  return scalarEnum<AccountType>(AccountType, {
    description: 'AccountType',
  }) as Serializer<AccountTypeArgs, AccountType>;
}

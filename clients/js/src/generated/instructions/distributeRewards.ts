/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import {
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type DistributeRewardsInstructionAccounts = {
  /** The address of the reward pool */
  rewardPool: PublicKey | Pda;
  /** The address of the reward mint */
  rewardMint: PublicKey | Pda;
  /** The address of Authority who is eligble for distributiong rewards for users */
  distributeAuthority: Signer;
};

// Data.
export type DistributeRewardsInstructionData = { discriminator: number };

export type DistributeRewardsInstructionDataArgs = {};

export function getDistributeRewardsInstructionDataSerializer(): Serializer<
  DistributeRewardsInstructionDataArgs,
  DistributeRewardsInstructionData
> {
  return mapSerializer<
    DistributeRewardsInstructionDataArgs,
    any,
    DistributeRewardsInstructionData
  >(
    struct<DistributeRewardsInstructionData>([['discriminator', u8()]], {
      description: 'DistributeRewardsInstructionData',
    }),
    (value) => ({ ...value, discriminator: 7 })
  ) as Serializer<
    DistributeRewardsInstructionDataArgs,
    DistributeRewardsInstructionData
  >;
}

// Instruction.
export function distributeRewards(
  context: Pick<Context, 'programs'>,
  input: DistributeRewardsInstructionAccounts
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'mplxRewards',
    'BF5PatmRTQDgEKoXR7iHRbkibEEi83nVM38cUKWzQcTR'
  );

  // Accounts.
  const resolvedAccounts = {
    rewardPool: {
      index: 0,
      isWritable: true as boolean,
      value: input.rewardPool ?? null,
    },
    rewardMint: {
      index: 1,
      isWritable: false as boolean,
      value: input.rewardMint ?? null,
    },
    distributeAuthority: {
      index: 2,
      isWritable: false as boolean,
      value: input.distributeAuthority ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getDistributeRewardsInstructionDataSerializer().serialize({});

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}

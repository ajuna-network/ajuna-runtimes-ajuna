// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	weights, AccountId, Balance, BlockNumber, Council, OriginCaller, Runtime, RuntimeBlockWeights,
	RuntimeCall, RuntimeEvent, RuntimeOrigin, TechnicalCommittee, AJUN, DAYS,
};
use frame_support::{
	parameter_types,
	traits::{ConstBool, ConstU32, EitherOfDiverse},
	weights::Weight,
};
use frame_system::EnsureRoot;
use pallet_collective::{EnsureMember, EnsureProportionAtLeast, EnsureProportionMoreThan};
use sp_runtime::Perbill;

pub type EnsureRootOrMoreThanHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureProportionMoreThan<AccountId, CouncilCollectiveInstance, 1, 2>,
>;

pub type EnsureRootOrAllCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureProportionMoreThan<AccountId, CouncilCollectiveInstance, 1, 1>,
>;

pub type EnsureRootOrMoreThanHalfTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 2>,
>;

/// Council collective instance declaration.
///
/// The council primarily serves to optimize and balance the inclusive referendum system
/// by being allowed to propose external democracy proposals, which can be fast tracked and
/// bypass the one active referendum at a time rule.
///
/// It also controls the treasury.
type CouncilCollectiveInstance = pallet_collective::Instance2;

parameter_types! {
	pub CouncilMotionDuration: BlockNumber = 3 * DAYS;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
	pub CouncilMaxMembers: u32 = 100;
}

impl pallet_collective::Config<CouncilCollectiveInstance> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = ConstU32<100>;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
	type SetMembersOrigin = EnsureRootOrAllCouncil;
	type MaxProposalWeight = MaxProposalWeight;
}

/// Helper pallet to manage Council members.
type CouncilMembershipInstance = pallet_membership::Instance2;
impl pallet_membership::Config<CouncilMembershipInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrMoreThanHalfCouncil;
	type RemoveOrigin = EnsureRootOrAllCouncil;
	type SwapOrigin = EnsureRootOrAllCouncil;
	type ResetOrigin = EnsureRootOrAllCouncil;
	type PrimeOrigin = EnsureRootOrAllCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

/// The technical committee primarily serves to safeguard against malicious referenda
/// and fast track critical referenda.
pub type TechnicalCommitteeInstance = pallet_collective::Instance1;

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 3 * DAYS;
	pub const TechnicalCommitteeMaxMembers: u32 = 100;
}

impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = ConstU32<100>;
	type MaxMembers = TechnicalCommitteeMaxMembers;
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
	type SetMembersOrigin = EnsureRootOrAllCouncil;
	type MaxProposalWeight = MaxProposalWeight;
}

/// Helper pallet to manage TechnicalCommittee members.
type TechnicalCommitteeMembershipInstance = pallet_membership::Instance1;
impl pallet_membership::Config<TechnicalCommitteeMembershipInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRootOrMoreThanHalfCouncil;
	type RemoveOrigin = EnsureRootOrAllCouncil;
	type SwapOrigin = EnsureRootOrAllCouncil;
	type ResetOrigin = EnsureRootOrAllCouncil;
	type PrimeOrigin = EnsureRootOrAllCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = TechnicalCommitteeMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ThreeDays: BlockNumber = 3 * DAYS;
	pub const SevenDays: BlockNumber = 7 * DAYS;
	pub const FourteenDays: BlockNumber = 14 * DAYS;
	pub const TwentyEightDays: BlockNumber = 28 * DAYS;
	pub const EnactmentPeriod: BlockNumber = 7 * DAYS;
	pub const MinimumDeposit: Balance = 500 * AJUN;
}

impl pallet_democracy::Config for Runtime {
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = pallet_scheduler::Pallet<Runtime>;
	type Preimages = pallet_preimage::Pallet<Runtime>;
	type Currency = pallet_balances::Pallet<Runtime>;
	type EnactmentPeriod = EnactmentPeriod;
	// LaunchPeriod should not be shorter than VotingPeriod. Otherwise,
	// we risk that the ongoing referenda might infinitely grow.
	type LaunchPeriod = FourteenDays;
	type VotingPeriod = FourteenDays;
	type VoteLockingPeriod = EnactmentPeriod;
	type MinimumDeposit = MinimumDeposit;
	type InstantAllowed = ConstBool<true>;
	type FastTrackVotingPeriod = ThreeDays;
	type CooloffPeriod = TwentyEightDays;
	type MaxVotes = ConstU32<100>;
	type MaxProposals = ConstU32<100>;
	type MaxDeposits = ConstU32<100>;
	type MaxBlacklisted = ConstU32<100>;
	type ExternalOrigin = EnsureRootOrMoreThanHalfCouncil;
	type ExternalMajorityOrigin = EnsureRootOrMoreThanHalfCouncil;
	type ExternalDefaultOrigin = EnsureRootOrMoreThanHalfCouncil;
	type SubmitOrigin = frame_system::EnsureSigned<AccountId>;
	// Allows to fast track a proposal with `voting_period` > `FastTrackVotingPeriod`
	type FastTrackOrigin = EnsureRootOrMoreThanHalfTechnicalCommittee;
	// Allows to fast track a proposal with `voting_period` < `FastTrackVotingPeriod`
	type InstantOrigin = EnsureRootOrMoreThanHalfTechnicalCommittee;
	// To cancel a proposal that has passed.
	type CancellationOrigin = EnsureRootOrMoreThanHalfTechnicalCommittee;
	// Forever blacklist a proposal hash such that it can never be executed.
	type BlacklistOrigin = EnsureRootOrMoreThanHalfCouncil;
	// To cancel a proposal before it has passed, and slash its backers.
	type CancelProposalOrigin = EnsureRootOrMoreThanHalfCouncil;
	// Any single technical committee member may veto a coming council proposal. However, it lasts
	// only for the cool-off period, and then the proposal can be resubmitted again.
	type VetoOrigin = EnsureMember<AccountId, TechnicalCommitteeInstance>;
	type PalletsOrigin = OriginCaller;
	type Slash = pallet_treasury::Pallet<Runtime>;
}

use ethers::{
    contract::{builders::ContractCall, Contract, Lazy},
    core::{abi::Abi, types::*},
    providers::Middleware,
};
use std::sync::Arc;

pub static STAKING_ABI: Lazy<Abi> = Lazy::new(|| {
    serde_json::from_str("[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_apeCoinContractAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_baycContractAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_maycContractAddress\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"_bakcContractAddress\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"BAKCAlreadyPaired\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"BAKCNotOwnedOrPaired\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"CallerNotOwner\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"DepositMoreThanOneAPE\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"EndNotWholeHour\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ExceededCapAmount\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ExceededStakedAmount\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidPoolId\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MainTokenNotOwnedOrPaired\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NeitherTokenInPairOwnedByCaller\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NotOwnerOfBAKC\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NotOwnerOfMain\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ProvidedTokensNotPaired\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"SplitPairCantPartiallyWithdraw\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"StartMustBeGreaterThanEnd\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"StartMustEqualLastEnd\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"StartNotWholeHour\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UncommitWrongParameters\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"ClaimRewards\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"ClaimRewardsNft\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"bakcTokenId\",\"type\":\"uint256\"}],\"name\":\"ClaimRewardsPairNft\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"Deposit\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"DepositNft\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"bakcTokenId\",\"type\":\"uint256\"}],\"name\":\"DepositPairNft\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"OwnershipTransferred\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"lastRewardedBlock\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"accumulatedRewardsPerShare\",\"type\":\"uint256\"}],\"name\":\"UpdatePool\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"Withdraw\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"}],\"name\":\"WithdrawNft\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":true,\"internalType\":\"address\",\"name\":\"user\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"bakcTokenId\",\"type\":\"uint256\"}],\"name\":\"WithdrawPairNft\",\"type\":\"event\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_startTimestamp\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_endTimeStamp\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_capPerPosition\",\"type\":\"uint256\"}],\"name\":\"addTimeRange\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"name\":\"addressPosition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"internalType\":\"int256\",\"name\":\"rewardsDebt\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"apeCoin\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"bakcToMain\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"tokenId\",\"type\":\"uint248\"},{\"internalType\":\"bool\",\"name\":\"isPaired\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"claimApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint128\",\"name\":\"mainTokenId\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"bakcTokenId\",\"type\":\"uint128\"}],\"internalType\":\"struct ApeCoinStaking.PairNft[]\",\"name\":\"_baycPairs\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint128\",\"name\":\"mainTokenId\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"bakcTokenId\",\"type\":\"uint128\"}],\"internalType\":\"struct ApeCoinStaking.PairNft[]\",\"name\":\"_maycPairs\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"claimBAKC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nfts\",\"type\":\"uint256[]\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"claimBAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nfts\",\"type\":\"uint256[]\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"claimMAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"claimSelfApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint128\",\"name\":\"mainTokenId\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"bakcTokenId\",\"type\":\"uint128\"}],\"internalType\":\"struct ApeCoinStaking.PairNft[]\",\"name\":\"_baycPairs\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint128\",\"name\":\"mainTokenId\",\"type\":\"uint128\"},{\"internalType\":\"uint128\",\"name\":\"bakcTokenId\",\"type\":\"uint128\"}],\"internalType\":\"struct ApeCoinStaking.PairNft[]\",\"name\":\"_maycPairs\",\"type\":\"tuple[]\"}],\"name\":\"claimSelfBAKC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nfts\",\"type\":\"uint256[]\"}],\"name\":\"claimSelfBAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"_nfts\",\"type\":\"uint256[]\"}],\"name\":\"claimSelfMAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"depositApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"mainTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"bakcTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint184\",\"name\":\"amount\",\"type\":\"uint184\"}],\"internalType\":\"struct ApeCoinStaking.PairNftDepositWithAmount[]\",\"name\":\"_baycPairs\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"mainTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"bakcTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint184\",\"name\":\"amount\",\"type\":\"uint184\"}],\"internalType\":\"struct ApeCoinStaking.PairNftDepositWithAmount[]\",\"name\":\"_maycPairs\",\"type\":\"tuple[]\"}],\"name\":\"depositBAKC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"}],\"name\":\"depositBAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"}],\"name\":\"depositMAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"depositSelfApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getAllStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getApeCoinStake\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getBakcStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getBaycStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getMaycStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"getPoolsUI\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint48\",\"name\":\"startTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"endTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint96\",\"name\":\"rewardsPerHour\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"capPerPosition\",\"type\":\"uint96\"}],\"internalType\":\"struct ApeCoinStaking.TimeRange\",\"name\":\"currentTimeRange\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.PoolUI\",\"name\":\"\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint48\",\"name\":\"startTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"endTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint96\",\"name\":\"rewardsPerHour\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"capPerPosition\",\"type\":\"uint96\"}],\"internalType\":\"struct ApeCoinStaking.TimeRange\",\"name\":\"currentTimeRange\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.PoolUI\",\"name\":\"\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint48\",\"name\":\"startTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"endTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint96\",\"name\":\"rewardsPerHour\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"capPerPosition\",\"type\":\"uint96\"}],\"internalType\":\"struct ApeCoinStaking.TimeRange\",\"name\":\"currentTimeRange\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.PoolUI\",\"name\":\"\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint48\",\"name\":\"startTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"endTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint96\",\"name\":\"rewardsPerHour\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"capPerPosition\",\"type\":\"uint96\"}],\"internalType\":\"struct ApeCoinStaking.TimeRange\",\"name\":\"currentTimeRange\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.PoolUI\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"getSplitStakes\",\"outputs\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"tokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"deposited\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"unclaimed\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"rewards24hr\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"mainTokenId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"mainTypePoolId\",\"type\":\"uint256\"}],\"internalType\":\"struct ApeCoinStaking.DashboardPair\",\"name\":\"pair\",\"type\":\"tuple\"}],\"internalType\":\"struct ApeCoinStaking.DashboardStake[]\",\"name\":\"\",\"type\":\"tuple[]\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_index\",\"type\":\"uint256\"}],\"name\":\"getTimeRangeBy\",\"outputs\":[{\"components\":[{\"internalType\":\"uint48\",\"name\":\"startTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint48\",\"name\":\"endTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint96\",\"name\":\"rewardsPerHour\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"capPerPosition\",\"type\":\"uint96\"}],\"internalType\":\"struct ApeCoinStaking.TimeRange\",\"name\":\"\",\"type\":\"tuple\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"mainToBakc\",\"outputs\":[{\"internalType\":\"uint248\",\"name\":\"tokenId\",\"type\":\"uint248\"},{\"internalType\":\"bool\",\"name\":\"isPaired\",\"type\":\"bool\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"nftContracts\",\"outputs\":[{\"internalType\":\"contract ERC721Enumerable\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"nftPosition\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"stakedAmount\",\"type\":\"uint256\"},{\"internalType\":\"int256\",\"name\":\"rewardsDebt\",\"type\":\"int256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"_tokenId\",\"type\":\"uint256\"}],\"name\":\"pendingRewards\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"name\":\"pools\",\"outputs\":[{\"internalType\":\"uint48\",\"name\":\"lastRewardedTimestampHour\",\"type\":\"uint48\"},{\"internalType\":\"uint16\",\"name\":\"lastRewardsRangeIndex\",\"type\":\"uint16\"},{\"internalType\":\"uint96\",\"name\":\"stakedAmount\",\"type\":\"uint96\"},{\"internalType\":\"uint96\",\"name\":\"accumulatedRewardsPerShare\",\"type\":\"uint96\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"}],\"name\":\"removeLastTimeRange\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"renounceOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_from\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"_to\",\"type\":\"uint256\"}],\"name\":\"rewardsBy\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\"}],\"name\":\"stakedTotal\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\"}],\"name\":\"transferOwnership\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_poolId\",\"type\":\"uint256\"}],\"name\":\"updatePool\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"withdrawApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"mainTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"bakcTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint184\",\"name\":\"amount\",\"type\":\"uint184\"},{\"internalType\":\"bool\",\"name\":\"isUncommit\",\"type\":\"bool\"}],\"internalType\":\"struct ApeCoinStaking.PairNftWithdrawWithAmount[]\",\"name\":\"_baycPairs\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint32\",\"name\":\"mainTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint32\",\"name\":\"bakcTokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint184\",\"name\":\"amount\",\"type\":\"uint184\"},{\"internalType\":\"bool\",\"name\":\"isUncommit\",\"type\":\"bool\"}],\"internalType\":\"struct ApeCoinStaking.PairNftWithdrawWithAmount[]\",\"name\":\"_maycPairs\",\"type\":\"tuple[]\"}],\"name\":\"withdrawBAKC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"withdrawBAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"},{\"internalType\":\"address\",\"name\":\"_recipient\",\"type\":\"address\"}],\"name\":\"withdrawMAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_amount\",\"type\":\"uint256\"}],\"name\":\"withdrawSelfApeCoin\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"}],\"name\":\"withdrawSelfBAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"uint32\",\"name\":\"tokenId\",\"type\":\"uint32\"},{\"internalType\":\"uint224\",\"name\":\"amount\",\"type\":\"uint224\"}],\"internalType\":\"struct ApeCoinStaking.SingleNft[]\",\"name\":\"_nfts\",\"type\":\"tuple[]\"}],\"name\":\"withdrawSelfMAYC\",\"outputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]").expect("Failed to parse staking ABI")
});

#[derive(Clone)]
pub struct Staking<M>(Contract<M>);

impl<M> std::ops::Deref for Staking<M> {
    type Target = Contract<M>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: Middleware> std::fmt::Debug for Staking<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple(stringify!(Staking))
            .field(&self.address())
            .finish()
    }
}

impl<'a, M: Middleware> Staking<M> {
    #[doc = r" Creates a new contract instance with the specified `ethers`"]
    #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
    #[doc = r" object"]
    pub fn new<T: Into<Address>>(address: T, client: Arc<M>) -> Self {
        let contract = Contract::new(address.into(), STAKING_ABI.clone(), client);
        Self(contract)
    }
}

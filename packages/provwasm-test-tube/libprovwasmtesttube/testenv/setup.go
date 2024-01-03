package testenv

import (
	"cosmossdk.io/math"
	"encoding/json"
	"fmt"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	cryptocodec "github.com/cosmos/cosmos-sdk/crypto/codec"
	"github.com/cosmos/cosmos-sdk/testutil/mock"
	authtypes "github.com/cosmos/cosmos-sdk/x/auth/types"
	banktypes "github.com/cosmos/cosmos-sdk/x/bank/types"
	minttypes "github.com/cosmos/cosmos-sdk/x/mint/types"
	stakingkeeper "github.com/cosmos/cosmos-sdk/x/staking/keeper"
	"github.com/spf13/pflag"
	"github.com/tendermint/tendermint/libs/log"
	tmtypes "github.com/tendermint/tendermint/types"
	dbm "github.com/tendermint/tm-db"
	"time"

	// helpers

	abci "github.com/tendermint/tendermint/abci/types"

	// tendermint

	tmproto "github.com/tendermint/tendermint/proto/tendermint/types"

	// cosmos-sdk

	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	"github.com/cosmos/cosmos-sdk/server"
	sdk "github.com/cosmos/cosmos-sdk/types"
	slashingtypes "github.com/cosmos/cosmos-sdk/x/slashing/types"
	stakingtypes "github.com/cosmos/cosmos-sdk/x/staking/types"
	// provenance
	"github.com/provenance-io/provenance/app"
	"github.com/provenance-io/provenance/cmd/provenanced/config"
	//concentrateliquiditytypes "github.com/osmosis-labs/osmosis/v20/x/concentrated-liquidity/types"
	//gammtypes "github.com/osmosis-labs/osmosis/v20/x/gamm/types"
	//ibcratelimittypes "github.com/osmosis-labs/osmosis/v20/x/ibc-rate-limit/types"
	//incentivetypes "github.com/osmosis-labs/osmosis/v20/x/incentives/types"
	//lockuptypes "github.com/osmosis-labs/osmosis/v20/x/lockup/types"
	//minttypes "github.com/osmosis-labs/osmosis/v20/x/mint/types"
	//poolincentivetypes "github.com/osmosis-labs/osmosis/v20/x/pool-incentives/types"
	//poolmanagertypes "github.com/osmosis-labs/osmosis/v20/x/poolmanager/types"
	//protorevtypes "github.com/osmosis-labs/osmosis/v20/x/protorev/types"
	//superfluidtypes "github.com/osmosis-labs/osmosis/v20/x/superfluid/types"
	//tokenfactorytypes "github.com/osmosis-labs/osmosis/v20/x/tokenfactory/types"
	//twaptypes "github.com/osmosis-labs/osmosis/v20/x/twap/types"
)

type TestEnv struct {
	App                *app.App
	Ctx                sdk.Context
	ParamTypesRegistry ParamTypeRegistry
	Validator          []byte
	NodeHome           string
}

// DebugAppOptions is a stub implementing AppOptions
type DebugAppOptions struct{}

// Get implements AppOptions
func (ao DebugAppOptions) Get(o string) interface{} {
	if o == server.FlagTrace {
		return true
	}
	return nil
}

func SetupProvenanceApp(nodeHome string) (*app.App, []byte) {
	db := dbm.NewMemDB()

	app.SetConfig(true, false)

	provwasmFlags := pflag.NewFlagSet("provwasm-test-tube-flags", pflag.ContinueOnError)

	config.SetPioConfigFromFlags(provwasmFlags)

	appInstance := app.New(
		log.NewNopLogger(),
		db,
		nil,
		true,
		map[int64]bool{},
		nodeHome,
		5,
		app.MakeEncodingConfig(),
		DebugAppOptions{},
	)
	privVal := mock.NewPV()
	pubKey, err := privVal.GetPubKey()

	// create validator set with single validator
	validator := tmtypes.NewValidator(pubKey, 1)
	valSet := tmtypes.NewValidatorSet([]*tmtypes.Validator{validator})

	// generate genesis account
	senderPrivKey := secp256k1.GenPrivKey()
	acc := authtypes.NewBaseAccount(senderPrivKey.PubKey().Address().Bytes(), senderPrivKey.PubKey(), 0, 0)
	balances := []banktypes.Balance{
		{
			Address: acc.GetAddress().String(),
			Coins:   sdk.NewCoins(sdk.NewCoin(sdk.DefaultBondDenom, sdk.NewInt(100000000000000))),
		},
	}

	genesisState := app.NewDefaultGenesisState(appInstance.AppCodec())
	genAccs := []authtypes.GenesisAccount{acc}
	// set genesis accounts
	authGenesis := authtypes.NewGenesisState(authtypes.DefaultParams(), genAccs)
	genesisState[authtypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(authGenesis)

	validators := make([]stakingtypes.Validator, 0, len(valSet.Validators))
	delegations := make([]stakingtypes.Delegation, 0, len(valSet.Validators))

	bondAmt := sdk.DefaultPowerReduction

	for _, val := range valSet.Validators {
		pk, _ := cryptocodec.FromTmPubKeyInterface(val.PubKey)
		pkAny, _ := codectypes.NewAnyWithValue(pk)
		validator := stakingtypes.Validator{
			OperatorAddress:   sdk.ValAddress(val.Address).String(),
			ConsensusPubkey:   pkAny,
			Jailed:            false,
			Status:            stakingtypes.Bonded,
			Tokens:            bondAmt,
			DelegatorShares:   sdk.OneDec(),
			Description:       stakingtypes.Description{},
			UnbondingHeight:   int64(0),
			UnbondingTime:     time.Unix(0, 0).UTC(),
			Commission:        stakingtypes.NewCommission(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec()),
			MinSelfDelegation: sdk.ZeroInt(),
		}
		validators = append(validators, validator)
		delegations = append(delegations, stakingtypes.NewDelegation(genAccs[0].GetAddress(), val.Address.Bytes(), sdk.OneDec()))
	}

	// set validators and delegations
	stakingGenesis := stakingtypes.NewGenesisState(stakingtypes.DefaultParams(), validators, delegations)
	genesisState[stakingtypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(stakingGenesis)

	totalSupply := sdk.NewCoins()
	for _, b := range balances {
		// add genesis acc tokens to total supply
		totalSupply = totalSupply.Add(b.Coins...)
	}

	if len(delegations) > 0 {
		bondCoins := sdk.NewCoin(sdk.DefaultBondDenom, bondAmt.Mul(sdk.NewInt(int64(len(delegations)))))
		// add delegated tokens to total supply
		totalSupply = totalSupply.Add(bondCoins)
		// add bonded amount to bonded pool module account
		balances = append(balances, banktypes.Balance{
			Address: authtypes.NewModuleAddress(stakingtypes.BondedPoolName).String(),
			Coins:   sdk.Coins{bondCoins},
		})
	}

	// update total supply
	bankGenesis := banktypes.NewGenesisState(banktypes.DefaultGenesisState().Params, balances, totalSupply, []banktypes.Metadata{}, []banktypes.SendEnabled{})
	genesisState[banktypes.ModuleName] = appInstance.AppCodec().MustMarshalJSON(bankGenesis)

	stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	requireNoErr(err)

	// init chain will set the validator set and initialize the genesis accounts
	appInstance.InitChain(
		abci.RequestInitChain{
			Validators: []abci.ValidatorUpdate{},
			ConsensusParams: &abci.ConsensusParams{
				Block: &abci.BlockParams{
					MaxBytes: 200000,
					MaxGas:   60_000_000,
				},
				Evidence: &tmproto.EvidenceParams{
					MaxAgeNumBlocks: 302400,
					MaxAgeDuration:  504 * time.Hour, // 3 weeks is the max duration
					MaxBytes:        10000,
				},
				Validator: &tmproto.ValidatorParams{
					PubKeyTypes: []string{
						tmtypes.ABCIPubKeyTypeEd25519,
					},
				},
			},
			AppStateBytes: stateBytes,
			ChainId:       "testnet",
		},
	)

	// commit genesis changes
	appInstance.Commit()
	appInstance.BeginBlock(abci.RequestBeginBlock{Header: tmproto.Header{
		Height:             appInstance.LastBlockHeight() + 1,
		AppHash:            appInstance.LastCommitID().Hash,
		ValidatorsHash:     valSet.Hash(),
		NextValidatorsHash: valSet.Hash(),
		ChainID:            "testnet",
	}})
	//
	//// Set up Wasm genesis state
	//wasmGen := wasm.GenesisState{
	//	Params: wasmtypes.Params{
	//		// Allow store code without gov
	//		CodeUploadAccess:             wasmtypes.AllowEverybody,
	//		InstantiateDefaultPermission: wasmtypes.AccessTypeEverybody,
	//	},
	//}
	//genesisState[wasm.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&wasmGen)
	//
	//// Set up staking genesis state
	//stakingParams := stakingtypes.DefaultParams()
	//stakingParams.UnbondingTime = time.Hour * 24 * 7 * 2 // 2 weeks
	//stakingGen := stakingtypes.GenesisState{
	//	Params: stakingParams,
	//}
	//genesisState[stakingtypes.ModuleName] = encCfg.Marshaler.MustMarshalJSON(&stakingGen)
	//
	//stateBytes, err := json.MarshalIndent(genesisState, "", " ")
	//
	//requireNoErr(err)

	//concensusParams := simapp.DefaultConsensusParams
	//concensusParams.Block = &abci.BlockParams{
	//	MaxBytes: 22020096,
	//	MaxGas:   -1,
	//}

	// replace sdk.DefaultDenom with "nhash", a bit of a hack, needs improvement
	//stateBytes = []byte(strings.Replace(string(stateBytes), "\"stake\"", "\"nhash\"", -1))
	//
	//appInstance.InitChain(
	//	abci.RequestInitChain{
	//		Validators:      []abci.ValidatorUpdate{},
	//		ConsensusParams: concensusParams,
	//		AppStateBytes:   stateBytes,
	//	},
	//)

	return appInstance, privVal.PrivKey.Bytes()
}

func (env *TestEnv) BeginNewBlock(executeNextEpoch bool, timeIncreaseSeconds uint64) {
	var valAddr []byte

	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	if len(validators) >= 1 {
		valAddrFancy, err := validators[0].GetConsAddr()
		requireNoErr(err)
		valAddr = valAddrFancy.Bytes()
	} else {
		_, valAddrFancy := env.setupValidator(stakingtypes.Bonded)
		validator, _ := env.App.StakingKeeper.GetValidator(env.Ctx, valAddrFancy)
		valAddr2, _ := validator.GetConsAddr()
		valAddr = valAddr2.Bytes()
	}

	env.beginNewBlockWithProposer(valAddr, timeIncreaseSeconds)
}

func (env *TestEnv) GetValidatorAddresses() []string {
	validators := env.App.StakingKeeper.GetAllValidators(env.Ctx)
	var addresses []string
	for _, validator := range validators {
		addresses = append(addresses, validator.OperatorAddress)
	}

	return addresses
}

func (env *TestEnv) GetValidatorPrivateKey() []byte {
	return env.Validator
}

// beginNewBlockWithProposer begins a new block with a proposer.
func (env *TestEnv) beginNewBlockWithProposer(proposer sdk.ValAddress, timeIncreaseSeconds uint64) {
	validator, found := env.App.StakingKeeper.GetValidator(env.Ctx, proposer)

	if !found {
		panic("validator not found")
	}

	valConsAddr, err := validator.GetConsAddr()
	requireNoErr(err)

	valAddr := valConsAddr.Bytes()

	newBlockTime := env.Ctx.BlockTime().Add(time.Duration(timeIncreaseSeconds) * time.Second)
	header := tmproto.Header{ChainID: "testnet", Height: env.Ctx.BlockHeight() + 1, Time: newBlockTime}
	newCtx := env.Ctx.WithBlockTime(newBlockTime).WithBlockHeight(env.Ctx.BlockHeight() + 1)
	env.Ctx = newCtx
	lastCommitInfo := abci.LastCommitInfo{
		Votes: []abci.VoteInfo{{
			Validator:       abci.Validator{Address: valAddr, Power: 1000},
			SignedLastBlock: true,
		}},
	}
	reqBeginBlock := abci.RequestBeginBlock{Header: header, LastCommitInfo: lastCommitInfo}

	env.App.BeginBlock(reqBeginBlock)
	env.Ctx = env.App.NewContext(false, reqBeginBlock.Header)
}

func (env *TestEnv) setupValidator(bondStatus stakingtypes.BondStatus) (*secp256k1.PrivKey, sdk.ValAddress) {
	valPriv := secp256k1.GenPrivKey()
	valPub := valPriv.PubKey()
	valAddr := sdk.ValAddress(valPub.Address())
	valAccAddress := sdk.MustAccAddressFromBech32(valAddr.String())

	bondDenom := env.App.StakingKeeper.GetParams(env.Ctx).BondDenom
	selfBond := sdk.NewCoins(sdk.NewInt64Coin(bondDenom, 100))

	err := env.App.MarkerKeeper.WithdrawCoins(env.Ctx, valAccAddress, valAccAddress, "nhash", sdk.NewCoins(sdk.NewInt64Coin(bondDenom, 100000000000)))
	requireNoErr(err)

	stakingHandler := stakingkeeper.NewMsgServerImpl(env.App.StakingKeeper)
	stakingCoin := sdk.NewCoin(bondDenom, selfBond[0].Amount)

	ZeroCommission := stakingtypes.NewCommissionRates(sdk.ZeroDec(), sdk.ZeroDec(), sdk.ZeroDec())
	msg, err := stakingtypes.NewMsgCreateValidator(valAddr, valPub, stakingCoin, stakingtypes.Description{}, ZeroCommission, math.NewInt(100))
	requireNoErr(err)

	res, err := stakingHandler.CreateValidator(env.Ctx, msg)
	requireNoErr(err)

	requireNoNil("staking handler", res)

	env.App.BankKeeper.SendCoinsFromModuleToModule(env.Ctx, stakingtypes.NotBondedPoolName, stakingtypes.BondedPoolName, sdk.NewCoins(stakingCoin))

	val, found := env.App.StakingKeeper.GetValidator(env.Ctx, valAddr)
	requireTrue("validator found", found)

	val = val.UpdateStatus(bondStatus)
	env.App.StakingKeeper.SetValidator(env.Ctx, val)

	consAddr, err := val.GetConsAddr()
	requireNoErr(err)

	signingInfo := slashingtypes.NewValidatorSigningInfo(
		consAddr,
		env.Ctx.BlockHeight(),
		0,
		time.Unix(0, 0),
		false,
		0,
	)
	env.App.SlashingKeeper.SetValidatorSigningInfo(env.Ctx, consAddr, signingInfo)

	return valPriv, valAddr
}

func (env *TestEnv) SetupParamTypes() {
	pReg := env.ParamTypesRegistry

	//pReg.RegisterParamSet(&lockuptypes.Params{})
	//pReg.RegisterParamSet(&incentivetypes.Params{})
	pReg.RegisterParamSet(&minttypes.Params{})
	//pReg.RegisterParamSet(&twaptypes.Params{})
	//pReg.RegisterParamSet(&gammtypes.Params{})
	//pReg.RegisterParamSet(&ibcratelimittypes.Params{})
	//pReg.RegisterParamSet(&tokenfactorytypes.Params{})
	//pReg.RegisterParamSet(&superfluidtypes.Params{})
	//pReg.RegisterParamSet(&poolincentivetypes.Params{})
	//pReg.RegisterParamSet(&protorevtypes.Params{})
	//pReg.RegisterParamSet(&poolmanagertypes.Params{})
	//pReg.RegisterParamSet(&concentrateliquiditytypes.Params{})
}

func requireNoErr(err error) {
	if err != nil {
		panic(err)
	}
}

func requireNoNil(name string, nilable any) {
	if nilable == nil {
		panic(fmt.Sprintf("%s must not be nil", name))
	}
}

func requireTrue(name string, b bool) {
	if !b {
		panic(fmt.Sprintf("%s must be true", name))
	}
}

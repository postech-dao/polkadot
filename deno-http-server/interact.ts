import { Keyring } from "https://deno.land/x/polkadot@0.0.9/keyring/mod.ts";
import {
  ApiPromise,
  WsProvider,
} from "https://deno.land/x/polkadot@0.0.9/api/mod.ts";
import {
  BlueprintPromise,
  CodePromise,
  ContractPromise,
} from "https://deno.land/x/polkadot@0.0.9/api-contract/mod.ts";
import fs from "https://deno.land/std@0.115.1/node/fs/promises.ts";
import {
  assert,
  assertThrows,
} from "https://deno.land/std@0.149.0/testing/asserts.ts";
import { KeyringPair } from "https://deno.land/x/polkadot@0.0.9/keyring/types.ts";
import { SubmittableExtrinsic } from "https://deno.land/x/polkadot@0.0.9/api/submittable/types.ts";
import { CodeSubmittableResult } from "https://deno.land/x/polkadot@0.0.9/api-contract/base/Code.ts";
import { BlueprintSubmittableResult } from "https://deno.land/x/polkadot@0.0.9/api-contract/base/Blueprint.ts";
import { ApiTypes } from "https://deno.land/x/polkadot@0.0.9/api-base/types/index.ts";
import type {} from "https://deno.land/x/polkadot@0.0.9/api-augment/mod.ts";
import type { AnyJson } from "https://deno.land/x/polkadot@0.0.9/types-codec/types/index.ts";
import { ABI_PATH, ABI_PATH_OPTIONS } from "./abi.ts";
import {
  CONTRACT,
  LIGHT_CLIENT_QUERY,
  LIGHT_CLIENT_TX_METHOD,
  SIMPLE_COUNTER_QUERY,
  SIMPLE_COUNTER_TX,
  SIMPLE_COUNTER_TX_METHOD,
  TREASURY_QUERY,
  TREASURY_TX_METHOD,
} from "./contract.ts";
import { RPC_ENDPOINT, SS58_FORMAT, TESTNET_MNEMONIC } from "./enum.ts";

const toCamelCase = (str: string): string => {
  return str.toLowerCase().replace(
    /[^a-zA-Z0-9]+(.)/g,
    (m, chr) => chr.toUpperCase(),
  );
};

const toPascalCase = (str: string): string => {
  return `${str}`
    .toLowerCase()
    .replace(new RegExp(/[-_]+/, "g"), " ")
    .replace(new RegExp(/[^\w\s]/, "g"), "")
    .replace(
      new RegExp(/\s+(.)(\w*)/, "g"),
      ($1, $2, $3) => `${$2.toUpperCase() + $3}`,
    )
    .replace(new RegExp(/\w/), (s) => s.toUpperCase());
};

export const getPairFromSeed = (mnemonic: string): KeyringPair => {
  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  const pair: KeyringPair = keyring.addFromUri(seed);
  return pair;
};

export const getPairFromSeedWithSS58 = (
  mnemonic: string,
  format: SS58_FORMAT,
): KeyringPair => {
  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  keyring.setSS58Format(format);
  const pair: KeyringPair = keyring.addFromUri(seed);
  return pair;
};

// test: keypair
Deno.test({
  name: "keypair test",
  fn() {
    const keyring = new Keyring({ type: "sr25519" });
    const shibuyaPair: KeyringPair = keyring.addFromUri(
      TESTNET_MNEMONIC.SHIBUYA,
    );
    keyring.setSS58Format(SS58_FORMAT.SHIBUYA);
    assert(
      shibuyaPair.address === "YtyhRxkUA5gAPsFXQzQKdexK4GUCaiDqk8RrQtU4FiwNYHY",
    );
    keyring.setSS58Format(SS58_FORMAT.DEFAULT);
    assert(
      shibuyaPair.address ===
        getPairFromSeedWithSS58(TESTNET_MNEMONIC.SHIBUYA, SS58_FORMAT.DEFAULT)
          .address,
    );
    const rococoPair = getPairFromSeed(TESTNET_MNEMONIC.ROCOCO);
    assert(
      rococoPair.address === "5CiTGDb8zaMMw6Sqrn8y3Awt9A6HiEdyf3wB7GrsbnpasVss",
    );
    assert(
      rococoPair.address ===
        getPairFromSeedWithSS58(TESTNET_MNEMONIC.ROCOCO, SS58_FORMAT.DEFAULT)
          .address,
    );
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

const getFreeBalance = async (
  fullNodeUri: string,
  address: string,
): Promise<bigint> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const { data: balance } = await api.query.system.account(address);
  const freeBalance = BigInt(balance?.free.toHuman().replace(/\,/g, ""));
  // 1 ROC = 1,000,000,000,000,000, 1 SBY = 1,000,000,000,000,000,000
  return freeBalance;
};

export const getReservedBalance = async (
  fullNodeUri: string,
  address: string,
): Promise<bigint> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const { data: balance } = await api.query.system.account(address);
  const reservedBalance = BigInt(
    balance?.reserved.toHuman().replace(/\,/g, ""),
  );
  // 1 ROC = 1,000,000,000,000,000, 1 SBY = 1,000,000,000,000,000,000
  return reservedBalance;
};

export const getTotalBalance = async (
  fullNodeUri: string,
  address: string,
): Promise<bigint> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const { data: balance } = await api.query.system.account(address);
  const freeBalance = BigInt(balance?.free.toHuman().replace(/\,/g, "")); // 1 ROC = 1,000,000,000,000,000
  const reservedBalance = BigInt(
    balance?.reserved.toHuman().replace(/\,/g, ""),
  ); // 1 ROC = 1,000,000,000,000,000, 1 SBY = 1,000,000,000,000,000,000
  return freeBalance + reservedBalance;
};

// test: balance
// check floating point error
Deno.test({
  name: "balance test",
  async fn() {
    const pair: KeyringPair = getPairFromSeed(TESTNET_MNEMONIC.SHIBUYA);
    const free = await getFreeBalance(
      RPC_ENDPOINT.SHIBUYA,
      pair.address,
    );
    const reserved = await getReservedBalance(
      RPC_ENDPOINT.SHIBUYA,
      pair.address,
    );
    const total = await getTotalBalance(
      RPC_ENDPOINT.SHIBUYA,
      pair.address,
    );
    assert(free + reserved === total);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

export const getAbiFromContractName = async (name: string): Promise<string> => {
  let path: ABI_PATH_OPTIONS;
  switch (name) {
    case CONTRACT.SIMPLE_COUNTER:
      path = ABI_PATH.SIMPLE_COUNTER;
      break;
    case CONTRACT.LIGHT_CLIENT:
      path = ABI_PATH.LIGHT_CLIENT;
      break;
    case CONTRACT.TREASURY:
      path = ABI_PATH.TREASURY;
      break;
    default:
      throw new Error("The contract name is invalid");
  }
  return await fs.readFile(path, "utf8");
};

export const query = async (
  fullNodeUri: string,
  contractName: string,
  contractAddr: string,
  field: string,
): Promise<AnyJson> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const abi: string = await getAbiFromContractName(contractName);
  const PDAO_TEST_ADDR = "5CiTGDb8zaMMw6Sqrn8y3Awt9A6HiEdyf3wB7GrsbnpasVss";
  const gasLimit: bigint = 30000n * 1000000n;
  const storageDepositLimit = null;

  const contract: ContractPromise = new ContractPromise(api, abi, contractAddr);
  let res: AnyJson;
  let messageName: SIMPLE_COUNTER_QUERY | LIGHT_CLIENT_QUERY | TREASURY_QUERY;
  switch (contractName) {
    case CONTRACT.SIMPLE_COUNTER:
      messageName = "get" + toPascalCase(field) as SIMPLE_COUNTER_QUERY;
      break;
    case CONTRACT.LIGHT_CLIENT:
      messageName = "get" + toPascalCase(field) as LIGHT_CLIENT_QUERY;
      break;
    case CONTRACT.TREASURY:
      messageName = "get" + toPascalCase(field) as TREASURY_QUERY;
      break;
    default:
      throw new Error("contract name is invalid");
  }
  const { gasRequired, storageDeposit, result, output } = await contract
    .query[messageName](PDAO_TEST_ADDR, { gasLimit, storageDepositLimit });
  console.log("storageDeposit: ", storageDeposit.toHuman());
  console.log("gasRequire: ", gasRequired.toHuman());
  console.log("result: ", result.toHuman());
  console.log("output: ", output?.toHuman());
  if (output && output.toHuman()) {
    res = output.toHuman();
  } else {
    throw new Error("output is invalid");
  }
  return res;
};

export const sendContractTx = async (
  fullNodeUri: string,
  mnemonic: string,
  contractName: string,
  contractAddr: string,
  methodName: string,
  methodParams: any[],
): Promise<string> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const abi: string = await getAbiFromContractName(contractName);
  const pair: KeyringPair = getPairFromSeed(mnemonic);
  const gasLimit: bigint = 30000n * 1000000n;
  const storageDepositLimit = null;
  const contract: ContractPromise = new ContractPromise(api, abi, contractAddr);
  let messageName:
    | SIMPLE_COUNTER_TX_METHOD
    | LIGHT_CLIENT_TX_METHOD
    | TREASURY_TX_METHOD;
  switch (contractName) {
    case CONTRACT.SIMPLE_COUNTER:
      messageName = toCamelCase(methodName) as SIMPLE_COUNTER_TX_METHOD;
      break;
    case CONTRACT.LIGHT_CLIENT:
      messageName = toCamelCase(methodName) as LIGHT_CLIENT_TX_METHOD;
      break;
    case CONTRACT.TREASURY:
      messageName = toCamelCase(methodName) as TREASURY_TX_METHOD;
      break;
    default:
      throw new Error("contract name is invalid");
  }
  let _txHash: string | undefined = undefined;
  await contract.tx[messageName](
    { storageDepositLimit, gasLimit },
    ...methodParams,
  )
    .signAndSend(pair, (result) => {
      if (result.status.isInBlock) {
        console.log("in a block");
        const { txHash } = result;
        _txHash = txHash.toString();
      } else if (result.status.isFinalized) {
        console.log("finalized");
      }
    });
  return new Promise((res, rej) => {
    let count = 0;
    const MAX_COUNT = 240;
    const timer = setInterval(() => {
      count++;
      if (_txHash !== undefined) {
        res(_txHash);
        clearInterval(timer);
      } else if (count > MAX_COUNT) {
        rej(new Error("Timeout: over 120 seconds"));
      }
    }, 500);
  });
};

// contract test: simple_counter; execute method
Deno.test({
  name: "simple_counter query and tx test: execute",
  async fn() {
    // Many transactions between queries can make it fail
    const SIMPLE_COUNTER_ADDR =
      "Xt1CVcr4nTd3oKrPk85xJWLTCMwGZa6KyxGo2kTGf2NjzLf";
    const INPUT = 5;
    const prevCount = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    const txHash = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "execute",
      [INPUT],
    );
    console.log("txHash: ", txHash);
    const subsequentCount = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    if (subsequentCount && prevCount) {
      assert(
        parseInt(subsequentCount?.toString()) -
            parseInt(prevCount?.toString()) === INPUT,
      );
    }
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

// contract test: simple_counter; increment & decrement method
Deno.test({
  name: "simple_counter query and tx test: increment & decrement",
  async fn() {
    const SIMPLE_COUNTER_ADDR =
      "Xt1CVcr4nTd3oKrPk85xJWLTCMwGZa6KyxGo2kTGf2NjzLf";
    // increment
    const prevCount = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    const txHash = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "increment",
      [],
    );
    console.log("txHash: ", txHash);
    const countAfterInc = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    // assert(parseInt(countAfterInc) - parseInt(prevCount) === 1);

    //decrement
    const txHash2 = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "decrement",
      [],
    );
    console.log("txHash2: ", txHash2);
    const countAfterDec = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    if (countAfterInc && prevCount) {
      assert(
        parseInt(countAfterInc?.toString()) -
            parseInt(prevCount?.toString()) === 1,
      );
    }
    if (countAfterDec && prevCount) {
      assert(
        parseInt(countAfterDec?.toString()) === parseInt(prevCount?.toString()),
      );
    }
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

// contract test: simple_counter; reset
Deno.test({
  name: "simple_counter query and tx test: reset",
  async fn() {
    const SIMPLE_COUNTER_ADDR =
      "Xt1CVcr4nTd3oKrPk85xJWLTCMwGZa6KyxGo2kTGf2NjzLf";
    const txHash = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "increment",
      [],
    );
    console.log("txHash: ", txHash);
    const txHash2 = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "reset",
      [],
    );
    console.log("txHash2: ", txHash2);
    const countAfterReset = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "count",
    );
    if (countAfterReset) assert(parseInt(countAfterReset.toString()) === 0);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

// contract test: simple_counter; addAuth and removeAuth
Deno.test({
  name: "simple_counter query and tx test: addAuth and removeAuth",
  async fn() {
    const SIMPLE_COUNTER_ADDR =
      "Xt1CVcr4nTd3oKrPk85xJWLTCMwGZa6KyxGo2kTGf2NjzLf";
    const AUTH_ADDR = "YtUkPWDB1thp87L9UeYUwx9nWNYv9JtvFihRzUWrnZ3j7zm";
    const prevList = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "auth",
    );
    const prevAuthList = prevList?.toString().split(",");
    const txHash = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "add_auth",
      [AUTH_ADDR],
    );
    console.log("txHash", txHash);
    const listAfterAdd = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "auth",
    );
    const authListAfterAdd = listAfterAdd?.toString().split(",");
    const txHash2 = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "remove_auth",
      [AUTH_ADDR],
    );
    console.log("txHash2", txHash2);
    const listAfterRemove = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      SIMPLE_COUNTER_ADDR,
      "auth",
    );
    const authListAfterRemove = listAfterRemove?.toString().split(",");
    console.log("prev: ", prevAuthList);
    console.log("after add: ", authListAfterAdd);
    console.log("after remove: ", authListAfterRemove);
    if (authListAfterAdd && prevAuthList) {
      assert(authListAfterAdd?.length - prevAuthList?.length === 1);
    }
    if (authListAfterAdd && authListAfterRemove) {
      assert(authListAfterAdd?.length - authListAfterRemove?.length === 1);
    }
    authListAfterAdd && prevAuthList
      ? assert(authListAfterAdd[prevAuthList.length] === AUTH_ADDR)
      : assert(false, "addAuth does not occur");
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

export type contractDeploymentResult = {
  contractAddr: string;
  txHash: string;
};

export const deployWithContractName = async (
  fullNodeUri: string,
  mnemonic: string,
  contractName: string,
  params: any[],
): Promise<contractDeploymentResult> => {
  const provider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: provider });
  const abi: string = await getAbiFromContractName(contractName);
  const wasm: string = JSON.parse(abi).source.wasm;
  const code: CodePromise = new CodePromise(api, abi, wasm);

  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  const pair: KeyringPair = keyring.addFromUri(seed);

  const gasLimit = 100000n * 1000000n;
  const storageDepositLimit: number | null = null;
  const tx: SubmittableExtrinsic<"promise", CodeSubmittableResult<ApiTypes>> =
    code.tx.new({ gasLimit, storageDepositLimit }, ...params);
  let address: string | undefined = undefined;
  let _txhash: string | undefined = undefined;
  const unsub = await tx.signAndSend(pair, ({ contract, status, txHash }) => {
    if (status.isInBlock || status.isFinalized) {
      address = contract?.address.toString();
      console.log("contract address : ", address);
      _txhash = txHash.toString();
      unsub();
    }
  });
  return new Promise((res, rej) => {
    let count = 0;
    const MAX_COUNT = 120;
    const timer = setInterval(() => {
      count++;
      if (address !== undefined && _txhash !== undefined) {
        res({ contractAddr: address, txHash: _txhash });
        clearInterval(timer);
      } else if (count > MAX_COUNT) {
        rej(new Error("Timeout: over 60 seconds"));
      }
    }, 500);
  });
};

export const deployWithCodeHash = async (
  fullNodeUri: string,
  mnemonic: string,
  contractName: string,
  salt: string | null,
  params: any[],
): Promise<contractDeploymentResult> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const abi: string = await getAbiFromContractName(contractName);
  const { source: { hash } } = JSON.parse(abi);
  const blueprint: BlueprintPromise = new BlueprintPromise(api, abi, hash);

  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  const pair: KeyringPair = keyring.addFromUri(seed);

  const gasLimit = 100000n * 1000000n;
  const storageDepositLimit: number | null = null;
  const tx: SubmittableExtrinsic<
    "promise",
    BlueprintSubmittableResult<ApiTypes>
  > = blueprint.tx.new({ gasLimit, storageDepositLimit, salt }, ...params);

  let address: string | undefined = undefined;
  let _txHash: string | undefined = undefined;
  const unsub = await tx.signAndSend(pair, ({ contract, status, txHash }) => {
    if (status.isInBlock || status.isFinalized) {
      address = contract?.address.toString();
      console.log("contract address : ", address);
      _txHash = txHash.toString();
      unsub();
    }
  });
  return new Promise((res, rej) => {
    let count = 0;
    const MAX_COUNT = 120;
    const timer = setInterval(() => {
      count++;
      if (address !== undefined && _txHash !== undefined) {
        res({ contractAddr: address, txHash: _txHash });
        clearInterval(timer);
      } else if (count > MAX_COUNT) {
        rej(new Error("Timeout: over 60 seconds"));
      }
    }, 500);
  });
};

// deployment test
Deno.test({
  name: "deployment test: deploy simple_counter; init",
  async fn() {
    // init with deploy
    const INIT_COUNT = 100;
    const { contractAddr, txHash: deployTxHash } = await deployWithContractName(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      [INIT_COUNT],
    );
    console.log("contract address: ", contractAddr);
    console.log("deployment tx hash: ", deployTxHash);
    const countResult: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "count",
    );
    let count: number | null = null;
    if (countResult?.toString()) count = parseInt(countResult.toString());
    assert(count === INIT_COUNT);

    // execute init method after deploy
    const INIT_COUNT_2 = 500;
    const FIRST_AUTH_ADDR =
      getPairFromSeedWithSS58(TESTNET_MNEMONIC.SHIBUYA, SS58_FORMAT.SHIBUYA)
        .address;
    const contractTxHash: string = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      SIMPLE_COUNTER_TX.INIT,
      [INIT_COUNT_2, FIRST_AUTH_ADDR],
    );
    console.log("contract tx hash: ", contractTxHash);
    const countResult2: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "count",
    );
    let count2: number | null = null;
    if (countResult2?.toString()) count2 = parseInt(countResult2.toString());
    assert(count2 === INIT_COUNT_2);

    // check first auth
    const authResult: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "auth",
    );
    let authList: string[] = [];
    if (authResult?.toString()) {
      authList = [...authResult.toString().split(",")];
    }
    assert(authList[0] === FIRST_AUTH_ADDR);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

// deployment test with code hash
Deno.test({
  name: "deployment test: deploy simple_counter with code hash; init",
  async fn() {
    // init with deploy
    const INIT_COUNT = 100;
    const salt = "aaaaaaaa"; // change this for every test
    const { contractAddr, txHash: deployTxHash } = await deployWithCodeHash(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      salt,
      [INIT_COUNT],
    );
    console.log("contract address: ", contractAddr);
    console.log("deployment tx hash: ", deployTxHash);
    const countResult: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "count",
    );
    let count: number | null = null;
    if (countResult?.toString()) count = parseInt(countResult.toString());
    assert(count === INIT_COUNT);

    // execute init method after deploy
    const INIT_COUNT_2 = 500;
    const FIRST_AUTH_ADDR =
      getPairFromSeedWithSS58(TESTNET_MNEMONIC.SHIBUYA, SS58_FORMAT.SHIBUYA)
        .address;
    const contractTxHash: string = await sendContractTx(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      SIMPLE_COUNTER_TX.INIT,
      [INIT_COUNT_2, FIRST_AUTH_ADDR],
    );
    console.log("contract tx hash: ", contractTxHash);
    const countResult2: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "count",
    );
    let count2: number | null = null;
    if (countResult2?.toString()) count2 = parseInt(countResult2.toString());
    assert(count2 === INIT_COUNT_2);

    // check first auth
    const authResult: AnyJson = await query(
      RPC_ENDPOINT.SHIBUYA,
      CONTRACT.SIMPLE_COUNTER,
      contractAddr,
      "auth",
    );
    let authList: string[] = [];
    if (authResult?.toString()) {
      authList = [...authResult.toString().split(",")];
    }
    assert(authList[0] === FIRST_AUTH_ADDR);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

export type BlockInfo = {
  blockHash: string;
  timestamp: number;
};

export const getBlockInfo = async (
  fullNodeUri: string,
  blockNumber: number,
): Promise<BlockInfo> => {
  const provider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
  const signedBlock = await api.rpc.chain.getBlock(blockHash);
  const { block: { extrinsics } } = signedBlock;
  const ex = extrinsics[0]; // rococo-contracts: extrinsic[1]
  const { args } = ex;
  const timestamp = parseInt(args[0].toString());
  const blockInfo = { blockHash: blockHash.toHex(), timestamp };
  return blockInfo;
};

export const getCurrentHeight = async (
  fullNodeUri: string,
): Promise<number> => {
  const provider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const { block: { header: { number } } } = await api.rpc.chain.getBlock();
  return parseInt(number.toString());
};

// block info test
Deno.test({
  name: "block info test: get block hash and timestamp from blocknumber",
  async fn() {
    const blockNumber = 2066556;
    const { blockHash, timestamp } = await getBlockInfo(
      RPC_ENDPOINT.SHIBUYA,
      blockNumber,
    );
    assert(
      blockHash ===
        "0x842dadc2b3afec1fa95b97e41cd0271201248dca5f9cd891e124e5e15f619d44",
    );
    assert(timestamp === 1660894626045);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

export const transferNativeToken = async (
  fullNodeUri: string,
  mnemonic: string,
  to: string,
  amountInUnits: bigint,
): Promise<string> => {
  const provider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const pair: KeyringPair = getPairFromSeed(mnemonic);
  const transfer = api.tx.balances.transfer(to, amountInUnits);
  const hash = await transfer.signAndSend(pair);
  return hash.toHex();
};

// token transfer test
Deno.test({
  name: "token transfer test: transfer native token in shibuya",
  async fn() {
    const AMOUNT = 100000000000000000n; // 0.1 SBY
    const receiverAddr: string =
      getPairFromSeedWithSS58(TESTNET_MNEMONIC.ROCOCO, SS58_FORMAT.SHIBUYA)
        .address;
    const prevBalance: bigint = await getTotalBalance(
      RPC_ENDPOINT.SHIBUYA,
      receiverAddr,
    );
    console.log(prevBalance);
    const txHash: string = await transferNativeToken(
      RPC_ENDPOINT.SHIBUYA,
      TESTNET_MNEMONIC.SHIBUYA,
      receiverAddr,
      AMOUNT,
    );
    console.log("txHash: ", txHash);
    let balanceAfterTransfer: bigint | null = null;
    setTimeout(async () => {
      balanceAfterTransfer = await getTotalBalance(
        RPC_ENDPOINT.SHIBUYA,
        receiverAddr,
      );
      console.log(balanceAfterTransfer);
      assert(balanceAfterTransfer - prevBalance === AMOUNT);
    }, 3000);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

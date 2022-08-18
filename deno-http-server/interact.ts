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
import type { AnyJson } from "https://deno.land/x/polkadot@0.0.9/types-codec/types/index.ts";
import fs from "https://deno.land/std@0.115.1/node/fs/promises.ts";
import {
  assert,
  assertEquals,
} from "https://deno.land/std@0.149.0/testing/asserts.ts";
import { KeyringPair } from "https://deno.land/x/polkadot@0.0.9/keyring/types.ts";
import { SubmittableExtrinsic } from "https://deno.land/x/polkadot@0.0.9/api/submittable/types.ts";
import { CodeSubmittableResult } from "https://deno.land/x/polkadot@0.0.9/api-contract/base/Code.ts";
import { BlueprintSubmittableResult } from "https://deno.land/x/polkadot@0.0.9/api-contract/base/Blueprint.ts";
import { ApiTypes } from "https://deno.land/x/polkadot@0.0.9/api-base/types/index.ts";
import type {} from "https://deno.land/x/polkadot@0.0.9/api-augment/mod.ts";
import { ABI_PATH, ABI_PATH_OPTIONS } from "./abi.ts";
import {
  CONTRACT,
  LIGHT_CLIENT_QUERY,
  LIGHT_CLIENT_TX_METHOD,
  SIMPLE_COUNTER_QUERY,
  SIMPLE_COUNTER_TX_METHOD,
  TREASURY_QUERY,
  TREASURY_TX_METHOD,
} from "./contract.ts";
import { getBlock } from "https://deno.land/x/polkadot@0.0.9/api-derive/chain/getBlock.ts";

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

// get free balance
// const getFreeBalance = async (address: string): Promise<bigint> => {
//   const wsProvider: WsProvider = new WsProvider(
//     "wss://rococo-contracts-rpc.polkadot.io:443",
//   );
//   const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
//   const { data: balance } = await api.query.system.account(address);
//   const freeBalance = BigInt(balance?.free.toHuman().replace(/\,/g, "")); // 1 DOT = 1,000,000,000,000,000
//   // console.log(freeBalance);
//   return freeBalance;
// };

// get reserved balance
// const getReservedBalance = async (address: string): Promise<bigint> => {
//   const wsProvider: WsProvider = new WsProvider(
//     "wss://rococo-contracts-rpc.polkadot.io:443",
//   );
//   const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
//   const { data: balance } = await api.query.system.account(address);
//   const reservedBalance = BigInt(
//     balance?.reserved.toHuman().replace(/\,/g, ""),
//   ); // 1 DOT = 1,000,000,000,000,000
//   // console.log(reservedBalance);
//   return reservedBalance;
// };

// get total balance
export const getTotalBalance = async (
  fullNodeUri: string,
  address: string,
): Promise<bigint> => {
  const wsProvider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const { data: balance } = await api.query.system.account(address);
  const freeBalance = BigInt(balance?.free.toHuman().replace(/\,/g, "")); // 1 ROC = 1,000,000,000,000,000
  const reservedBalance = BigInt(
    balance?.reserved.toHuman().replace(/\,/g, ""),
  ); // 1 ROC = 1,000,000,000,000,000, 1 SBY = 1,000,000,000,000,000,000
  // console.log(freeBalance);
  // console.log(reservedBalance);
  // console.log(freeBalance + reservedBalance);
  return freeBalance + reservedBalance;
};
// const balance = await getTotalBalance("wss://shibuya-rpc.dwellir.com:443", "YtyhRxkUA5gAPsFXQzQKdexK4GUCaiDqk8RrQtU4FiwNYHY")
// console.log(balance);

// Deno.test({
//   name: "balance test",
//   async fn() {
//     const pair: KeyringPair = getTestPair();
//     const free = await getFreeBalance(pair.address);
//     const reserved = await getReservedBalance(pair.address);
//     const total = await getTotalBalance(
//       "wss://rococo-contracts-rpc.polkadot.io:443",
//       pair.address,
//     );
//     assert(total === (free + reserved));
//   },
//   sanitizeResources: false,
//   sanitizeOps: false,
// });

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
): Promise<string> => {
  const provider: WsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider });
  const abi: string = await getAbiFromContractName(contractName);
  const PDAO_TEST_ADDR = "5CiTGDb8zaMMw6Sqrn8y3Awt9A6HiEdyf3wB7GrsbnpasVss";
  const gasLimit: bigint = 30000n * 1000000n;
  const storageDepositLimit = null;

  const contract: ContractPromise = new ContractPromise(api, abi, contractAddr);
  let res: string;
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
  console.log("output", output?.toHuman());
  if (output && output.toString()) {
    res = output.toString();
  } else {
    throw new Error("output is invalid");
  }
  return res;
};
// const testQuery = await query("wss://shibuya-rpc.dwellir.com:443", "simple_counter", "Xt1CVcr4nTd3oKrPk85xJWLTCMwGZa6KyxGo2kTGf2NjzLf", "count");
// console.log(testQuery);

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
  const storageDepositLimit = 10000000; // 수정해야할 수도 있음
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
  let hash: string | undefined = undefined;
  await contract.tx[messageName](
    { storageDepositLimit, gasLimit },
    ...methodParams,
  )
    .signAndSend(pair, (result) => {
      if (result.status.isInBlock) {
        console.log("in a block");
        // console.log("in a block : ", result);
      } else if (result.status.isFinalized) {
        console.log("finalized");
        // console.log("finalized : ", result);
        const { txHash } = result;
        hash = txHash.toString();
      }
    });
  return new Promise((res, rej) => {
    let count = 0;
    const MAX_COUNT = 120;
    const timer = setInterval(() => {
      count++;
      if (hash !== undefined) {
        res(hash);
        clearInterval(timer);
      } else if (count > MAX_COUNT) {
        rej(new Error("Timeout: over 60 seconds"));
      }
    }, 500);
  });
};
// const testContractTx = await sendContractTx(
//   "wss://rococo-contracts-rpc.polkadot.io:443",
//   "where sign course conduct include wide because skull boss slice close tomorrow",
//   "simple_counter",
//   "5DDD98nuSmshMy1RjFCoM4vy44VqU5EdgVPJHagmefpMGkr5",
//   "execute",
//   ["5"],
// );
// console.log('txHash: ', testContractTx);
type contractDeploymentInfo = {
  contractAddr: string;
  txHash: string;
};

export const deployWithContractName = async (
  fullNodeUri: string,
  mnemonic: string,
  contractName: string,
): Promise<contractDeploymentInfo> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const abi: string = await getAbiFromContractName(contractName);
  const wasm: string = JSON.parse(abi).source.wasm;
  const code: CodePromise = new CodePromise(api, abi, wasm);

  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  const pair: KeyringPair = keyring.addFromUri(seed);

  const initCount = 10;
  const gasLimit = 100000n * 1000000n;
  const storageDepositLimit: number | null = null;
  const tx: SubmittableExtrinsic<"promise", CodeSubmittableResult<ApiTypes>> =
    code.tx.new({ gasLimit, storageDepositLimit }, initCount);
  let address: string | undefined = undefined;
  let hash: string | undefined = undefined;
  const unsub = await tx.signAndSend(pair, ({ contract, status, txHash }) => {
    if (status.isInBlock || status.isFinalized) {
      address = contract?.address.toString();
      console.log("contract address : ", address);
      hash = txHash.toString();
      unsub();
    }
  });
  return new Promise((res, rej) => {
    let count = 0;
    const MAX_COUNT = 120;
    const timer = setInterval(() => {
      count++;
      if (address !== undefined && hash !== undefined) {
        res({ contractAddr: address, txHash: hash });
        clearInterval(timer);
      } else if (count > MAX_COUNT) {
        rej(new Error("Timeout: over 60 seconds"));
      }
    }, 500);
  });
};

// const deployTest = await deployWithContractName("wss://rococo-contracts-rpc.polkadot.io:443", 'where sign course conduct include wide because skull boss slice close tomorrow', 'simple_counter');
// console.log("deployed contract address: ", deployTest);

export const deployWithCodeHash = async (
  fullNodeUri: string,
  mnemonic: string,
  contractName: string,
  salt: string | null,
): Promise<contractDeploymentInfo> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const abi: string = await getAbiFromContractName(contractName);
  const { source: { hash } } = JSON.parse(abi);
  const blueprint: BlueprintPromise = new BlueprintPromise(api, abi, hash);

  const seed: string = mnemonic;
  const keyring: Keyring = new Keyring({ type: "sr25519" });
  const pair: KeyringPair = keyring.addFromUri(seed);

  const initCount = 10;
  const gasLimit = 100000n * 1000000n;
  const storageDepositLimit: number | null = null;
  const tx: SubmittableExtrinsic<
    "promise",
    BlueprintSubmittableResult<ApiTypes>
  > = blueprint.tx.new({ gasLimit, storageDepositLimit, salt }, initCount);

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
// const bpTest = await deployWithCodeHash(
//   "wss://rococo-contracts-rpc.polkadot.io:443",
//   "니모닉",
//   "simple_counter",
//   "10",
// );
// console.log(bpTest);

const listenToNewBlock = async () => {
  const wsProvider = new WsProvider(
    "wss://rococo-contracts-rpc.polkadot.io:443",
  );
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  let count = 0;
  const unsubscribe = await api.rpc.chain.subscribeNewHeads((header) => {
    console.log(`Chain is at block: #${header.number}`);

    if (++count === 2) {
      unsubscribe();
      Deno.exit(0);
    }
  });
};
// await listenToNewBlock();

export type BlockInfo = {
  blockHash: string;
  timestamp: number;
};

export const getBlockInfo = async (
  fullNodeUri: string,
  blockNumber: number,
): Promise<BlockInfo> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
  const signedBlock = await api.rpc.chain.getBlock(blockHash);
  const { block: { extrinsics } } = signedBlock;
  const ex = extrinsics[0];
  const { args } = ex;
  const timestamp = parseInt(args[0].toString());
  const blockInfo = { blockHash: blockHash.toHex(), timestamp };
  return blockInfo;
};
const test = await getBlockInfo("wss://shibuya-rpc.dwellir.com:443", 2046251);
console.log(test);


export const getCurrentHeight = async (
  fullNodeUri: string,
): Promise<number> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const { block: { header: { number } } } = await api.rpc.chain.getBlock();
  return parseInt(number.toString());
};
// await getCurrentHeight();

export const transferNativeToken = async (
  fullNodeUri: string,
  mnemonic: string,
  to: string,
  amountInUnits: number,
): Promise<string> => {
  const wsProvider = new WsProvider(fullNodeUri);
  const api: ApiPromise = await ApiPromise.create({ provider: wsProvider });
  const pair: KeyringPair = getPairFromSeed(mnemonic);
  const transfer = api.tx.balances.transfer(to, amountInUnits);
  const hash = await transfer.signAndSend(pair);
  // console.log(hash);
  return hash.toHex();
};
// transfer 0.1 ROC
// await transferNativeToken("wss://rococo-contracts-rpc.polkadot.io:443", '니모닉', '5CiR2BqPzn1yEVUMLPFvdiiAzJVPkr4qE2DPrGx3KBzaQ6rz', 100000000000);
// let test = BigInt("123456789123456789")
// console.log(test.toString());

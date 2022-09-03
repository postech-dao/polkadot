import { Keyring } from "https://deno.land/x/polkadot@0.0.9/keyring/mod.ts";
import { assert } from "https://deno.land/std@0.154.0/testing/asserts.ts";
import { KeyringPair } from "https://deno.land/x/polkadot@0.0.9/keyring/types.ts";
import type {} from "https://deno.land/x/polkadot@0.0.9/api-augment/mod.ts";
import type { AnyJson } from "https://deno.land/x/polkadot@0.0.9/types-codec/types/index.ts";
import { CONTRACT, SIMPLE_COUNTER_TX } from "./contract.ts";
import { RPC_ENDPOINT, SS58_FORMAT, TESTNET_MNEMONIC } from "./enum.ts";
import {
  deployWithCodeHash,
  deployWithContractName,
  getBlockInfo,
  getFreeBalance,
  getPairFromSeed,
  getPairFromSeedWithSS58,
  getReservedBalance,
  getTotalBalance,
  query,
  sendContractTx,
  transferNativeToken,
} from "./interact.ts";

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
      assert(
        balanceAfterTransfer !== null &&
          balanceAfterTransfer - prevBalance === AMOUNT,
      );
    }, 3000);
  },
  sanitizeResources: false,
  sanitizeOps: false,
});

import * as oak from "https://deno.land/x/oak@v10.6.0/mod.ts";
import { Application, Router } from "https://deno.land/x/oak/mod.ts";
import {
  BlockInfo,
  deployWithCodeHash,
  deployWithContractName,
  getBlockInfo,
  getCurrentHeight,
  getPairFromSeed,
  getTotalBalance,
  query,
  sendContractTx,
  transferNativeToken,
} from "./interact.ts";
import { KeyringPair } from "https://deno.land/x/polkadot@0.0.9/keyring/types.ts";
import type { AnyJson } from "https://deno.land/x/polkadot@0.0.9/types-codec/types/index.ts";

const port = 8080;
const app = new Application();
const router = new Router();

router.get("/", (ctx) => {
  ctx.response.body = "PDAO http server";
});

// complete
router.post("/current-height", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const height: number = await getCurrentHeight(reqBody.fullNodeUri);
    ctx.response.body = {
      success: true,
      data: {
        height,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// complete
router.post("/block-info", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const block: BlockInfo = await getBlockInfo(
      reqBody.fullNodeUri,
      reqBody.height,
    );
    const { blockHash, timestamp } = block;
    ctx.response.body = {
      success: true,
      data: {
        blockHash,
        timestamp,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// complete
router.post("/account-info", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const totalBalance: bigint = await getTotalBalance(
      reqBody.fullNodeUri,
      reqBody.addr,
    );
    const nativeToken = totalBalance.toString();
    ctx.response.body = {
      success: true,
      data: {
        nativeToken,
        memeToken: "0",
        nonFungibleToken: "0",
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

router.post("/contract-state", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const output: string = await query(
      reqBody.fullNodeUri,
      reqBody.contractName,
      reqBody.contractAddr,
      reqBody.field,
    );
    ctx.response.body = {
      success: true,
      data: {
        contractName: reqBody.contractName,
        messageName: "get_" + reqBody.field,
        messageType: "query",
        output,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// tx
router.post("/native-token/transfer", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const amount: bigint = reqBody.amount;
    if (amount > Number.MAX_SAFE_INTEGER) {
      throw new Error("The number of amount exceeds MAX_VALUE");
    }
    const amountInPlanck = Number(amount);
    const txHash: string = await transferNativeToken(
      reqBody.fullNodeUri,
      reqBody.mnemonic,
      reqBody.toAddr,
      amountInPlanck,
    );

    ctx.response.body = {
      success: true,
      data: {
        txHash,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// tx
router.post("/contract-method/execute", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const params = [...reqBody.arguments];
    const txHash: string = await sendContractTx(
      reqBody.fullNodeUri,
      reqBody.mnemonic,
      reqBody.contractName,
      reqBody.contractAddr,
      reqBody.methodName,
      params,
    );
    ctx.response.body = {
      success: true,
      data: {
        contractName: reqBody.contractName,
        messageName: reqBody.methodName,
        messageType: "tx",
        txHash,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// tx
router.post("/contract/deploy", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    const { contractAddr, txHash } = await deployWithContractName(
      reqBody.fullNodeUri,
      reqBody.mnemonic,
      reqBody.contractName,
    );
    ctx.response.body = {
      success: true,
      data: {
        contractName: reqBody.contractName,
        contractAddr,
        txHash,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

// tx
router.post("/contract-from-code-hash/deploy", async (ctx) => {
  try {
    if (!ctx.request.hasBody) ctx.throw(415);
    const reqBody = await ctx.request.body().value;
    let salt: string | null;
    reqBody.salt === "null" ? salt = null : salt = reqBody.salt;
    if (reqBody.salt === null) salt = null;
    const { contractAddr, txHash } = await deployWithCodeHash(
      reqBody.fullNodeUri,
      reqBody.mnemonic,
      reqBody.contractName,
      salt,
    );
    ctx.response.body = {
      success: true,
      data: {
        contractName: reqBody.contractName,
        contractAddr,
        txHash,
      },
    };
  } catch (err) {
    ctx.response.body = {
      success: false,
      msg: err.toString(),
    };
  }
});

app.use(router.allowedMethods());
app.use(router.routes());

app.listen({ port });

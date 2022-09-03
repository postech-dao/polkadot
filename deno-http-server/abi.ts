export const ABI_PATH = {
  SIMPLE_COUNTER: "./deno-http-server/contracts/simple_counter.contract",
  LIGHT_CLIENT: "./deno-http-server/contracts/light_client.contract",
  TREASURY: "./deno-http-server/contracts/treasury",
} as const;

export type ABI_PATH_OPTIONS = typeof ABI_PATH[keyof typeof ABI_PATH];

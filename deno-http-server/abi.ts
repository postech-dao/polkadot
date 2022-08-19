export const ABI_PATH = {
  SIMPLE_COUNTER: "./contracts/simple_counter.contract",
  LIGHT_CLIENT: "./contracts/light_client.contract",
  TREASURY: "./contracts/treasury",
} as const;

export type ABI_PATH_OPTIONS = typeof ABI_PATH[keyof typeof ABI_PATH];
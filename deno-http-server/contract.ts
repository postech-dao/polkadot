export const CONTRACT = {
    SIMPLE_COUNTER: "simple_counter",
    LIGHT_CLIENT: "light_client",
    TREASURY: "treasury",
  } as const;
  export type CONTRACT_OPTIONS = typeof CONTRACT[keyof typeof CONTRACT];
  
  export const SIMPLE_COUNTER = {
    GET_COUNT: "getCount",
    GET_AUTH: "getAuth",
  } as const;
  export type SIMPLE_COUNTER_QUERY =
    typeof SIMPLE_COUNTER[keyof typeof SIMPLE_COUNTER];
  
  export const SIMPLE_COUNTER_TX = {
    INIT: "init",
    EXECUTE: "execute",
    ADD_AUTH: "addAuth",
    REMOVE_AUTH: "removeAuth",
    INCREMENT: "increment",
    DECREMENT: "decrement",
    RESET: "reset",
  } as const;
  export type SIMPLE_COUNTER_TX_METHOD =
    typeof SIMPLE_COUNTER_TX[keyof typeof SIMPLE_COUNTER_TX];
  
  export const LIGHT_CLIENT = {} as const;
  export type LIGHT_CLIENT_QUERY = typeof LIGHT_CLIENT[keyof typeof LIGHT_CLIENT];
  
  export const LIGHT_CLIENT_TX = {} as const;
  export type LIGHT_CLIENT_TX_METHOD =
    typeof LIGHT_CLIENT_TX[keyof typeof LIGHT_CLIENT_TX];
  
  export const TREASURY = {} as const;
  export type TREASURY_QUERY = typeof TREASURY[keyof typeof TREASURY];
  
  export const TREASURY_TX = {} as const;
  export type TREASURY_TX_METHOD = typeof TREASURY_TX[keyof typeof TREASURY_TX];

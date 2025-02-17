/* tslint:disable */
/* eslint-disable */
/**
 * Initialize Javascript logging and panic handler
 */
export function solana_program_init(): void;
/**
 * A (twisted) ElGamal encryption keypair.
 *
 * The instances of the secret key are zeroized on drop.
 */
export class ElGamalKeypair {
  private constructor();
  free(): void;
  /**
   * Generates the public and secret keys for ElGamal encryption.
   *
   * This function is randomized. It internally samples a scalar element using `OsRng`.
   */
  static new_rand(): ElGamalKeypair;
  pubkey_owned(): ElGamalPubkey;
}
/**
 * Public key for the ElGamal encryption scheme.
 */
export class ElGamalPubkey {
  private constructor();
  free(): void;
}
/**
 * A hash; the 32-byte output of a hashing algorithm.
 *
 * This struct is used most often in `solana-sdk` and related crates to contain
 * a [SHA-256] hash, but may instead contain a [blake3] hash.
 *
 * [SHA-256]: https://en.wikipedia.org/wiki/SHA-2
 * [blake3]: https://github.com/BLAKE3-team/BLAKE3
 */
export class Hash {
  free(): void;
  /**
   * Create a new Hash object
   *
   * * `value` - optional hash as a base58 encoded string, `Uint8Array`, `[number]`
   */
  constructor(value: any);
  /**
   * Return the base58 string representation of the hash
   */
  toString(): string;
  /**
   * Checks if two `Hash`s are equal
   */
  equals(other: Hash): boolean;
  /**
   * Return the `Uint8Array` representation of the hash
   */
  toBytes(): Uint8Array;
}
/**
 * wasm-bindgen version of the Instruction struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Instruction struct.
 */
export class Instruction {
  private constructor();
  free(): void;
}
export class Instructions {
  free(): void;
  constructor();
  push(instruction: Instruction): void;
}
export class JSOwner {
  private constructor();
  free(): void;
}
/**
 * A vanilla Ed25519 key pair
 */
export class Keypair {
  free(): void;
  /**
   * Create a new `Keypair `
   */
  constructor();
  /**
   * Convert a `Keypair` to a `Uint8Array`
   */
  toBytes(): Uint8Array;
  /**
   * Recover a `Keypair` from a `Uint8Array`
   */
  static fromBytes(bytes: Uint8Array): Keypair;
  /**
   * Return the `Pubkey` for this `Keypair`
   */
  pubkey(): Pubkey;
}
/**
 * wasm-bindgen version of the Message struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Message struct.
 */
export class Message {
  private constructor();
  free(): void;
  /**
   * The id of a recent ledger entry.
   */
  recent_blockhash: Hash;
}
/**
 * The `ElGamalPubkey` type as a `Pod`.
 */
export class PodElGamalPubkey {
  free(): void;
  /**
   * Create a new `PodElGamalPubkey` object
   *
   * * `value` - optional public key as a base64 encoded string, `Uint8Array`, `[number]`
   */
  constructor(value: any);
  /**
   * Return the base64 string representation of the public key
   */
  toString(): string;
  /**
   * Checks if two `ElGamalPubkey`s are equal
   */
  equals(other: PodElGamalPubkey): boolean;
  /**
   * Return the `Uint8Array` representation of the public key
   */
  toBytes(): Uint8Array;
  static compressed(decoded: ElGamalPubkey): PodElGamalPubkey;
  decompressed(): ElGamalPubkey;
}
/**
 * The address of a [Solana account][acc].
 *
 * Some account addresses are [ed25519] public keys, with corresponding secret
 * keys that are managed off-chain. Often, though, account addresses do not
 * have corresponding secret keys &mdash; as with [_program derived
 * addresses_][pdas] &mdash; or the secret key is not relevant to the operation
 * of a program, and may have even been disposed of. As running Solana programs
 * can not safely create or manage secret keys, the full [`Keypair`] is not
 * defined in `solana-program` but in `solana-sdk`.
 *
 * [acc]: https://solana.com/docs/core/accounts
 * [ed25519]: https://ed25519.cr.yp.to/
 * [pdas]: https://solana.com/docs/core/cpi#program-derived-addresses
 * [`Keypair`]: https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html
 */
export class Pubkey {
  free(): void;
  /**
   * Create a new Pubkey object
   *
   * * `value` - optional public key as a base58 encoded string, `Uint8Array`, `[number]`
   */
  constructor(value: any);
  /**
   * Return the base58 string representation of the public key
   */
  toString(): string;
  /**
   * Check if a `Pubkey` is on the ed25519 curve.
   */
  isOnCurve(): boolean;
  /**
   * Checks if two `Pubkey`s are equal
   */
  equals(other: Pubkey): boolean;
  /**
   * Return the `Uint8Array` representation of the public key
   */
  toBytes(): Uint8Array;
  /**
   * Derive a Pubkey from another Pubkey, string seed, and a program id
   */
  static createWithSeed(base: Pubkey, seed: string, owner: Pubkey): Pubkey;
  /**
   * Derive a program address from seeds and a program id
   */
  static createProgramAddress(seeds: any[], program_id: Pubkey): Pubkey;
  /**
   * Find a valid program address
   *
   * Returns:
   * * `[PubKey, number]` - the program address and bump seed
   */
  static findProgramAddress(seeds: any[], program_id: Pubkey): any;
}
/**
 * wasm-bindgen version of the Transaction struct.
 * This duplication is required until https://github.com/rustwasm/wasm-bindgen/issues/3671
 * is fixed. This must not diverge from the regular non-wasm Transaction struct.
 */
export class Transaction {
  free(): void;
  /**
   * Create a new `Transaction`
   */
  constructor(instructions: Instructions, payer?: Pubkey);
  /**
   * Return a message containing all data that should be signed.
   */
  message(): Message;
  /**
   * Return the serialized message data to sign.
   */
  messageData(): Uint8Array;
  /**
   * Verify the transaction
   */
  verify(): void;
  partialSign(keypair: Keypair, recent_blockhash: Hash): void;
  isSigned(): boolean;
  toBytes(): Uint8Array;
  static fromBytes(bytes: Uint8Array): Transaction;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: (a: number, b: number) => number;
  readonly __wbg_jsowner_free: (a: number, b: number) => void;
  readonly __wbg_hash_free: (a: number, b: number) => void;
  readonly hash_constructor: (a: any) => [number, number, number];
  readonly hash_toString: (a: number) => [number, number];
  readonly hash_equals: (a: number, b: number) => number;
  readonly hash_toBytes: (a: number) => [number, number];
  readonly __wbg_instruction_free: (a: number, b: number) => void;
  readonly __wbg_message_free: (a: number, b: number) => void;
  readonly __wbg_get_message_recent_blockhash: (a: number) => number;
  readonly __wbg_set_message_recent_blockhash: (a: number, b: number) => void;
  readonly __wbg_instructions_free: (a: number, b: number) => void;
  readonly instructions_constructor: () => number;
  readonly instructions_push: (a: number, b: number) => void;
  readonly systeminstruction_createAccount: (a: number, b: number, c: bigint, d: bigint, e: number) => number;
  readonly systeminstruction_createAccountWithSeed: (a: number, b: number, c: number, d: number, e: number, f: bigint, g: bigint, h: number) => number;
  readonly systeminstruction_assign: (a: number, b: number) => number;
  readonly systeminstruction_assignWithSeed: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly systeminstruction_transfer: (a: number, b: number, c: bigint) => number;
  readonly systeminstruction_transferWithSeed: (a: number, b: number, c: number, d: number, e: number, f: number, g: bigint) => number;
  readonly systeminstruction_allocate: (a: number, b: bigint) => number;
  readonly systeminstruction_allocateWithSeed: (a: number, b: number, c: number, d: number, e: bigint, f: number) => number;
  readonly systeminstruction_createNonceAccount: (a: number, b: number, c: number, d: bigint) => any;
  readonly systeminstruction_advanceNonceAccount: (a: number, b: number) => number;
  readonly systeminstruction_withdrawNonceAccount: (a: number, b: number, c: number, d: bigint) => number;
  readonly systeminstruction_authorizeNonceAccount: (a: number, b: number, c: number) => number;
  readonly pubkey_constructor: (a: any) => [number, number, number];
  readonly pubkey_toString: (a: number) => [number, number];
  readonly pubkey_isOnCurve: (a: number) => number;
  readonly pubkey_createWithSeed: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly pubkey_createProgramAddress: (a: number, b: number, c: number) => [number, number, number];
  readonly pubkey_findProgramAddress: (a: number, b: number, c: number) => [number, number, number];
  readonly __wbg_keypair_free: (a: number, b: number) => void;
  readonly __wbg_transaction_free: (a: number, b: number) => void;
  readonly keypair_constructor: () => number;
  readonly keypair_toBytes: (a: number) => [number, number];
  readonly keypair_fromBytes: (a: number, b: number) => [number, number, number];
  readonly keypair_pubkey: (a: number) => number;
  readonly transaction_constructor: (a: number, b: number) => number;
  readonly transaction_message: (a: number) => number;
  readonly transaction_messageData: (a: number) => [number, number];
  readonly transaction_verify: (a: number) => [number, number];
  readonly transaction_partialSign: (a: number, b: number, c: number) => void;
  readonly transaction_isSigned: (a: number) => number;
  readonly transaction_toBytes: (a: number) => [number, number];
  readonly transaction_fromBytes: (a: number, b: number) => [number, number, number];
  readonly __wbg_elgamalkeypair_free: (a: number, b: number) => void;
  readonly elgamalkeypair_new_rand: () => number;
  readonly elgamalkeypair_pubkey_owned: (a: number) => number;
  readonly __wbg_elgamalpubkey_free: (a: number, b: number) => void;
  readonly podelgamalpubkey_constructor: (a: any) => [number, number, number];
  readonly podelgamalpubkey_toString: (a: number) => [number, number];
  readonly podelgamalpubkey_compressed: (a: number) => number;
  readonly podelgamalpubkey_decompressed: (a: number) => [number, number, number];
  readonly solana_program_init: () => void;
  readonly pubkey_equals: (a: number, b: number) => number;
  readonly podelgamalpubkey_equals: (a: number, b: number) => number;
  readonly pubkey_toBytes: (a: number) => [number, number];
  readonly podelgamalpubkey_toBytes: (a: number) => [number, number];
  readonly __wbg_pubkey_free: (a: number, b: number) => void;
  readonly __wbg_podelgamalpubkey_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_4: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_export_7: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly closure121_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure129_externref_shim: (a: number, b: number, c: any) => void;
  readonly _ZN132__LT_dyn_u20_core__ops__function__FnMut_LT__LP__RP__GT__u2b_Output_u20__u3d__u20_R_u20_as_u20_wasm_bindgen__closure__WasmClosure_GT_8describe6invoke17h2e5a831b63380ba5E: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

# Whitelist Anchor Program - programs\hello_anchor

This markdown provides a detailed guide for installing, using, and understanding the whitelist Solana program. This program implements a whitelist for token sales, initializes a token with metadata, and allows for minting tokens to whitelisted addresses.

## Installation Process

### Prerequisites

Ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solanalabs.com/cli/install)
- [Anchor](https://www.anchor-lang.com/docs/installation)

### Clone the Repository

Clone the repository containing the `hello_anchor` program:

```shell
git clone https://github.com/donjne/whitelist_program.git
cd hello_anchor
```

### Build the Project

Build the project using anchor:

```shell
anchor build
```

### Deploy the project

Build the project using anchor:

```shell
anchor deploy
```

## Usage Instructions

### Initialize the Whitelist

To initialize the whitelist, call the init_whitelist instruction:

```shell
anchor test --program-name hello_anchor
```

### Initialize the Token

To initialize the whitelist, call the init_token instruction:

```shell
anchor test --program-name hello_anchor --args '{"name": "TokenName", "symbol": "TKN", "uri": "https://example.com/metadata.json", "decimals": 0}'
```

### Mint Tokens

To mint tokens to a whitelisted address, call the mint_tokens instruction:

```shell
anchor test --program-name hello_anchor --args '{"quantity": 10}'
```

## Transactions

### [initialize_whitelist](https://explorer.solana.com/tx/3o6qzQ6GHryCL5u4H5h2a8ufL5CSraep9hYgAXmjtqp5KT3mPD2x4bHBVuLZALVmBXgBL1SNvkeNEUXwuMvw4nb1?cluster=devnet)

### [initialize_token](https://explorer.solana.com/tx/5gSPAkeuGsvf9E3iBg4TrZS2Pz4kX4iKGJbs1QvKTQZVMx4LSbbVAdr1icq3T2tVWmB7tGL9y6FMtftmbo8dVUb2?cluster=devnet)

## Program Instructions

### `init_whitelist`

Initializes the whitelist program account with the following parameters:

- `whitelist_addresses`: Empty list of whitelisted addresses.
- `is_whitelist_finalized`: False.
- `is_sale_active`: False.
- `token_price`: The price of the token.
- `max_tokens_per_address`: Maximum tokens allowed per address.
- `total_tokens_sold`: Total number of tokens sold.
- `token_balance_keys`: Empty list of token balance keys.
- `token_balance_values`: Empty list of token balance values.

### `init_token`

Initializes a token mint with the provided metadata:

- `name`: Name of the token.
- `symbol`: Symbol of the token.
- `uri`: URI of the metadata.
- `decimals`: Number of decimals for the token.

### `mint_tokens`

Mints tokens to a specified destination account if the sale is active and the address is whitelisted:

- `quantity`: Number of tokens to mint.

## Error Codes

The program defines the following error codes:

- `WhitelistFinalized`: Whitelist is already finalized.
- `AlreadyWhitelisted`: Address is already whitelisted.
- `NotWhitelisted`: Address is not whitelisted.
- `SaleNotActive`: Token sale is not active.
- `MaxTokensExceeded`: Exceeded maximum tokens per address.

## Accounts

### `WhitelistProgram`

Stores the state of the whitelist program:

- `whitelist_addresses`: List of whitelisted addresses.
- `is_whitelist_finalized`: Boolean indicating if the whitelist is finalized.
- `is_sale_active`: Boolean indicating if the sale is active.
- `token_price`: Price of the token.
- `max_tokens_per_address`: Maximum tokens allowed per address.
- `total_tokens_sold`: Total number of tokens sold.
- `token_balance_keys`: List of token balance keys.
- `token_balance_values`: List of token balance values.

### `InitTokenParams`

Parameters for initializing the token:

- `name`: Name of the token.
- `symbol`: Symbol of the token.
- `uri`: URI of the metadata.
- `decimals`: Number of decimals for the token.

### `InitWhitelist`

Accounts required for initializing the whitelist:

- `payer`: Signer who pays for the transaction.
- `whitelist_program`: Account to store the whitelist program state.
- `system_program`: System program.
- `rent`: Rent sysvar.

### `InitToken`

Accounts required for initializing the token:

- `metadata`: Unchecked account for the metadata.
- `mint`: Account for the token mint.
- `payer`: Signer who pays for the transaction.
- `whitelist_program`: Account to store the whitelist program state.
- `rent`: Rent sysvar.
- `system_program`: System program.
- `token_program`: Token program.
- `token_metadata_program`: Token metadata program.

### `MintTokens`

Accounts required for minting tokens:

- `mint`: Account for the token mint.
- `destination`: Token account to receive the minted tokens.
- `payer`: Signer who pays for the transaction.
- `whitelist_program`: Account to store the whitelist program state.
- `rent`: Rent sysvar.
- `system_program`: System program.
- `token_program`: Token program.
- `associated_token_program`: Associated token program.

import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { HelloAnchor } from "../target/types/hello_anchor";
describe("whitelist", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HelloAnchor as anchor.Program<HelloAnchor>;
  
  // Initializing a new whitelist
  const tokenPrice = new anchor.BN(10);
  it("Is initialized!", async () => {
    // Add your test here.
    const WHITELIST_SEED = "whitelist";
    const [whitelist] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(WHITELIST_SEED)],
      program.programId
    );
    const tx = await program.methods.initWhitelist(tokenPrice).accounts({
      whitelistProgram: whitelist,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  //Initializing a new mint:

    // Metaplex Constants
    const METADATA_SEED = "metadata";
    const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // Constants from our program
    const MINT_SEED = "mint";
  
    // Data for our tests
    const payer = program.provider.publicKey;
    const metadata = {
      name: "Just a Test Token",
      symbol: "TEST",
      uri: "https://5vfxc4tr6xoy23qefqbj4qx2adzkzapneebanhcalf7myvn5gzja.arweave.net/7UtxcnH13Y1uBCwCnkL6APKsge0hAgacQFl-zFW9NlI",
      decimals: 9,
    };
    const mintAmount = 10;
    const [mint] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      program.programId
    );

    const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );
    const WHITELIST_SEED = "whitelist";
    const [whitelist] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(WHITELIST_SEED)],
      program.programId
    );
    // Test init token
    it("initialize", async () => {
      const info = await program.provider.connection.getAccountInfo(mint);
      if (info) {
        return; // Do not attempt to initialize if already initialized
      }
      console.log("  Mint not found. Attempting to initialize.");
      
      const context = {
        metadata: metadataAddress,
        mint,
        payer,
        whitelistProgram: whitelist,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      };
  
      const txHash = await program.methods
        .initToken(metadata)
        .accounts(context)
        .rpc();
  
      await program.provider.connection.confirmTransaction(txHash, 'finalized');
      console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
      const newInfo = await program.provider.connection.getAccountInfo(mint);
      assert(newInfo, "  Mint should be initialized.");
    });
});
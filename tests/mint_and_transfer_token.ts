import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { token } from "@project-serum/anchor/dist/cjs/utils";
import { MintAndTransferToken } from "../target/types/mint_and_transfer_token";

describe("mint_and_transfer_token", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .MintAndTransferToken as Program<MintAndTransferToken>;
  const wallet = anchor.AnchorProvider.env().wallet;
  const [authority_pda, bump] = anchor.utils.publicKey.findProgramAddressSync(
    [wallet.publicKey.toBuffer(), Buffer.from("authority")],
    program.programId
  );
  const firstPerson = anchor.web3.Keypair.generate();
  const token_mint = anchor.web3.Keypair.generate();
  const token_account_firstperson = await anchor.utils.token.associatedAddress({
    mint: token_mint.publicKey,
    owner: firstPerson.publicKey,
  });

  const token_accout_program = await anchor.utils.token.associatedAddress({
    mint: token_mint.publicKey,
    owner: authority_pda,
  });

  const pos_mint = anchor.web3.Keypair.generate();
  const pos_account_first_person = await anchor.utils.token.associatedAddress({
    mint: pos_mint.publicKey,
    owner: firstPerson.publicKey,
  });

  const pos_account_program = await anchor.utils.token.associatedAddress({
    mint: pos_mint.publicKey,
    owner: authority_pda,
  });

  it("Mint and Transfer!", async () => {
    // Add your test here.
    const tx = await program.methods
      .mintAndTransfer()
      .accounts({
        wallet: wallet.publicKey,
        mint: token_mint.publicKey,
        firstPerson: firstPerson.publicKey,
        firstPersonTokenAccount: token_account_firstperson,
        authorityPda: authority_pda,
        posMint: pos_mint.publicKey,
        posProgramAccount: pos_account_program,
        programTokenAccount: token_accout_program,
        firstPersonPosAccount: pos_account_first_person,
      })
      .signers([token_mint, pos_mint, firstPerson])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Stake Tokens!", async () => {
    // Add your test here.
    const tx = await program.methods
      .stakeToken(new anchor.BN(20))
      .accounts({
        wallet: wallet.publicKey,
        mint: token_mint.publicKey,
        firstPerson: firstPerson.publicKey,
        firstPersonTokenAccount: token_account_firstperson,
        authorityPda: authority_pda,
        posMint: pos_mint.publicKey,
        posProgramAccount: pos_account_program,
        programTokenAccount: token_accout_program,
        firstPersonPosAccount: pos_account_first_person,
      })
      .signers([token_mint, firstPerson])
      .rpc();

    console.log("Your transaction signature", tx);
  });
});

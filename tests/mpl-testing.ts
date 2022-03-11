import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MplTesting } from "../target/types/mpl_testing";
import {ASSOCIATED_TOKEN_PROGRAM_ID,TOKEN_PROGRAM_ID,mintTo,MINT_SIZE,createMint, createInitializeMintInstruction,createAssociatedTokenAccountInstruction,getAssociatedTokenAddress} from "@solana/spl-token"


describe("mpl-testing", async() => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MplTesting as Program<MplTesting>;

 

  it("Is initialized!", async () => {
    // Add your test here.
    console.log("INF");
    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
      'metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s',
    );
    const lamports = await program.provider.connection.getMinimumBalanceForRentExemption(MINT_SIZE);
    const mintKey = anchor.web3.Keypair.generate();

  
    let ata = await getAssociatedTokenAddress(
      mintKey.publicKey, // mint
      program.provider.wallet.publicKey // owner
    );
    console.log(`ATA: ${ata.toBase58()}`);
    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: program.provider.wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        lamports,
        programId:TOKEN_PROGRAM_ID,
    }), createInitializeMintInstruction(
      mintKey.publicKey, // mint pubkey
      0, // decimals
      program.provider.wallet.publicKey, // mint authority
      program.provider.wallet.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
    ),
    createAssociatedTokenAccountInstruction(
      program.provider.wallet.publicKey,
      ata,
      program.provider.wallet.publicKey,
      mintKey.publicKey
    )
    )
  
    const res = await program.provider.send(mint_tx,[mintKey]);
  
    console.log(await program.provider.connection.getParsedAccountInfo(mintKey.publicKey));
  
    console.log("ACCOUNT",res);
   console.log("MINTKEY",mintKey.publicKey.toString());
   console.log("USER",program.provider.wallet.publicKey.toString())
    const [metadatakey,] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKey.publicKey.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    );
    console.log("METDATA",metadatakey.toString());
    const tx = await program.rpc.createMetaData("GAGE","G","https://arweave.net/sCuT4ASiUgq7JxgU_3aoq0xJLpwH2Z1z2R2_xwPM8uc",1000,true,{
    accounts:{
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      payer: program.provider.wallet.publicKey,
      mplProgram:TOKEN_METADATA_PROGRAM_ID,
      metadata:metadatakey,
      mint:mintKey.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID
    }
    });
    console.log("TX",tx);
    const [masterKey,] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from('metadata'),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKey.publicKey.toBuffer(),
        Buffer.from('edition')
      ],
      TOKEN_METADATA_PROGRAM_ID,
    );

    console.log("MA",masterKey.toString());

    const cerate_tx = await program.rpc.createNft({
      accounts:{
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        updateAuth:program.provider.wallet.publicKey,
        metadataAccount:metadatakey,
        mplProgram:TOKEN_METADATA_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadataMint:mintKey.publicKey,
        masterEdition:masterKey,
        recieverAccount: ata
      }
    })

    console.log("MINTED",cerate_tx);
  });
});

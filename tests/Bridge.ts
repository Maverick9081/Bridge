import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BN } from "bn.js";
import { Bridge } from "../target/types/bridge";
import {Keypair, SystemProgram, Transaction,Connection, clusterApiUrl } from "@solana/web3.js";
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { getAssociatedTokenAddress, TOKEN_PROGRAM_ID, MintLayout, AccountLayout,getMinimumBalanceForRentExemptMint, createMintToInstruction, createInitializeMintInstruction, createMint, mintTo, createAccount } from "@solana/spl-token";

describe("Bridge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const commitment= 'processed';
  const connection = new Connection(clusterApiUrl('devnet'));
  
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  const program = anchor.workspace.Bridge as Program<Bridge>;

  it("Is initialized!", async () => {
    // Add your test here.

    // let mint =  Keypair.generate();
    // let tx1 = new Transaction().add(
    //   // create mint account
    //   SystemProgram.createAccount({
    //     fromPubkey: program.provider.publicKey,
    //     newAccountPubkey: mint.publicKey,
    //     space: MintLayout.span,
    //     lamports: await getMinimumBalanceForRentExemptMint(program.provider.connection),
    //     programId: TOKEN_PROGRAM_ID,
    //   }),
      // init mint account
    //   createInitializeMintInstruction(
    //      // always TOKEN_PROGRAM_ID
    //     mint.publicKey, // mint pubkey
    //     0, // decimals
    //     program.provider.publicKey, // mint authority
    //     program.provider.publicKey // freeze authority (if you don't need it, you can set `null`)
    //   )
    // );
    const payer = anchor.web3.Keypair.generate();


    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 1000000000),
      "processed"
    );

    let mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      0,
    );
    console.log('10')

    let senderAta = await createAccount(connection,payer,mint,payer.publicKey);

    await mintTo(connection,payer,mint,payer.publicKey,payer.publicKey,1);



    console.log('1')
  

    // await provider.sendAndConfirm(tx1,[mint])
    // console.log('1')

    // const senderAta = await getAssociatedTokenAddress(mint.publicKey,program.provider.publicKey);

    // let tx2 = new Transaction().add(
    //   createMintToInstruction(
    //      // always TOKEN_PROGRAM_ID
    //     mint.publicKey, // mint
    //     senderAta, // receiver (sholud b a token account)
    //     program.provider.publicKey, // mint authority
    //     1 ,
    //     // amount. if your decimals is 8, you mint 10^8 for 1 token.
    //   )
    // );

    // console.log(tx2)

    
//  await provider.sendAndConfirm(tx2,[])

    
 console.log('1')
    

    const vaultAccount = await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("vault"),
      mint.publicKey.toBuffer()
      ],
      program.programId
  )
 
  const freezingConfig = await anchor.web3.PublicKey.findProgramAddress([
      Buffer.from("config"),
      mint.publicKey.toBuffer()
      ],
      program.programId
  )
  const tokenProgram = new anchor.web3.PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
  const systemProgram = new anchor.web3.PublicKey('11111111111111111111111111111111');
  const rent = new anchor.web3.PublicKey("SysvarRent111111111111111111111111111111111");






    let amount = new anchor.BN(1);
    const freeze =  await program.methods.freezeToken(amount,2,'hello').accounts({
      sender :program.provider.publicKey,
       senderAta :senderAta,
       mint :mint.publicKey,
       vaultAccount :vaultAccount[0],
       freezingConfig :freezingConfig[0],
       tokenProgram :tokenProgram,
       systemProgram : systemProgram, 
       rent :rent
    }
  ).rpc();
    console.log("Your transaction signature", freeze);
  });
});

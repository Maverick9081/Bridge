import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Bridge } from "../target/types/bridge";

describe("Bridge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bridge as Program<Bridge>;

  it("Is initialized!", async () => {
    // Add your test here.
    await program.addEventListener("MyEvent",(success)=>{
      console.log(success)
    })
    const tx = await program.methods.freezeToken().rpc();
    console.log("Your transaction signature", tx);
  });
});

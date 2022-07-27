import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BN } from "bn.js";
import { Bridge } from "../target/types/bridge";

describe("Bridge", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bridge as Program<Bridge>;

  it("Is initialized!", async () => {
    // Add your test here.
    let amount = new BN(1);
    const tx = await program.methods.freezeToken(amount,{
      accounts:{

      }
    })
    console.log("Your transaction signature", tx);
  });
});

import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AnchorTestPoc } from "../target/types/anchor_test_poc";

describe("anchor-test-poc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorTestPoc as Program<AnchorTestPoc>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});

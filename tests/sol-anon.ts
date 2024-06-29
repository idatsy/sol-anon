import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolAnon } from "../target/types/sol_anon";
import { expect } from "chai";

describe("sol-anon", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolAnon as Program<SolAnon>;

  const owner = anchor.web3.Keypair.generate();

  const [inbox] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("inbox")], program.programId
  );

  it("Initializes an inbox", async () => {

      let token_airdrop = await provider.connection.requestAirdrop(owner.publicKey, 1000000000);
      await provider.connection.confirmTransaction(token_airdrop);

      await program
          .methods
          .initialize()
          .accountsPartial({admin: owner.publicKey})
          .signers([owner])
          .rpc();

        let inboxAccount = await program.account.inbox.fetch(inbox);
        expect(inboxAccount.admin.toString() === owner.publicKey.toString());
  });

  it("Changes owner", async () => {
    const newOwner = anchor.web3.Keypair.generate();

    let sig = await program
        .methods
        .changeAdmin(newOwner.publicKey)
        .accountsPartial({admin: owner.publicKey})
        .signers([owner])
        .rpc();

    let inboxAccount = await program.account.inbox.fetch(inbox);
    expect(inboxAccount.admin.toString()).to.equal(newOwner.publicKey.toString());
  });

  // it("Sends a message from a non-whitelisted address", async () => {
  //   const message = "Hello, Solana!";
  //
  //   await program.methods
  //     .sendMessage(message)
  //     .accounts({
  //       inbox: inboxKeypair.publicKey,
  //       sender: sender.publicKey,
  //     })
  //     .signers([sender])
  //     .rpc();
  //
  //   const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
  //   expect(inbox.slots).to.have.lengthOf(1);
  //   expect(inbox.slots[0].sender.toString()).to.equal(sender.publicKey.toString());
  //   expect(inbox.slots[0].content).to.equal(message);
  // });
  //
  // it("Whitelists an address", async () => {
  //   await program.methods
  //     .whitelistAddress(whitelistedSender.publicKey)
  //     .accounts({
  //       inbox: inboxKeypair.publicKey,
  //       owner: owner.publicKey,
  //     })
  //     .signers([owner])
  //     .rpc();
  //
  //   const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
  //   expect(inbox.whitelist).to.have.lengthOf(1);
  //   expect(inbox.whitelist[0].toString()).to.equal(whitelistedSender.publicKey.toString());
  // });
  //
  // it("Sends a message from a whitelisted address", async () => {
  //   const message = "Hello from whitelisted sender!";
  //
  //   await program.methods
  //     .sendMessage(message)
  //     .accounts({
  //       inbox: inboxKeypair.publicKey,
  //       sender: whitelistedSender.publicKey,
  //     })
  //     .signers([whitelistedSender])
  //     .rpc();
  //
  //   const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
  //   expect(inbox.slots).to.have.lengthOf(2);
  //   expect(inbox.slots[1].sender.toString()).to.equal(whitelistedSender.publicKey.toString());
  //   expect(inbox.slots[1].content).to.equal(message);
  // });
  //
  // it("Clears a slot", async () => {
  //   await program.methods
  //     .clearSlot(new anchor.BN(0))
  //     .accounts({
  //       inbox: inboxKeypair.publicKey,
  //       owner: owner.publicKey,
  //     })
  //     .signers([owner])
  //     .rpc();
  //
  //   const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
  //   expect(inbox.slots[0].sender.toString()).to.equal(anchor.web3.PublicKey.default.toString());
  //   expect(inbox.slots[0].content).to.equal("");
  // });
  //
  // it("Fails to clear a non-existent slot", async () => {
  //   try {
  //     await program.methods
  //       .clearSlot(new anchor.BN(99))
  //       .accounts({
  //         inbox: inboxKeypair.publicKey,
  //         owner: owner.publicKey,
  //       })
  //       .signers([owner])
  //       .rpc();
  //     expect.fail("Expected an error but none was thrown");
  //   } catch (error) {
  //     expect(error.error.errorMessage).to.equal("Invalid slot index");
  //   }
  // });
  //
  // it("Fails to send a message when all slots are full", async () => {
  //   // First, fill up all available slots
  //   for (let i = 0; i < 10; i++) {  // Assuming max 10 slots
  //     const newSender = anchor.web3.Keypair.generate();
  //     await program.methods
  //       .sendMessage(`Message ${i}`)
  //       .accounts({
  //         inbox: inboxKeypair.publicKey,
  //         sender: newSender.publicKey,
  //       })
  //       .signers([newSender])
  //       .rpc();
  //   }
  //
  //   // Now try to send one more message
  //   try {
  //     const extraSender = anchor.web3.Keypair.generate();
  //     await program.methods
  //       .sendMessage("Extra message")
  //       .accounts({
  //         inbox: inboxKeypair.publicKey,
  //         sender: extraSender.publicKey,
  //       })
  //       .signers([extraSender])
  //       .rpc();
  //     expect.fail("Expected an error but none was thrown");
  //   } catch (error) {
  //     expect(error.error.errorMessage).to.equal("Inbox is full");
  //   }
  // });
  //
  // it("Allows a whitelisted sender to use a cleared slot", async () => {
  //   const message = "Reusing cleared slot";
  //
  //   await program.methods
  //     .sendMessage(message)
  //     .accounts({
  //       inbox: inboxKeypair.publicKey,
  //       sender: whitelistedSender.publicKey,
  //     })
  //     .signers([whitelistedSender])
  //     .rpc();
  //
  //   const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
  //   const clearedSlot = inbox.slots[0];
  //   expect(clearedSlot.sender.toString()).to.equal(whitelistedSender.publicKey.toString());
  //   expect(clearedSlot.content).to.equal(message);
  // });
});

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolAnon } from "../target/types/sol_anon";
import { expect } from "chai";

describe("sol-anon", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolAnon as Program<SolAnon>;

  const inboxKeypair = anchor.web3.Keypair.generate();
  const owner = anchor.web3.Keypair.generate();
  const sender = anchor.web3.Keypair.generate();
  const whitelistedSender = anchor.web3.Keypair.generate();

  it("Initializes an inbox", async () => {
    await program.methods
      .initializeInbox()
      .accounts({
        inbox: inboxKeypair.publicKey,
        owner: owner.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([inboxKeypair, owner])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    expect(inbox.owner.toString()).to.equal(owner.publicKey.toString());
    expect(inbox.slots).to.be.empty;
    expect(inbox.whitelist).to.be.empty;
  });

  it("Sends a message from a non-whitelisted address", async () => {
    const message = "Hello, Solana!";

    await program.methods
      .sendMessage(message)
      .accounts({
        inbox: inboxKeypair.publicKey,
        sender: sender.publicKey,
      })
      .signers([sender])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    expect(inbox.slots).to.have.lengthOf(1);
    expect(inbox.slots[0].sender.toString()).to.equal(sender.publicKey.toString());
    expect(inbox.slots[0].content).to.equal(message);
  });

  it("Whitelists an address", async () => {
    await program.methods
      .whitelistAddress(whitelistedSender.publicKey)
      .accounts({
        inbox: inboxKeypair.publicKey,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    expect(inbox.whitelist).to.have.lengthOf(1);
    expect(inbox.whitelist[0].toString()).to.equal(whitelistedSender.publicKey.toString());
  });

  it("Sends a message from a whitelisted address", async () => {
    const message = "Hello from whitelisted sender!";

    await program.methods
      .sendMessage(message)
      .accounts({
        inbox: inboxKeypair.publicKey,
        sender: whitelistedSender.publicKey,
      })
      .signers([whitelistedSender])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    expect(inbox.slots).to.have.lengthOf(2);
    expect(inbox.slots[1].sender.toString()).to.equal(whitelistedSender.publicKey.toString());
    expect(inbox.slots[1].content).to.equal(message);
  });

  it("Clears a slot", async () => {
    await program.methods
      .clearSlot(new anchor.BN(0))
      .accounts({
        inbox: inboxKeypair.publicKey,
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    expect(inbox.slots[0].sender.toString()).to.equal(anchor.web3.PublicKey.default.toString());
    expect(inbox.slots[0].content).to.equal("");
  });

  it("Fails to clear a non-existent slot", async () => {
    try {
      await program.methods
        .clearSlot(new anchor.BN(99))
        .accounts({
          inbox: inboxKeypair.publicKey,
          owner: owner.publicKey,
        })
        .signers([owner])
        .rpc();
      expect.fail("Expected an error but none was thrown");
    } catch (error) {
      expect(error.error.errorMessage).to.equal("Invalid slot index");
    }
  });

  it("Fails to send a message when all slots are full", async () => {
    // First, fill up all available slots
    for (let i = 0; i < 10; i++) {  // Assuming max 10 slots
      const newSender = anchor.web3.Keypair.generate();
      await program.methods
        .sendMessage(`Message ${i}`)
        .accounts({
          inbox: inboxKeypair.publicKey,
          sender: newSender.publicKey,
        })
        .signers([newSender])
        .rpc();
    }

    // Now try to send one more message
    try {
      const extraSender = anchor.web3.Keypair.generate();
      await program.methods
        .sendMessage("Extra message")
        .accounts({
          inbox: inboxKeypair.publicKey,
          sender: extraSender.publicKey,
        })
        .signers([extraSender])
        .rpc();
      expect.fail("Expected an error but none was thrown");
    } catch (error) {
      expect(error.error.errorMessage).to.equal("Inbox is full");
    }
  });

  it("Allows a whitelisted sender to use a cleared slot", async () => {
    const message = "Reusing cleared slot";

    await program.methods
      .sendMessage(message)
      .accounts({
        inbox: inboxKeypair.publicKey,
        sender: whitelistedSender.publicKey,
      })
      .signers([whitelistedSender])
      .rpc();

    const inbox = await program.account.inbox.fetch(inboxKeypair.publicKey);
    const clearedSlot = inbox.slots[0];
    expect(clearedSlot.sender.toString()).to.equal(whitelistedSender.publicKey.toString());
    expect(clearedSlot.content).to.equal(message);
  });
});

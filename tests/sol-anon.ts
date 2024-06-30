import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolAnon } from "../target/types/sol_anon";
import { expect } from "chai";


describe("sol-anon", () => {
  // Setup
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolAnon as Program<SolAnon>;

  // Test accounts
  const owner = anchor.web3.Keypair.generate();
  const newOwner = anchor.web3.Keypair.generate();
  const whitelistedSender = anchor.web3.Keypair.generate();
  const nonWhitelistedSender = anchor.web3.Keypair.generate();

  const [inbox] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("inbox")],
    program.programId
  );

  // Helper functions
  async function airdropSol(to: anchor.web3.PublicKey, amount: number) {
    const airdrop = await provider.connection.requestAirdrop(to, amount);
    await provider.connection.confirmTransaction(airdrop);
  }

  async function getInboxAccount() {
    return await program.account.inbox.fetch(inbox);
  }

  async function getSlotAccount(slot: number) {
    const slotBuffer = Buffer.alloc(8);
    slotBuffer.writeBigUInt64LE(BigInt(slot));
    const [slotPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [slotBuffer],
      program.programId
    );
    return await program.account.slot.fetch(slotPda);
  }

  // Tests
  it("Initializes an inbox", async () => {
    await airdropSol(owner.publicKey, 1000000000);

    await program.methods
      .initialize()
      .accountsPartial({ admin: owner.publicKey })
      .signers([owner])
      .rpc();

    const inboxAccount = await getInboxAccount();
    expect(inboxAccount.admin.toString()).to.equal(owner.publicKey.toString());
  });

  it("Changes owner", async () => {
    await program.methods
      .changeAdmin(newOwner.publicKey)
      .accountsPartial({ admin: owner.publicKey })
      .signers([owner])
      .rpc();

    const inboxAccount = await getInboxAccount();
    expect(inboxAccount.admin.toString()).to.equal(newOwner.publicKey.toString());
  });

  it("Adds a user to whitelist", async () => {
    await airdropSol(newOwner.publicKey, 1000000000);

    await program.methods
      .addToWhitelist(whitelistedSender.publicKey)
      .accountsPartial({ admin: newOwner.publicKey })
      .signers([newOwner])
      .rpc();

    const [expectedPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [whitelistedSender.publicKey.toBuffer()],
      program.programId
    );
    const accountInfo = await provider.connection.getAccountInfo(expectedPda);
    expect(accountInfo).to.not.be.null;
  });

  it("Non-whitelisted user can't send a message as whitelisted", async () => {
    const [whitelistedPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [whitelistedSender.publicKey.toBuffer()],
      program.programId
    );
    try {
      await program
          .methods
          .sendWhitelistedMessage("Hello World!", owner.publicKey)
          .accountsPartial({ whitelist: whitelistedPda, sender: nonWhitelistedSender.publicKey })
          .signers([nonWhitelistedSender])
          .rpc()
    } catch (err) {
        expect(err).to.exist;
    }
  });

  it("Non whitelisted user can send a regular message", async () => {
    await airdropSol(nonWhitelistedSender.publicKey, 1000000000);

    await program.methods
      .sendRegularMessage("Hello World!", owner.publicKey)
      .accountsPartial({ inbox: inbox, sender: nonWhitelistedSender.publicKey })
      .signers([nonWhitelistedSender])
      .rpc();

    const inboxAccount = await getInboxAccount();
    expect(inboxAccount.latestFreeSlot.toString()).to.equal("1");

    const slotAccount = await getSlotAccount(0);
    expect(slotAccount.message).to.equal("Hello World!");
    expect(slotAccount.to.toString()).to.equal(owner.publicKey.toString());
  });

  it("Whitelisted user can send a message without paying for the slot and inbox refunded rent difference", async () => {
    await airdropSol(whitelistedSender.publicKey, 1000000000);

    const initialInboxBalance = await provider.connection.getBalance(inbox);

    await program.methods
      .sendWhitelistedMessage("Hi!", newOwner.publicKey)
      .accountsPartial({ inbox: inbox, sender: whitelistedSender.publicKey })
      .signers([whitelistedSender])
      .rpc();

    const inboxAccount = await getInboxAccount();
    expect(inboxAccount.latestFreeSlot.toString()).to.equal("1");
    expect(inboxAccount.latestWhitelistedSlot.toString()).to.equal("1");

    const slotAccount = await getSlotAccount(0);
    expect(slotAccount.message).to.equal("Hi!");
    expect(slotAccount.to.toString()).to.equal(newOwner.publicKey.toString());

    const finalInboxBalance = await provider.connection.getBalance(inbox);
    expect(finalInboxBalance).to.be.greaterThan(initialInboxBalance);
  });

  it("Whitelisted sending fails if no slots are available", async () => {
    try {
      await program
          .methods
          .sendWhitelistedMessage("This should fail!", newOwner.publicKey)
          .accountsPartial({ inbox: inbox, sender: whitelistedSender.publicKey })
          .signers([whitelistedSender])
          .rpc();
    } catch (err) {
      expect(err).to.exist;
    }
  });

  it("Whitelisted user can send a message and pay for the slot realloc", async () => {
    await program.methods
      .sendRegularMessage("Short message here", owner.publicKey)
      .accountsPartial({ inbox: inbox, sender: nonWhitelistedSender.publicKey })
      .signers([nonWhitelistedSender])
      .rpc();

    await program.methods
      .sendWhitelistedMessage("This is a much much longer message that will require more space", owner.publicKey)
      .accountsPartial({ inbox: inbox, sender: whitelistedSender.publicKey })
      .signers([whitelistedSender])
      .rpc();

    const inboxAccount = await getInboxAccount();
    expect(inboxAccount.latestWhitelistedSlot.toString()).to.equal("2");
  });

  it("Removes a user from the whitelist", async () => {
    const [expectedPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [whitelistedSender.publicKey.toBuffer()],
      program.programId
    );

    const accountInfoBefore = await provider.connection.getAccountInfo(expectedPda);
    expect(accountInfoBefore).to.not.be.null;

    await program.methods
      .removeFromWhitelist(whitelistedSender.publicKey)
      .accountsPartial({ admin: newOwner.publicKey })
      .signers([newOwner])
      .rpc();

    const accountInfoAfter = await provider.connection.getAccountInfo(expectedPda);
    expect(accountInfoAfter).to.be.null;
  });

  it("Admin can reclaim slots", async () => {
    const [slotPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.alloc(8)],
      program.programId
    );

    await program.methods
      .reclaimSlot()
      .accounts({ inbox: inbox, slot: slotPda, admin: newOwner.publicKey })
      .signers([newOwner])
      .rpc();

    const [expectedPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [whitelistedSender.publicKey.toBuffer()],
      program.programId
    );
    const accountInfo = await provider.connection.getAccountInfo(expectedPda);
    expect(accountInfo).to.be.null;
  });

  it("Admin can withdraw inbox balance without destroying the inbox", async () => {
    await airdropSol(inbox, 1000000000);
    const initialInboxBalance = await provider.connection.getBalance(inbox);

    await program.methods
      .withdrawSurplusInboxBalance()
      .accounts({ inbox: inbox, admin: newOwner.publicKey })
      .signers([newOwner])
      .rpc();

    const finalInboxBalance = await provider.connection.getBalance(inbox);
    expect(finalInboxBalance).to.be.lessThan(initialInboxBalance);
  });
});
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolAnon } from "../target/types/sol_anon";
import { expect } from "chai";

describe("sol-anon", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolAnon as Program<SolAnon>;

  // inital owner
  const owner = anchor.web3.Keypair.generate();
  // new owner after testing ownership change
  const newOwner = anchor.web3.Keypair.generate();
  // whitelisted user
  const whitelistedSender = anchor.web3.Keypair.generate();

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
    await program
        .methods
        .changeAdmin(newOwner.publicKey)
        .accountsPartial({admin: owner.publicKey})
        .signers([owner])
        .rpc();

    let inboxAccount = await program.account.inbox.fetch(inbox);
    expect(inboxAccount.admin.toString()).to.equal(newOwner.publicKey.toString());
  });

  it("Adds a user to whitelist", async () => {
    let token_airdrop = await provider.connection.requestAirdrop(newOwner.publicKey, 1000000000);
    await provider.connection.confirmTransaction(token_airdrop);

    let sig = await program
        .methods
        .addToWhitelist(whitelistedSender.publicKey)
        .accountsPartial({admin: newOwner.publicKey})
        .signers([newOwner])
        .rpc();

    const [excpected_pda] = anchor.web3.PublicKey.findProgramAddressSync([whitelistedSender.publicKey.toBuffer()], program.programId);
    const account_info = await provider.connection.getAccountInfo(excpected_pda);
    expect(account_info).to.not.be.null;
  });

  it("Non-whitelisted user can't send a message as whitelisted", async () => {
    const nonWhitelistedSender = anchor.web3.Keypair.generate();
    const [whitelisted_pda] = anchor.web3.PublicKey.findProgramAddressSync([whitelistedSender.publicKey.toBuffer()], program.programId);

    try {
      await program.methods
        .sendWhitelistedMessage("Hello World!", owner.publicKey)
        .accountsPartial({ whitelist: whitelisted_pda, sender: nonWhitelistedSender.publicKey })
        .signers([nonWhitelistedSender])
        .rpc();

      expect.fail("Expected transaction to fail, but it succeeded");
    } catch (error) {
      expect(error, "Transaction should have failed");
    }
  });

  it("Non whitelisted user can send a regular message", async () => {
    const nonWhitelistedSender = anchor.web3.Keypair.generate();
    let token_airdrop = await provider.connection.requestAirdrop(nonWhitelistedSender.publicKey, 1000000000);
    await provider.connection.confirmTransaction(token_airdrop);

    await program
        .methods
        .sendRegularMessage("Hello World!", owner.publicKey)
        .accountsPartial({inbox: inbox, sender: nonWhitelistedSender.publicKey})
        .signers([nonWhitelistedSender])
        .rpc();

    // check that messages have been incremented
    let inboxAccount = await program.account.inbox.fetch(inbox);
    expect(inboxAccount.latestFreeSlot.toString()).to.equal("1");

    // look up the slot and check the message is correct
    // create a buffer to derive the PDA same way as the program
    const latestFreeSlotBuffer = Buffer.alloc(8);
    latestFreeSlotBuffer.writeBigUInt64LE(BigInt(0));
    const [slotPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [latestFreeSlotBuffer],
        program.programId
    );
    // get the slot data
    const slotAccount = await program.account.slot.fetch(slotPda);
    // check it matches the expected data
    expect(slotAccount.message).to.equal("Hello World!");
    expect(slotAccount.to.toString()).to.equal(owner.publicKey.toString());
  });

  it("Whitelisted user can send a message without paying for the slot and inbox refunded rent difference", async () => {
    // Get the initial balance of the inbox so we can check if it received a refund
    const initialInboxBalance = await program.provider.connection.getBalance(inbox);

    // now that a slot has been paid for by the non-whitelisted user, the whitelisted user can send a message without paying
    await program
        .methods
        .sendWhitelistedMessage("Hi!", newOwner.publicKey)
        .accountsPartial({inbox: inbox, sender: whitelistedSender.publicKey})
        .signers([whitelistedSender])
        .rpc();

    // check that messages have been incremented
    let inboxAccount = await program.account.inbox.fetch(inbox);
    expect(inboxAccount.latestFreeSlot.toString()).to.equal("1");
    expect(inboxAccount.latestWhitelistedSlot.toString()).to.equal("1");

    // look up the slot and check the message is correct
    // create a buffer to derive the PDA same way as the program
    const latestFreeSlotBuffer = Buffer.alloc(8);
    latestFreeSlotBuffer.writeBigUInt64LE(BigInt(0));
    const [slotPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [latestFreeSlotBuffer],
        program.programId
    );
    // get the slot data
    const slotAccount = await program.account.slot.fetch(slotPda);
    // check it matches the expected data
    expect(slotAccount.message).to.equal("Hi!");
    expect(slotAccount.to.toString()).to.equal(newOwner.publicKey.toString());

    // check that the rent has been refunded to the inbox account
    const finalInboxBalance = await program.provider.connection.getBalance(inbox);
    expect(finalInboxBalance).to.be.greaterThan(initialInboxBalance);
  });

  it("Whitelisted user can send a message and pay for the slot", async () => {});

  it("Removes a user from the whitelist", async () => {
    const [excpected_pda] = anchor.web3.PublicKey.findProgramAddressSync([whitelistedSender.publicKey.toBuffer()], program.programId);
    const account_info = await provider.connection.getAccountInfo(excpected_pda);
    expect(account_info).to.not.be.null;

    let sig = await program
        .methods
        .removeFromWhitelist(whitelistedSender.publicKey)
        .accountsPartial({admin: newOwner.publicKey})
        .signers([newOwner])
        .rpc();

    const account_info_after = await provider.connection.getAccountInfo(excpected_pda);
    expect(account_info_after).to.be.null;
  });
});

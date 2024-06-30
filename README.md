# Sol-Anon: Secure On-Chain Messaging for Solana
> The latest documentation for this project is available at [https://idatsy.github.io/sol-anon/](https://idatsy.github.io/sol-anon/)

Sol-Anon is an on-chain inbox program built on Solana. It leverages the spam commonly occuring on the network by providing a whitelist mechanism for trusted senders. 
Non-whitelisted (spammers) subsidise the rent costs for whitelisted users by creating slots for them. Whitelisted users can then use these slots to store messages.

## Key Features

 **Spam Prevention**
   - Slot-based system to control message flow
   - Whitelisting mechanism for trusted senders

**Efficient Resource Management**
   - Dynamic slot allocation and deallocation
   - Rent refund system for inbox owners
   - PDA-based whitelist provides efficient access control
  - Efficient use of storage to minimize rent costs
  - Innovative rent refund mechanism to incentivize active inbox management

**Adaptive Slot Sizing**
   - Slots dynamically resize based on message length, optimizing storage usage and subsidizing rent-costs for whitelisted users.

**Inbox Owner Interface**
   - Tools for managing slots, whitelist, and inbox balance

## Future Enhancements
- ZK proof option as an alternative to whitelist.
- Versioned transaction with partial signatures to allow relayers to submit messages on behalf of users.
- Development of a user-friendly front-end interface

Developed with ❤️ for the Solana community.
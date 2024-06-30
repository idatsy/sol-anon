use anchor_lang::prelude::*;

declare_id!("73o3wfunMxS44tEuia2D4WJKmJtqRewX3F4tG2fmkSEV");

#[program]
pub mod sol_anon {
    use super::*;

    pub fn initialize(ctx: Context<InitializeInbox>) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = *ctx.accounts.admin.key;
        // dev: these seem to not be needed, fields are 0-initialized by default
        // inbox.latest_free_slot = 0;
        // inbox.latest_whitelisted_slot = 0;

        msg!("Inbox initialized with admin: {:?}", inbox.admin);

        Ok(())
    }

    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;
        inbox.admin = new_admin.key().clone();

        msg!("Inbox admin changed to: {:?}", inbox.admin);

        Ok(())
    }

    pub fn add_to_whitelist(_ctx: Context<AddToWhitelist>, address_to_whitelist: Pubkey) -> Result<()> {
        msg!("Adding address to whitelist: {:?}", address_to_whitelist);
        Ok(())
    }

    pub fn remove_from_whitelist(_ctx: Context<RemoveFromWhitelist>, address_to_remove: Pubkey) -> Result<()> {
        msg!("Removing address from whitelist: {:?}", address_to_remove);
        Ok(())
    }

    pub fn send_regular_message(ctx: Context<SendRegularMessage>, message: String, to: Pubkey) -> Result<()> {
        let slot = &mut ctx.accounts.slot;
        slot.to = to;
        slot.message = message;

        let inbox = &mut ctx.accounts.inbox;
        inbox.latest_free_slot += 1;

        msg!("Message {:?} sent to: {:?}. New free slot: {:?}", slot.message, slot.to, inbox.latest_free_slot);
        Ok(())
    }

    pub fn send_whitelisted_message(ctx: Context<SendWhitelistedMessage>, message: String, to: Pubkey) -> Result<()> {
        let inbox = &mut ctx.accounts.inbox;

        let slot = &mut ctx.accounts.slot.to_account_info();
        let current_space = slot.to_account_info().data_len();
        let required_space = 8 + 32 + 4 + message.len();

        slot.realloc(required_space, false)?;

        let rent = Rent::get()?;
        let new_rent = rent.minimum_balance(required_space);
        let old_rent = rent.minimum_balance(current_space);

        if new_rent > old_rent {
            // transfer from the signer to the slot
            let diff = new_rent - old_rent;
            **ctx.accounts.sender.to_account_info().try_borrow_mut_lamports()? -= diff;
            **slot.try_borrow_mut_lamports()? += diff;
        } else if new_rent < old_rent {
            let diff = old_rent - new_rent;
            **slot.try_borrow_mut_lamports()? -= diff;
            **inbox.to_account_info().try_borrow_mut_lamports()? += diff;
        }

        let slot = &mut ctx.accounts.slot;
        slot.to = to;
        slot.message = message;

        inbox.latest_whitelisted_slot += 1;

        msg!("Whitelisted message {:?} sent to: {:?}. New whitelisted slot: {:?}", slot.message, slot.to, inbox.latest_whitelisted_slot);
        Ok(())
    }
}

#[account]
pub struct Slot {
    pub to: Pubkey,
    pub message: String,
}

#[derive(Accounts)]
#[instruction(message: String)]
pub struct SendRegularMessage<'info> {
    #[account(mut)]
    pub inbox: Account<'info, Inbox>,
    #[account(init, seeds=[&inbox.latest_free_slot.to_le_bytes()], bump, payer = sender, space = 8 + 32 + 4 + message.len())]
    pub slot: Account<'info, Slot>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendWhitelistedMessage<'info> {
    #[account(mut)]
    pub inbox: Account<'info, Inbox>,
    #[account(
        mut,
        seeds=[&inbox.latest_whitelisted_slot.to_le_bytes()],
        bump,
        // ensure whitelisted messages can only be sent to slots that have been allocated by the inbox
        constraint = inbox.latest_whitelisted_slot < inbox.latest_free_slot,
        // DEV: If we realloc here, any message that is smaller than the already allocated space would result in a refund to the payer.
        // This would create some weird incentives where anyone whitelisted could be incentivised to send an empty message to get the refund.
        // Implementing realloc in the method execution allows us to make sure that the refund goes to the inbox admin instead of the sender
        // in cases where the message is smaller than the allocated space.
        //
        // realloc = 8 + 32 + 4 + message.len(),
        // realloc:payer = sender,
        // realloc::zero = false,
    )]
    pub slot: Account<'info, Slot>,
    #[account(seeds=[sender.key().as_ref()], bump)]
    pub whitelist: Account<'info, Whitelist>,
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Inbox {
    pub admin: Pubkey,
    pub latest_free_slot: u64,
    pub latest_whitelisted_slot: u64,
}

#[derive(Accounts)]
pub struct InitializeInbox<'info> {
    #[account(init, seeds=[b"inbox"], bump, payer = admin, space = 8 + 32 + 8 + 8)]
    pub inbox: Account<'info, Inbox>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut, seeds=[b"inbox"], bump, has_one = admin)]
    pub inbox: Account<'info, Inbox>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Whitelist {}

#[derive(Accounts)]
#[instruction(address_to_whitelist: Pubkey)] // We could make this an account but showcasing how to do this for reference ^.^
pub struct AddToWhitelist<'info> {
    // TODO: If time allows, don't use anchor so we can skip the 8 byte discriminant and save another 7 bytes of space!
    // Also would be nice to have a better understanding of PDA initialization, for example seeding a PDA with rent excemption alone is not secure
    // as there *could* be a way to to send money to a non-existing PDA even though it hasn't been created by this admin.
    // Either way due to time contsraints this probably won't be possible to do with Anchor but interesting to explore using a native implementation
    #[account(init, seeds=[address_to_whitelist.key().as_ref()], bump, payer = admin, space = 8)]
    pub whitelist_pda: Account<'info, Whitelist>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(address_to_remove: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(mut, seeds=[address_to_remove.key().as_ref()], bump, close = admin)]
    pub whitelist_pda: Account<'info, Whitelist>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

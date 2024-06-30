use anchor_lang::prelude::*;
use crate::state::Inbox;
use crate::constants::SLOT_BASE_SPACE;

/// Reallocates the slot account if necessary and handles the transfer of lamports
pub fn realloc_slot<'a>(
    slot: &AccountInfo<'a>,
    message: &str,
    inbox: &mut Account<'a, Inbox>,
    sender: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
) -> Result<()> {
    let current_space = slot.data_len();
    let required_space = SLOT_BASE_SPACE + message.len();

    slot.realloc(required_space, false)?;

    let rent = Rent::get()?;
    let new_rent = rent.minimum_balance(required_space);
    let old_rent = rent.minimum_balance(current_space);

    if new_rent > old_rent {
        let diff = new_rent.saturating_sub(old_rent);
        let inbox_rent = rent.minimum_balance(inbox.to_account_info().data_len());
        let inbox_surplus = inbox.to_account_info().lamports().saturating_sub(inbox_rent);

        let remaining_diff = diff.saturating_sub(inbox_surplus);
        if remaining_diff > 0 {
            let transfer_instruction = solana_program::system_instruction::transfer(
                sender.key,
                slot.key,
                remaining_diff
            );
            solana_program::program::invoke(
                &transfer_instruction,
                &[sender.clone(), slot.clone(), system_program.clone()],
            )?;
        }

        // DEV: We have to put PDA transfers after native transfers for *reasons*. See: https://solana.stackexchange.com/questions/4519/anchor-error-error-processing-instruction-0-sum-of-account-balances-before-and
        if inbox_surplus > 0 {
            **inbox.to_account_info().try_borrow_mut_lamports()? -= inbox_surplus;
            **slot.try_borrow_mut_lamports()? += inbox_surplus;
        }
    } else if new_rent < old_rent {
        let diff = old_rent - new_rent;
        **slot.try_borrow_mut_lamports()? -= diff;
        **inbox.to_account_info().try_borrow_mut_lamports()? += diff;
    }

    Ok(())
}
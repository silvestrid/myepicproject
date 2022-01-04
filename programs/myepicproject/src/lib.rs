use anchor_lang::prelude::*;

// declare_id!("AqYeE39uDeQz8yCSMswbJrqzbd6KXVfiZPWsyoKWPVbj");  // devnet
declare_id!("AqYeE39uDeQz8yCSMswbJrqzbd6KXVfiZPWsyoKWPVbj"); // localnet

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(_ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn add_gif(_ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let user = &mut _ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            votes_count: 0,
        };
        if let Some(_pos) = base_account
            .gif_list
            .iter()
            .position(|x| x.gif_link == gif_link && x.user_address == *user.to_account_info().key)
        {
            return Err(ErrorCode::GifDuplicate.into());
        }

        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. The Gif can only be removed from the owner
    pub fn delete_gif(_ctx: Context<DelGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;
        let user = &mut _ctx.accounts.user;

        if let Some(pos) = base_account
            .gif_list
            .iter()
            .position(|x| x.gif_link == gif_link && x.user_address == *user.to_account_info().key)
        {
            base_account.gif_list.remove(pos);
            base_account.total_gifs -= 1;
            return Ok(());
        }
        Err(ErrorCode::GifDelError.into())
    }

    // The function now accepts a gif_link param from the user. The Gif can only be removed from the owner
    pub fn vote_gif(_ctx: Context<VoteGif>, gif: ItemStruct) -> ProgramResult {
        let base_account = &mut _ctx.accounts.base_account;

        if let Some(pos) = base_account
            .gif_list
            .iter()
            .position(|x| x.gif_link == gif.gif_link && x.user_address == gif.user_address)
        {
            base_account.gif_list[pos].votes_count += 1;
            return Ok(());
        }
        Err(ErrorCode::GifMissing.into())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct DelGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct VoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes_count: u64,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}

#[error]
pub enum ErrorCode {
    #[msg("This Gif does not exist or you do not have the permission to delete it")]
    GifDelError,
    #[msg("This Gif already exists for this account")]
    GifDuplicate,
    #[msg("This Gif does not exists")]
    GifMissing,
}

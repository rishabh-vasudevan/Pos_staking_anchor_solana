use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token, token};

declare_id!("6azXDMKVeDTXeHCueTfba2F7YCkHfU1tQ6ZoA4MVb4WG");

#[program]
pub mod mint_and_transfer_token {

    use super::*;

    pub fn mint_and_transfer(ctx: Context<MintAndTransfer>) -> Result<()> {
        let (_pda, bump) = Pubkey::find_program_address(
            &[ctx.accounts.wallet.key.as_ref(), b"authority".as_ref()],
            ctx.program_id,
        );
        let _signer = &[
            ctx.accounts.wallet.key.as_ref(),
            b"authority".as_ref(),
            &[bump],
        ];
        let pda_signer = &[&_signer[..]];
        system_program::create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.wallet.to_account_info(),
                    to: ctx.accounts.mint.to_account_info(),
                },
            ),
            10000000,
            82,
            ctx.accounts.token_program.key,
        )?;
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            9,
            ctx.accounts.wallet.key,
            Some(ctx.accounts.authority_pda.key),
        )?;

        // associated_token::create(CpiContext::new(ctx.accounts.associated_token_program.to_account_info(), associated_token::Create { payer: ctx.accounts.wallet.to_account_info(), associated_token: ctx.accounts.first_person_token_account.to_account_info(), authority: ctx.accounts.wallet.to_account_info(), mint: ctx.accounts.mint.to_account_info(), system_program: ctx.accounts.system_program.to_account_info(), token_program: ctx.accounts.token_program.to_account_info()}))?;
        // associated_token::create(CpiContext::new_with_signer(ctx.accounts.associated_token_program.to_account_info(), associated_token::Create { payer: ctx.accounts.wallet.to_account_info(), associated_token:ctx.accounts.token.to_account_info() ,authority: ctx.accounts.authority_pda.to_account_info(), mint: ctx.accounts.mint.to_account_info(), system_program: ctx.accounts.system_program.to_account_info(), token_program: ctx.accounts.token_program.to_account_info(), rent:ctx.accounts.rent.to_account_info() }, pda_signer))?;

        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.wallet.to_account_info(),
                associated_token: ctx.accounts.first_person_token_account.to_account_info(),
                //Not confirmed but most probably authority is not mint authority it is the authority of the token_account
                authority: ctx.accounts.first_person.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.first_person_token_account.to_account_info(),
                    authority: ctx.accounts.wallet.to_account_info(),
                },
            ),
            100000000000,
        )?;

        //

        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.wallet.to_account_info(),
                associated_token: ctx.accounts.pos_program_account.to_account_info(),
                authority: ctx.accounts.authority_pda.to_account_info(),
                mint: ctx.accounts.pos_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.pos_mint.to_account_info(),
                    to: ctx.accounts.pos_program_account.to_account_info(),
                    authority: ctx.accounts.authority_pda.to_account_info(),
                },
            )
            .with_signer(pda_signer),
            100000000000,
        )?;

        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.wallet.to_account_info(),
                associated_token: ctx.accounts.program_token_account.to_account_info(),
                authority: ctx.accounts.authority_pda.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.program_token_account.to_account_info(),
                    authority: ctx.accounts.wallet.to_account_info(),
                },
            ),
            100000000000,
        )?;

        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.program_token_account.to_account_info(),
                    authority: ctx.accounts.wallet.to_account_info(),
                },
            ),
            100000000000,
        )?;

        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.wallet.to_account_info(),
                associated_token: ctx.accounts.first_person_pos_account.to_account_info(),
                authority: ctx.accounts.first_person.to_account_info(),
                mint: ctx.accounts.pos_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        Ok(())
    }

    pub fn stake_token(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let (_pda, bump) = Pubkey::find_program_address(
            &[ctx.accounts.wallet.key.as_ref(), b"authority".as_ref()],
            ctx.program_id,
        );
        let _signer = &[
            ctx.accounts.wallet.key.as_ref(),
            b"authority".as_ref(),
            &[bump],
        ];
        let pda_signer = &[&_signer[..]];
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.first_person_token_account.to_account_info(),
                    to: ctx.accounts.program_token_account.to_account_info(),
                    authority: ctx.accounts.first_person.to_account_info(),
                },
            ),
            amount,
        )?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.pos_program_account.to_account_info(),
                    to: ctx.accounts.first_person_pos_account.to_account_info(),
                    authority: ctx.accounts.authority_pda.to_account_info(),
                },
            )
            .with_signer(pda_signer),
            amount,
        )?;
        Ok(())
    }

    pub fn withraw(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let (_pda, bump) = Pubkey::find_program_address(
            &[ctx.accounts.wallet.key.as_ref(), b"authority".as_ref()],
            ctx.program_id,
        );
        let _signer = &[
            ctx.accounts.wallet.key.as_ref(),
            b"authority".as_ref(),
            &[bump],
        ];
        let pda_signer = &[&_signer[..]];
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.program_token_account.to_account_info(),
                    to: ctx.accounts.first_person_token_account.to_account_info(),
                    authority: ctx.accounts.first_person.to_account_info(),
                },
            ).with_signer(pda_signer),
            amount,
        )?;

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.first_person_pos_account.to_account_info(),
                    to: ctx.accounts.pos_program_account.to_account_info(),
                    authority: ctx.accounts.authority_pda.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!("withraw of amout {} completed", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintAndTransfer<'info> {
    #[account(mut)]
    wallet: Signer<'info>,
    #[account(mut)]
    mint: Signer<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    program_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut, seeds = [wallet.key.as_ref(), b"authority".as_ref()], bump)]
    authority_pda: UncheckedAccount<'info>,

    #[account(init, payer = wallet,mint::decimals = 9, mint::authority = authority_pda, mint::freeze_authority = authority_pda )]
    pos_mint: Account<'info, token::Mint>,
    ///CHECK:
    #[account(mut)]
    pos_program_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_pos_account: AccountInfo<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    wallet: Signer<'info>,
    #[account(mut)]
    mint: Signer<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    program_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut, seeds = [wallet.key.as_ref(), b"authority".as_ref()], bump)]
    authority_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pos_mint: Account<'info, token::Mint>,
    ///CHECK:
    #[account(mut)]
    pos_program_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_pos_account: AccountInfo<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    wallet: Signer<'info>,
    #[account(mut)]
    mint: Signer<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    program_token_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut, seeds = [wallet.key.as_ref(), b"authority".as_ref()], bump)]
    authority_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pos_mint: Account<'info, token::Mint>,
    ///CHECK:
    #[account(mut)]
    pos_program_account: AccountInfo<'info>,
    ///CHECK:
    #[account(mut)]
    first_person_pos_account: AccountInfo<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, token::Token>,
    associated_token_program: Program<'info, associated_token::AssociatedToken>,
    rent: Sysvar<'info, Rent>,
}

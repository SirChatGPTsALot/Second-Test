use anchor_lang::prelude::*;

// Placeholder program ID. Replace with actual ID on deployment.
declare_id!("BetJury11111111111111111111111111111111111");

#[program]
pub mod bet_jury {
    use super::*;

    pub fn create_bet(ctx: Context<CreateBet>, terms: String, stake: u64) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        bet.terms = terms;
        bet.stake = stake;
        bet.creator = *ctx.accounts.creator.key;
        bet.state = BetState::Pending;
        Ok(())
    }

    pub fn accept_bet(ctx: Context<AcceptBet>) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        require!(bet.state == BetState::Pending, BetError::BetNotPending);
        bet.participants.push(*ctx.accounts.participant.key);
        Ok(())
    }

    pub fn submit_outcome(ctx: Context<SubmitOutcome>, proposed_winner: Pubkey) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        require!(bet.state == BetState::Active, BetError::BetNotActive);
        bet.proposed_winner = Some(proposed_winner);
        bet.state = BetState::Voting;
        Ok(())
    }

    pub fn vote_outcome(ctx: Context<VoteOutcome>, vote_for: Pubkey) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        require!(bet.state == BetState::Voting, BetError::BetNotVoting);
        bet.votes.push(JuryVote { juror: *ctx.accounts.juror.key, vote_for });
        Ok(())
    }

    pub fn finalize_bet(ctx: Context<FinalizeBet>) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        require!(bet.state == BetState::Voting, BetError::BetNotVoting);
        // Naive majority calculation
        let mut counts: std::collections::HashMap<Pubkey, u64> = std::collections::HashMap::new();
        for v in bet.votes.iter() {
            *counts.entry(v.vote_for).or_default() += 1;
        }
        if let Some((winner, _)) = counts.into_iter().max_by_key(|(_, c)| *c) {
            bet.winner = Some(winner);
            bet.state = BetState::Finalized;
        }
        Ok(())
    }

    pub fn cancel_bet(ctx: Context<CancelBet>) -> Result<()> {
        let bet = &mut ctx.accounts.bet;
        require!(bet.creator == *ctx.accounts.creator.key, BetError::Unauthorized);
        bet.state = BetState::Cancelled;
        Ok(())
    }
}

#[account]
pub struct Bet {
    pub creator: Pubkey,
    pub terms: String,
    pub stake: u64,
    pub participants: Vec<Pubkey>,
    pub jury: Vec<Pubkey>,
    pub votes: Vec<JuryVote>,
    pub proposed_winner: Option<Pubkey>,
    pub winner: Option<Pubkey>,
    pub state: BetState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BetState {
    Pending,
    Active,
    Voting,
    Finalized,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct JuryVote {
    pub juror: Pubkey,
    pub vote_for: Pubkey,
}

#[derive(Accounts)]
pub struct CreateBet<'info> {
    #[account(init, payer = creator, space = 8 + 2000)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptBet<'info> {
    #[account(mut)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub participant: Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitOutcome<'info> {
    #[account(mut)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub proposer: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteOutcome<'info> {
    #[account(mut)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub juror: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizeBet<'info> {
    #[account(mut)]
    pub bet: Account<'info, Bet>,
}

#[derive(Accounts)]
pub struct CancelBet<'info> {
    #[account(mut)]
    pub bet: Account<'info, Bet>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[error_code]
pub enum BetError {
    #[msg("Bet not pending")] BetNotPending,
    #[msg("Bet not active")] BetNotActive,
    #[msg("Bet not in voting state")] BetNotVoting,
    #[msg("Unauthorized")] Unauthorized,
}

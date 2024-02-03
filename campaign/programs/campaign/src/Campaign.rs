use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("6AdyKjD2cYkqykkKHpQs9KxJGu5XzAZgwwJgzSyKWnDf");

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Campaign {
    pub minimum_contribution: u64,
    pub manager: Pubkey,
    pub approvers: Vec<Pubkey>,
    pub approvers_count: u64,
    pub requests: Vec<Request>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Request {
    pub description: String,
    pub value: u64,
    pub recipient: Pubkey,
    pub complete: bool,
    pub approval_count: u64,
    pub approvals: Vec<bool>,
}

#[program]
pub mod campaign_factory {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, minimum_contribution: u64) -> ProgramResult {
        let campaign = Campaign {
            minimum_contribution,
            manager: *ctx.accounts.manager.key,
            approvers: vec![],
            approvers_count: 0,
            requests: vec![],
        };

        let campaign_account = &mut ctx.accounts.campaign_account;
        campaign_account.minimum_contribution = minimum_contribution;
        campaign_account.manager = *ctx.accounts.manager.key;
        campaign_account.approvers = vec![];
        campaign_account.approvers_count = 0;
        campaign_account.requests = vec![];

        Ok(())
    }

    pub fn contribute(ctx: Context<Contribute>, amount: u64) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign_account;

        if amount < campaign_account.minimum_contribution {
            return Err(ErrorCode::InsufficientContribution.into());
        }

        campaign_account.approvers.push(*ctx.accounts.contributor.key);
        campaign_account.approvers_count += 1;

        Ok(())
    }

    pub fn create_request(
        ctx: Context<CreateRequest>,
        description: String,
        value: u64,
        recipient: Pubkey,
    ) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign_account;

        let request = Request {
            description,
            value,
            recipient,
            complete: false,
            approval_count: 0,
            approvals: vec![false; campaign_account.approvers.len()],
        };

        campaign_account.requests.push(request);

        Ok(())
    }

    pub fn approve_request(ctx: Context<ApproveRequest>, index: u64) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign_account;
        let request = &mut campaign_account.requests[index as usize];

        if !campaign_account.approvers.contains(&*ctx.accounts.approver.key) {
            return Err(ErrorCode::NotAnApprover.into());
        }

        if request.approvals[index as usize] {
            return Err(ErrorCode::AlreadyApproved.into());
        }

        request.approvals[index as usize] = true;
        request.approval_count += 1;

        Ok(())
    }

    pub fn finalize_request(ctx: Context<FinalizeRequest>, index: u64) -> ProgramResult {
        let campaign_account = &mut ctx.accounts.campaign_account;
        let request = &mut campaign_account.requests[index as usize];

        if request.approval_count <= (campaign_account.approvers_count / 2) {
            return Err(ErrorCode::NotEnoughApprovals.into());
        }

        if request.complete {   
            return Err(ErrorCode::RequestAlreadyFinalized.into());
        }
    
        request.complete = true;
    }
    
        Ok(())
    }
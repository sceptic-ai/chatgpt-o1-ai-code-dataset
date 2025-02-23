/*
=====================================================
Anchor Counter Program (lib.rs)
=====================================================
Purpose:
    Demonstrates a simple Anchor program that:
    1. Stores a counter in a program-owned account.
    2. Provides an instruction to increment this counter.
Crates/Dependencies:
    - anchor-lang for building on Solana with Anchor.
Usage:
    1. `anchor build` to build the program.
    2. `anchor deploy` to deploy it to the Solana cluster.
    3. `anchor test` to run tests (if you have a test file).
*/

use anchor_lang::prelude::*;

/// The main entry point to our Anchor program.
/// The `declare_id!` macro sets the program ID.
/// This must match the program ID that is deployed on Solana.
declare_id!("8HzSo6A4rvQo61zMqwEK3H7QNxVD6sbRQEQ9N2hLQcNZ"); // Example ID, change to your program ID.

#[program]
pub mod counter {
    use super::*;

    /// The `initialize` function creates a new `CounterAccount` and sets its initial state.
    /// This is typically called once per user or entity that wants to store a counter.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        Ok(())
    }

    /// The `increment` function updates the `CounterAccount` by incrementing the `count`.
    /// Anyone holding the account's address and passing the correct `Context` can invoke this.
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = counter_account
            .count
            .checked_add(1)
            .ok_or_else(|| error!(CounterError::Overflow))?;
        Ok(())
    }
}

/// Context for the `initialize` instruction.  
/// We define which accounts are involved and how they should be accessed.
#[derive(Accounts)]
pub struct Initialize<'info> {
    /// The account that will store our counter data.
    /// - `init` indicates this account is being created.
    /// - `payer` indicates who pays for the account creation.
    /// - `space` is the size (in bytes) needed for storing our data.
    #[account(init, payer = user, space = 8 + 8)]
    pub counter_account: Account<'info, CounterAccount>,

    /// The user paying for the account creation. Must be a signer.
    #[account(mut)]
    pub user: Signer<'info>,

    /// System program is required for account creation.
    pub system_program: Program<'info, System>,
}

/// Context for the `increment` instruction.
#[derive(Accounts)]
pub struct Increment<'info> {
    /// Mutably borrow our counter account to update its data.
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

/// Our account data structure.  
/// We store a single `i64` counter. The `#[account]` attribute tells Anchor
/// that this struct represents a Solana account's data layout.
#[account]
pub struct CounterAccount {
    pub count: i64,
}

/// A custom error enumeration for demonstration purposes.
#[error_code]
pub enum CounterError {
    #[msg("Counter has overflowed")]
    Overflow,
}
/*
===============================================================
Anchor Program: Token Mint and Associated Token Account
===============================================================
Purpose:
    Demonstrates how to create and initialize a token mint
    and an associated token account (ATA) using Anchor and SPL Token.
Crates/Dependencies:
    - anchor-lang, anchor-spl
Usage:
    1. Configure your Anchor.toml with correct program ID.
    2. 'anchor build' and 'anchor deploy'.
    3. Write or run tests to create a token mint and an ATA.
*/

use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

// Example program ID
declare_id!("3kZ2RkZnKDYNxGwxnEkTSQnQ1zFQgkk73xLtESTgCxmf");

#[program]
pub mod anchor_token_example {
    use super::*;

    /// Create a token mint with the specified decimals, then create an associated token account for
    /// the specified `authority`.
    pub fn create_mint_and_ata(
        ctx: Context<CreateMintAndATA>,
        decimals: u8,
    ) -> Result<()> {
        // Initialize the token mint
        token::initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.mint.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            decimals,
            &ctx.accounts.authority.key(),
            Some(&ctx.accounts.authority.key()),
        )?;

        // The associated token account is automatically created by Anchor
        // thanks to `associated_token::create` in the context.
        // No extra instruction needed if we simply define it in the context.

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMintAndATA<'info> {
    /// The payer for creating accounts, must be a signer.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The new token mint account. This will be created and owned by the Token Program.
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub mint: Account<'info, Mint>,

    /// The authority that will own the new mint and associated token account.
    /// Typically the user or a specific governance account.
    pub authority: SystemAccount<'info>,

    /// Associated token account (ATA) for the minted tokens.
    /// Anchor will automatically create it if it doesn't exist.
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub associated_token_account: Account<'info, TokenAccount>,

    /// The system program is required for creating accounts on Solana.
    pub system_program: Program<'info, System>,
    
    /// The token program is required for mint/ATA creation instructions.
    pub token_program: Program<'info, Token>,

    /// The associated token program is used to derive and create the associated token account.
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Rent sysvar for determining rent-exempt status.
    pub rent: Sysvar<'info, Rent>,
}
/*
=================================================
Direct Solana Transfer (Off-Chain)
=================================================
Purpose:
    Demonstrate how to use Solana's Rust crates to:
    1. Create a local keypair (or read from file).
    2. Connect to a Solana cluster (devnet or localnet).
    3. Transfer SOL from one account to another.
Crates/Dependencies:
    solana-sdk, solana-client, solana-keygen, solana-program
Usage:
    1. Update `Cargo.toml` accordingly:
       [dependencies]
       solana-sdk = "1.14.11"
       solana-client = "1.14.11"
    2. `cargo run`
*/

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};

fn main() {
    // Connect to devnet for this example.
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Generate a new ephemeral keypair (for demonstration).
    // In practice, you might load from a file or a wallet manager.
    let from_keypair = Keypair::new();
    let from_pubkey = from_keypair.pubkey();

    // A test recipient address. Replace with actual Pubkey for real usage.
    let to_pubkey = Pubkey::new_unique();

    // Airdrop some SOL to the sender on devnet so we can make a transfer.
    println!("Requesting airdrop...");
    let airdrop_sig = client.request_airdrop(&from_pubkey, 2_000_000_000).unwrap();
    client
        .confirm_transaction(&airdrop_sig)
        .expect("Failed to confirm airdrop");

    // Build a transaction that transfers 0.5 SOL.
    let tx_amount = 500_000_000; // in lamports (1 SOL = 1e9 lamports)
    let transfer_ix = system_instruction::transfer(&from_pubkey, &to_pubkey, tx_amount);

    // A Solana transaction requires a recent blockhash and instructions.
    let latest_hash = client.get_latest_blockhash().unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&from_pubkey),
        &[&from_keypair],
        latest_hash,
    );

    // Send and confirm the transaction.
    let signature = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");
    
    println!("Transfer complete! Tx signature: {}", signature);
    println!("{} lamports (0.5 SOL) sent to {}", tx_amount, to_pubkey);
}
/*
==========================================================
1) Non-Anchor System Program Transfer (Off-Chain Client)
==========================================================
Purpose:
  - Connect to Solana devnet (or another cluster).
  - Create a keypair or load from file.
  - Request an airdrop (devnet) to fund the sender.
  - Transfer SOL (lamports) to another public key.
*/

use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
};

fn main() {
    // Connect to devnet (replace with localnet or mainnet as needed).
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // Generate a new ephemeral keypair for demonstration.
    // In production, load a keypair from file or a wallet manager.
    let from_keypair = Keypair::new();
    let from_pubkey = from_keypair.pubkey();
    println!("Sender pubkey: {}", from_pubkey);

    // A test recipient address. Use a real public key in production.
    let to_pubkey = Pubkey::new_unique();
    println!("Recipient pubkey: {}", to_pubkey);

    // Airdrop 1 SOL to the sender for devnet usage (1 SOL = 1e9 lamports).
    println!("Requesting airdrop of 1 SOL...");
    let airdrop_sig = client.request_airdrop(&from_pubkey, 1_000_000_000).unwrap();
    client
        .confirm_transaction(&airdrop_sig)
        .expect("Failed to confirm airdrop");

    // Prepare a system transfer instruction for 0.1 SOL (100 million lamports).
    let lamports_to_send = 100_000_000; // 0.1 SOL
    let transfer_ix = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports_to_send);

    // Get a recent blockhash required for transaction creation.
    let latest_blockhash = client.get_latest_blockhash().unwrap();

    // Create and sign the transaction.
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],
        Some(&from_pubkey),
        &[&from_keypair],
        latest_blockhash,
    );

    // Send and confirm the transaction on-chain.
    let signature = client
        .send_and_confirm_transaction(&tx)
        .expect("Failed to send transaction");
    
    println!("Transfer success! Tx signature: {}", signature);
    println!(
        "Sent {} lamports (0.1 SOL) from {} to {}",
        lamports_to_send, from_pubkey, to_pubkey
    );
}
/*
===========================================
2) Anchor "Hello World" Program
===========================================
Purpose:
  - Illustrate a minimal Anchor program that just logs
    a greeting message when the instruction is invoked.
*/

use anchor_lang::prelude::*;

// Replace with your actual program ID.
declare_id!("Fg6PaFpoGXkYsidMpWTK4W6W7YfZBNuEhu5uuvvFLJBk");

#[program]
pub mod hello_world {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello, Solana! This is an Anchor program log.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
/*
======================================================
3) Anchor PDA Example
======================================================
Purpose:
  - Demonstrate how to derive a program-owned account
    using seeds and store data in it.
  - Provide an 'initialize' instruction to create the PDA,
    and an 'update' instruction to modify stored data.
*/

use anchor_lang::prelude::*;

declare_id!("BpfProgram1111111111111111111111111111111111"); // Placeholder, replace with real ID

#[program]
pub mod pda_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed_string: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.data = seed_string;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_data: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.data = new_data;
        Ok(())
    }
}

/// Instruction accounts for "initialize".
#[derive(Accounts)]
#[instruction(seed_string: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        // We derive the PDA using seeds = [b"base_seed", user.key().as_ref(), seed_string.as_bytes()]
        // bump is auto-populated by anchor in the context
        seeds = [b"base_seed", user.key().as_ref(), seed_string.as_bytes()],
        bump,
        space = 8 + 64 // 8 bytes account discriminator + 64 for data
    )]
    pub base_account: Account<'info, BaseAccount>,

    // Payer for the account creation
    #[account(mut)]
    pub user: Signer<'info>,

    // System Program
    pub system_program: Program<'info, System>,
}

/// Instruction accounts for "update".
#[derive(Accounts)]
pub struct Update<'info> {
    // We must reference the same seeds to ensure we get the same PDA
    #[account(
        mut,
        seeds = [b"base_seed", authority.key().as_ref(), base_account.data.as_bytes()],
        bump
    )]
    pub base_account: Account<'info, BaseAccount>,

    // The authority that originally created the account might need to sign
    #[account(mut)]
    pub authority: Signer<'info>,
}

/// Our custom account that holds some string data.
#[account]
pub struct BaseAccount {
    pub data: String,
}
/*
========================================================
4) Anchor CPI to System Program
========================================================
Purpose:
  - Illustrate how to create an account via CPI to the System Program.
  - The "create_account" instruction will do so on behalf of the caller,
    but from inside our Anchor program.
*/

use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_lang::solana_program::system_instruction;

declare_id!("CpiSystem1111111111111111111111111111111111"); // Placeholder

#[program]
pub mod cpi_system_example {
    use super::*;

    pub fn create_account_cpi(ctx: Context<CreateAccountCpi>) -> Result<()> {
        let bump = *ctx.bumps.get("my_account").unwrap();

        // Let's define the rent-exempt minimum for the new account
        let lamports_required = Rent::get()?.minimum_balance(100);

        // Build the system_program instruction
        let cpi_ix = system_instruction::create_account(
            &ctx.accounts.user.key(),
            &ctx.accounts.my_account.key(),
            lamports_required,
            100, // space for data, just for demonstration
            &ctx.program_id,
        );

        // Build CPI context
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::CreateAccount {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.my_account.to_account_info(),
            },
        );

        // Execute the CPI
        system_program::create_account(cpi_ctx, lamports_required, 100, &ctx.program_id)?;

        // Alternatively, we could manually call invoke_signed. But Anchor provides these CPI helpers.
        msg!("Account created via CPI.");

        // Bump usage example (not strictly needed here).
        msg!("Bump: {}", bump);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAccountCpi<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        // We do NOT init here; we do it manually via CPI
        seeds = [b"test", user.key().as_ref()],
        bump
    )]
    pub my_account: UninitializedAccount<'info>,

    pub system_program: Program<'info, System>,
}
/*
========================================================
5) Anchor SPL Token Example
========================================================
Purpose:
  - Demonstrate creation of a token mint via Anchor,
    and how to transfer tokens to a recipient's ATA.
  - Uses anchor-spl for convenience methods.
*/

use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount, InitializeMint, MintTo, Transfer,
};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("TokenExample111111111111111111111111111111111"); // Placeholder

#[program]
pub mod anchor_spl_example {
    use super::*;

    /// Create a new mint and mint some initial supply to the authority's associated token account.
    pub fn create_mint_and_mint_to(ctx: Context<CreateMintAndMintTo>, amount: u64) -> Result<()> {
        // Initialize the token mint
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 6, &ctx.accounts.authority.key(), Some(&ctx.accounts.authority.key()))?;

        // Mint tokens to the authority's ATA
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.authority_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;

        msg!("Created mint with 6 decimals, minted {} tokens to authority ATA.", amount);
        Ok(())
    }

    /// Transfer tokens from the signer to a recipient's ATA.
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_ata.to_account_info(),
            to: ctx.accounts.to_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        msg!("Transferred {} tokens.", amount);
        Ok(())
    }
}

/// Accounts for creating a mint and minting tokens to an authority's ATA.
#[derive(Accounts)]
pub struct CreateMintAndMintTo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The mint account to be created.
    #[account(
        init,
        payer = authority,
        mint::decimals = 0, // We'll override in code, but let's keep it consistent
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub mint: Account<'info, Mint>,

    /// The associated token account for the authority's minted tokens.
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub authority_ata: Account<'info, TokenAccount>,

    #[account(address = spl_token::id())]
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Accounts for transferring tokens from 'authority' to 'to_ata'.
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        // Ensure the 'from_ata' is owned by 'authority'
        constraint = from_ata.owner == authority.key()
    )]
    pub from_ata: Account<'info, TokenAccount>,

    /// The associated token account of the recipient
    #[account(mut)]
    pub to_ata: Account<'info, TokenAccount>,

    #[account(address = spl_token::id())]
    pub token_program: Program<'info, Token>,
}
/*
========================================================
6) Anchor NFT Metadata Example (Conceptual)
========================================================
Purpose:
  - Mint an NFT (1 token, 0 decimals).
  - Call Metaplex's token metadata program via CPI
    to create metadata for this NFT.
Note:
  - This is a conceptual snippet. The real token metadata
    program requires many additional checks & parameters.
*/

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, InitializeMint, MintTo};
use anchor_spl::associated_token::AssociatedToken;

// Typically: use metaplex-token-metadata crates for real usage
// For demonstration, we assume a hypothetical CPI call to "create_metadata_accounts".

declare_id!("NFTMeta1111111111111111111111111111111111"); // Placeholder

#[program]
pub mod nft_metadata_example {
    use super::*;

    pub fn mint_nft_with_metadata(
        ctx: Context<MintNftWithMetadata>,
        uri: String,
        title: String,
        symbol: String,
    ) -> Result<()> {
        // 1) Initialize mint as an NFT (0 decimals, supply = 1 minted to the owner).
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 0, &ctx.accounts.authority.key(), Some(&ctx.accounts.authority.key()))?;

        // 2) Mint 1 token to the owner's ATA
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        // 3) CPI to the token metadata program to create metadata
        // (In real usage, you'd need the correct instruction data & seeds).
        msg!("CPI to Metaplex Token Metadata: create_metadata_accounts");
        // For demonstration, this might be something like:
        // invoke_signed(
        //   &create_metadata_accounts_ix(...),
        //   &[
        //       ctx.accounts.metadata_account.to_account_info(),
        //       ...
        //   ],
        //   &[...]
        // )?;

        msg!(
            "NFT minted with URI: {}, Title: {}, Symbol: {}",
            uri,
            title,
            symbol
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNftWithMetadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        mint::decimals = 0,
        mint::authority = authority,
        mint::freeze_authority = authority
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub owner_ata: Account<'info, TokenAccount>,

    // Typically you'd create a metadata PDA via the Metaplex program
    // We'll just show it as a generic uninitialized account for demonstration.
    #[account(mut)]
    pub metadata_account: UninitializedAccount<'info>,

    #[account(address = spl_token::id())]
    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    // The real Metaplex token metadata program
    // pub token_metadata_program: Program<'info, TokenMetadataProgram>,
}
/*
=============================================================
7) Anchor CPI: One Anchor Program calling Another
=============================================================
Purpose:
  - "Program A" calls "Program B" via a CPI to the 'ping' instruction.
  - Both are Anchor-based. "Program B" must allow external calls (public).
*/

use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction;
use anchor_lang::solana_program::program::invoke_signed;

// Program B's ID
pub const CALLEE_PROGRAM_ID: &str = "Callee1111111111111111111111111111111111";

declare_id!("Caller11111111111111111111111111111111111111");

#[program]
pub mod caller_program {
    use super::*;

    pub fn cpi_ping_program_b(ctx: Context<CpiPing>) -> Result<()> {
        // Construct the "ping" instruction data. 
        // Anchor instructions typically have a discriminator. For demonstration, let's assume we know the IDL or we simply do:
        // let ix_data = <the Bincode or Anchor-serialized "ping" data>;
        // In real usage, you can get it from the IDL or build manually.

        let ix_data = vec![123, 234]; // Placeholder, not real. Replace with correct Anchor instruction data.

        let accounts = vec![
            // Program B's "ping" might require some accounts. 
            // For demonstration, let's assume it only needs the 'signer' as a read account.
            AccountMeta::new_readonly(ctx.accounts.user.key(), true),
        ];

        let ix = Instruction {
            program_id: Pubkey::from_str(CALLEE_PROGRAM_ID).unwrap(),
            accounts,
            data: ix_data,
        };

        // If Program B expects a sign from our caller's PDA, we'd do `invoke_signed`.
        // If no sign needed, `invoke` is enough. Here we show `invoke`.
        let account_infos = [
            ctx.accounts.user.to_account_info(),
            ctx.accounts.callee_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        invoke_signed(
            &ix,
            &account_infos,
            &[], // sign seeds if needed
        )?;

        msg!("Successfully invoked Program B's ping instruction.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CpiPing<'info> {
    pub user: Signer<'info>,

    // Program B, the callee
    #[account(address = Pubkey::from_str(CALLEE_PROGRAM_ID).unwrap())]
    pub callee_program: Program<'info, System>, // or a custom type

    pub system_program: Program<'info, System>,
}
/*
========================================================
8) Off-Chain Ephemeral Key Example (Token Mint)
========================================================
Purpose:
  - Create an ephemeral keypair to sign an SPL token
    mint transaction, then discard it.
*/

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use anchor_spl::token::{mint_to, MintTo, Token};
use anchor_lang::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to devnet
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    // 1) Create ephemeral keypair
    let ephemeral_keypair = Keypair::new();
    println!("Ephemeral public key: {}", ephemeral_keypair.pubkey());

    // 2) Suppose we have an existing mint & user ATA
    let mint_pubkey = Pubkey::new_unique();
    let user_ata = Pubkey::new_unique();

    // 3) Build a MintTo instruction (using anchor_spl style).
    // In practice, you'd need to gather all required account infos.
    let mint_to_ix = anchor_spl::token::instruction::mint_to(
        &spl_token::id(),
        &mint_pubkey,
        &user_ata,
        &ephemeral_keypair.pubkey(), // ephemeral as the authority
        &[],
        100, // amount to mint
    )?;

    // 4) Construct transaction
    let blockhash = client.get_latest_blockhash().await.unwrap();
    let tx = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&ephemeral_keypair.pubkey()),
        &[&ephemeral_keypair],
        blockhash,
    );

    // 5) Send and confirm
    let signature = client.send_and_confirm_transaction(&tx).await;
    match signature {
        Ok(sig) => println!("Mint successful. Tx signature: {}", sig),
        Err(e) => println!("Mint failed: {}", e),
    }

    // 6) Discard ephemeral key. It's simply dropped from scope here.

    Ok(())
}
/*
========================================================
9) Anchor Test Example
========================================================
Purpose:
  - Demonstrate how to write a test in the `tests/` folder
    that calls our Anchor program instructions.
*/

use anchor_lang::prelude::*;
use anchor_lang::AccountDeserialize;
use anchor_lang::ToAccountInfos;
use anchor_spl::token::{Token, Mint};
use solana_program_test::*;
use solana_sdk::{signer::Signer, transport::TransportError};

use counter::program::Counter; // Example if our program is named 'counter'
use counter::{self, CounterAccount};

#[tokio::test]
async fn test_counter_program() -> Result<(), TransportError> {
    // 1) Start a local program test environment
    let mut program_test = ProgramTest::new(
        "counter", // the name from Cargo.toml
        counter::id(), // the program ID
        processor!(counter::entry), // the entrypoint
    );

    // Add any other programs or set up if needed
    program_test.add_program("spl_token", spl_token::id(), None);

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // 2) Create a new Keypair for our counter account
    let counter_account = Keypair::new();

    // 3) Build and send an "initialize" instruction
    let init_tx = counter::initialize(
        // anchor test uses a client-based builder pattern
        &mut banks_client,
        &payer,
        &recent_blockhash,
        counter_account.pubkey(),
    )?;
    // If your program doesn't have a helper, you'd manually create an Instruction + signers.

    banks_client.process_transaction(init_tx).await?;

    // 4) Build and send an "increment" instruction
    let inc_tx = counter::increment(
        &mut banks_client,
        &payer,
        &recent_blockhash,
        counter_account.pubkey(),
    )?;
    banks_client.process_transaction(inc_tx).await?;

    // 5) Fetch the account data from on-chain
    let account_data = banks_client
        .get_account(counter_account.pubkey())
        .await?
        .expect("Counter account not found.");

    // 6) Deserialize the data into our CounterAccount type
    let counter_state = CounterAccount::try_deserialize(&mut &account_data.data[..])?;
    assert_eq!(counter_state.count, 1);

    println!("Test success! Counter = {}", counter_state.count);

    Ok(())
}

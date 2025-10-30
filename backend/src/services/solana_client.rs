use base64::engine::general_purpose::STANDARD as base64_standard;
use base64::Engine as _;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_request::TokenAccountsFilter,
    rpc_response::RpcKeyedAccount,
};
use solana_program::program_pack::Pack;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

use crate::config::SolanaConfig;
use anyhow::Result;
use solana_commitment_config::CommitmentConfig;
// Type alias for thread-safe errors
type ThreadSafeError = Box<dyn std::error::Error + Send + Sync>;

// Jupiter API response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterQuote {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub other_amount_threshold: String,
    pub swap_mode: String,
    pub slippage_bps: u64,
    pub platform_fee: Option<String>,
    pub price_impact_pct: Option<String>,
    pub route_plan: Vec<RoutePlan>,
    pub context_slot: u64,
    pub time_taken: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutePlan {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInfo {
    pub amm_key: String,
    pub label: Option<String>,
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: String,
    pub out_amount: String,
    pub fee_amount: String,
    pub fee_mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JupiterSwapResponse {
    pub swap_transaction: String,
}

// Token account information
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenAccountInfo {
    pub mint: String,
    pub owner: String,
    pub amount: f64,
    pub decimals: u8,
}

// Transaction info
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub signature: String,
    pub slot: u64,
    pub timestamp: Option<i64>,
    pub err: Option<solana_sdk::transaction::TransactionError>,
}

// Token balance
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBalance {
    pub mint: String,
    pub balance: f64,
    pub decimals: u8,
}

// Solana client service
pub struct SolanaClient {
    rpc_client: RpcClient,
    program_id: String,
    http: HttpClient,
}

impl SolanaClient {
    pub fn new(config: &SolanaConfig) -> Self {
        tracing::info!("🔗 Connecting to Solana RPC: {}", config.rpc_url);
        tracing::info!("📝 Using program ID: {}", config.program_id);

        let commitment = match config.commitment.as_str() {
            "processed" => CommitmentConfig::processed(),
            "confirmed" => CommitmentConfig::confirmed(),
            "finalized" => CommitmentConfig::finalized(),
            _ => {
                tracing::warn!(
                    "Unknown commitment level: {}, using 'confirmed'",
                    config.commitment
                );
                CommitmentConfig::confirmed()
            }
        };

        Self {
            rpc_client: RpcClient::new_with_commitment(config.rpc_url.clone(), commitment),
            program_id: config.program_id.clone(),
            http: HttpClient::new(),
        }
    }

    // Get token accounts for a wallet
    pub async fn get_token_accounts(
        &self,
        wallet: &str,
    ) -> Result<Vec<TokenAccountInfo>, ThreadSafeError> {
        let pubkey = Pubkey::from_str(wallet)?;

        let accounts = self
            .rpc_client
            .get_token_accounts_by_owner(&pubkey, TokenAccountsFilter::ProgramId(spl_token::id()))
            .await?;

        let mut token_accounts = Vec::new();

        for RpcKeyedAccount { account, .. } in accounts {
            // FIX: Handle different account data formats properly
            match &account.data {
                solana_account_decoder::UiAccountData::Json(parsed_data) => {
                    if let Some(info) = parsed_data.parsed.get("info") {
                        let mint = info.get("mint").and_then(|v| v.as_str()).unwrap_or("");
                        let owner = info.get("owner").and_then(|v| v.as_str()).unwrap_or("");
                        let amount = info
                            .get("tokenAmount")
                            .and_then(|t| t.get("amount"))
                            .and_then(|a| a.as_str())
                            .and_then(|s| s.parse::<f64>().ok())
                            .unwrap_or(0.0);
                        let decimals = info
                            .get("tokenAmount")
                            .and_then(|t| t.get("decimals").and_then(|d| d.as_u64()))
                            .unwrap_or(0) as u8;

                        token_accounts.push(TokenAccountInfo {
                            mint: mint.to_string(),
                            owner: owner.to_string(),
                            amount,
                            decimals,
                        });
                    }
                }
                // Handle other account data formats if needed
                _ => {
                    tracing::warn!("Unsupported account data format for wallet: {}", wallet);
                }
            }
        }

        Ok(token_accounts)
    }

    // Execute swap transaction using Jupiter
    pub async fn execute_swap_transaction(
        &self,
        wallet: &str,
        input_mint: &str,
        output_mint: &str,
        amount: f64,
        slippage_bps: u64,
    ) -> Result<String, ThreadSafeError> {
        tracing::info!(
            "🔄 Executing swap: {} {} -> {} (slippage: {} bps, wallet: {})",
            amount,
            input_mint,
            output_mint,
            slippage_bps,
            wallet
        );

        // Convert amount to string (assuming 6 decimals)
        let amount_string = format!("{:.0}", amount * (10f64).powi(6));

        // Get quote from Jupiter
        let quote = self
            .get_jupiter_quote(input_mint, output_mint, &amount_string, slippage_bps)
            .await?;

        tracing::info!(
            "📊 Jupiter quote received: {} -> {}",
            quote.in_amount,
            quote.out_amount
        );

        // Get swap transaction from Jupiter
        let swap_response = self
            .get_jupiter_swap_transaction(wallet, &quote, slippage_bps)
            .await?;

        // Send the transaction
        let signature = self
            .send_jupiter_transaction(&swap_response.swap_transaction)
            .await?;

        tracing::info!(
            "✅ Swap transaction submitted with signature: {}",
            signature
        );

        Ok(signature)
    }

    // Get quote from Jupiter API
    async fn get_jupiter_quote(
        &self,
        input_mint: &str,
        output_mint: &str,
        amount: &str,
        slippage_bps: u64,
    ) -> Result<JupiterQuote, ThreadSafeError> {
        let url = format!(
            "https://quote-api.jup.ag/v6/quote?inputMint={}&outputMint={}&amount={}&slippageBps={}",
            input_mint, output_mint, amount, slippage_bps
        );

        let response = self.http.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(format!("Jupiter API error: {}", response.status()).into());
        }

        let quote: JupiterQuote = response.json().await?;
        Ok(quote)
    }

    // Get swap transaction from Jupiter API
    async fn get_jupiter_swap_transaction(
        &self,
        wallet: &str,
        quote: &JupiterQuote,
        slippage_bps: u64,
    ) -> Result<JupiterSwapResponse, ThreadSafeError> {
        let url = "https://quote-api.jup.ag/v6/swap".to_string();

        let request_body = json!({
            "quoteResponse": quote,
            "userPublicKey": wallet,
            "slippageBps": slippage_bps,
        });

        let response = self
            .http
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Jupiter swap API error: {}", error_text).into());
        }

        let swap_response: JupiterSwapResponse = response.json().await?;
        Ok(swap_response)
    }

    // Send the Jupiter transaction to Solana network
    async fn send_jupiter_transaction(
        &self,
        swap_transaction: &str,
    ) -> Result<String, ThreadSafeError> {
        // Decode the base64 transaction
        let transaction_data = base64_standard.decode(swap_transaction)?;

        // Deserialize the transaction
        let transaction: Transaction = bincode::deserialize(&transaction_data)?;

        // Send the transaction
        let signature = self.rpc_client.send_transaction(&transaction).await?;

        Ok(signature.to_string())
    }

    // Alternative method with signer
    pub async fn execute_swap_transaction_with_signer(
        &self,
        wallet_keypair: &Keypair,
        input_mint: &str,
        output_mint: &str,
        amount: f64,
        slippage_bps: u64,
    ) -> Result<String, ThreadSafeError> {
        let amount_string = format!("{:.0}", amount * (10f64).powi(6));
        let wallet_address = wallet_keypair.pubkey().to_string();

        // Get quote
        let quote = self
            .get_jupiter_quote(input_mint, output_mint, &amount_string, slippage_bps)
            .await?;

        // Get swap transaction
        let swap_response = self
            .get_jupiter_swap_transaction(&wallet_address, &quote, slippage_bps)
            .await?;

        // Decode and sign transaction
        let transaction_data = base64_standard.decode(&swap_response.swap_transaction)?;
        let mut transaction: Transaction = bincode::deserialize(&transaction_data)?;

        // Get recent blockhash and sign
        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;
        transaction.sign(&[wallet_keypair], recent_blockhash);

        // Send signed transaction
        let signature = self.rpc_client.send_transaction(&transaction).await?;

        Ok(signature.to_string())
    }

    // Fetch 24h volatility for a given token mint using Coingecko
    pub async fn get_token_volatility(&self, mint: &str) -> f64 {
        let (coingecko_id, default_vol) = match mint {
            "So11111111111111111111111111111111111111112" | "So11111111111111111111111111111111111111111" => ("solana", 0.05),
            "Es9vMFrzaCERz8iYwByJ3Q6sX6ixSeKuuNHYsYAGP6X" => ("usd-coin", 0.002),
            _ => ("solana", 0.05),
        };

        let url = format!(
            "https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days=1",
            coingecko_id
        );

        let response = match self.http.get(&url).send().await {
            Ok(r) => r,
            Err(_) => return default_vol,
        };

        let json: serde_json::Value = match response.json().await {
            Ok(j) => j,
            Err(_) => return default_vol,
        };

        let prices = match json.get("prices").and_then(|v| v.as_array()) {
            Some(p) if p.len() >= 2 => p,
            _ => return default_vol,
        };

        let first = prices.first().and_then(|p| p.get(1)).and_then(|v| v.as_f64()).unwrap_or(0.0);
        let last = prices.last().and_then(|p| p.get(1)).and_then(|v| v.as_f64()).unwrap_or(0.0);

        if first > 0.0 && last > 0.0 {
            ((last - first) / first).abs().clamp(0.0, 1.0)
        } else {
            default_vol
        }
    }
}

impl Clone for SolanaClient {
    fn clone(&self) -> Self {
        Self {
            rpc_client: RpcClient::new(self.rpc_client.url().to_string()),
            program_id: self.program_id.clone(),
            http: HttpClient::new(),
        }
    }
}
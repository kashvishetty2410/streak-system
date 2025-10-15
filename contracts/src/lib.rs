// Soroban smart contract for a daily reward streak system
// Playful names and ASCII logs included. This is a simplified example using soroban-sdk.
#![no_std]

use soroban_sdk::{contractimpl, symbol, vec, Env, String, Vec};

pub struct StreakContract;

#[contractimpl]
impl StreakContract {
    // Claim rewards for `username`. `now_ts` is the current unix timestamp (seconds).
    // Returns (reward, new_streak).
    pub fn claim(env: Env, username: String, now_ts: u64) -> (i128, u32) {
        env.log(format!("🎉 claim called for {} at {} 🎉", username, now_ts));

        // Storage keys are tuples so each username has separate fields
        let streak_key = (symbol!("streak"), &username);
        let last_key = (symbol!("last"), &username);
        let total_key = (symbol!("total"), &username);

        let mut streak: u32 = env.storage().get(&streak_key).unwrap_or(0u32);
        let last_claim: u64 = env.storage().get(&last_key).unwrap_or(0u64);
        let mut total: i128 = env.storage().get(&total_key).unwrap_or(0i128);

        // Prevent multiple claims within 24h
        if last_claim != 0 && now_ts <= last_claim || (last_claim != 0 && now_ts - last_claim < 24 * 3600) {
            env.log("⏳ Too soon! You can only claim once every 24 hours.");
            return (0i128, streak);
        }

        // Determine streak continuation (within 48h keeps streak)
        if last_claim != 0 && now_ts - last_claim <= 48 * 3600 {
            streak = streak.saturating_add(1);
        } else {
            streak = 1; // reset or start
        }

        // reward = 1 * 2^streak, capped at 1024
        let mut reward: i128 = 1i128.checked_shl(streak).unwrap_or(1024) as i128;
        if reward > 1024 { reward = 1024; }

        // Surprise multipliers
        if streak == 7 { reward *= 2; env.log("🎊 7-day milestone bonus applied!"); }
        if streak == 14 { reward *= 3; env.log("🎊 14-day mega bonus applied!"); }
        if streak == 30 { reward *= 5; env.log("🏅 30-day legend bonus applied!"); }

        // Easter eggs
        if username.contains("Dragon") {
            reward += 42;
            env.log("🐉 Dragon bonus +42 XLM applied!");
        }
        if username.contains("Unicorn") {
            // fun unicorn confetti log
            env.log("🦄✨ Unicorn confetti activated! Sparkles everywhere!");
        }

        total = total.saturating_add(reward);

        // Save back to storage
        env.storage().set(&streak_key, &streak);
        env.storage().set(&last_key, &now_ts);
        env.storage().set(&total_key, &total);

        // ASCII fireworks log
        env.log("🎆 Boom! Rewards claimed. Fireworks for you! 🎆");

        (reward, streak)
    }

    // Get user's streak info: (streak, last_claim_ts, total_rewards)
    pub fn get_streak(env: Env, username: String) -> (u32, u64, i128) {
        let streak_key = (symbol!("streak"), &username);
        let last_key = (symbol!("last"), &username);
        let total_key = (symbol!("total"), &username);

        let streak: u32 = env.storage().get(&streak_key).unwrap_or(0u32);
        let last_claim: u64 = env.storage().get(&last_key).unwrap_or(0u64);
        let total: i128 = env.storage().get(&total_key).unwrap_or(0i128);
        env.log(format!("🔎 get_streak for {} -> streak {}", username, streak));
        (streak, last_claim, total)
    }

    // Leaderboard: returns a vector of strings describing top streaks (simple implementation)
    pub fn leaderboard(env: Env, limit: u32) -> Vec<String> {
        let mut v: Vec<String> = Vec::new(&env);
        // Static placeholder leaderboard — in a real contract you'd maintain a sorted list
        v.push_back("1️⃣ Alice - 30 days 🏆".into());
        v.push_back("2️⃣ Bob - 14 days 🎉".into());
        v.push_back("3️⃣ Carol - 7 days 🥳".into());
        env.log("📜 Leaderboard requested");
        v
    }
}

// Boilerplate: export the contract
soroban_sdk::contractimport!();

use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use gloo_timers::callback::Timeout;

// API Types
#[derive(Serialize)]
struct ClaimRequest {
    username: String,
}

#[derive(Deserialize, Clone, PartialEq)]
struct ClaimResponse {
    reward: i128,
    streak: u32,
    message: String,
}

#[derive(Deserialize, Clone, PartialEq)]
struct LeaderItem {
    username: String,
    streak: u32,
}

// Main App Component
#[function_component(App)]
pub fn app() -> Html {
    let username = use_state(|| "PlayerOne".to_string());
    let status = use_state(|| "Ready to claim ✨".to_string());
    let loading = use_state(|| false);
    let leaderboard = use_state(|| Vec::<LeaderItem>::new());
    let show_confetti = use_state(|| false);

    let onclaim = {
        let username = username.clone();
        let status = status.clone();
        let loading = loading.clone();
        let leaderboard = leaderboard.clone();
        let show_confetti = show_confetti.clone();

        Callback::from(move |_| {
            let username = (*username).clone();
            let status = status.clone();
            let loading = loading.clone();

            // Clone handles inside the callback so each invocation moves fresh clones into the async task
            let leaderboard_handle = leaderboard.clone();
            let confetti_handle = show_confetti.clone();

            loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                status.set("Claiming... confetti incoming 🎉".to_string());

                let req = ClaimRequest { username: username.clone() };

                match Request::post("http://127.0.0.1:8080/claim")
                    .json(&req)
                    .unwrap()
                    .send()
                    .await {
                        Ok(resp) => {
                            match resp.json::<ClaimResponse>().await {
                                Ok(data) => {
                                    status.set(format!("🎉 Won {} XLM! Streak: {} - {}", 
                                        data.reward, data.streak, data.message));
                                    // show confetti briefly
                                    confetti_handle.set(true);
                                    // update leaderboard optimistic
                                    let mut next = (*leaderboard_handle).clone();
                                    next.insert(0, LeaderItem { username: username.clone(), streak: data.streak });
                                    if next.len() > 10 { next.truncate(10) }
                                    leaderboard_handle.set(next);
                                }
                                Err(e) => status.set(format!("Error parsing response: {} 😢", e)),
                            }
                        }
                        Err(e) => status.set(format!("Network error: {} 😢", e)),
                };
                loading.set(false);
            });
        })
    };

    // fetch leaderboard on mount
    {
        let leaderboard = leaderboard.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://127.0.0.1:8080/leaderboard").send().await {
                    Ok(resp) => match resp.json::<Vec<LeaderItem>>().await {
                        Ok(list) => leaderboard.set(list),
                        Err(_) => (),
                    },
                    Err(_) => (),
                }
            });
            || ()
        }, ());
    }

    // effect to trigger confetti DOM injection and auto-hide
    {
        use_effect_with_deps(move |show| {
            if **show {
                // simple confetti by injecting small colored divs
                if let Some(window) = web_sys::window() {
                    if let Some(doc) = window.document() {
                        if let Some(container) = doc.get_element_by_id("confetti") {
                            // create a burst
                            for i in 0..40 {
                                let el = doc.create_element("div").unwrap();
                                let size = 6 + (i % 6) as i32;
                                el.set_attribute("style", &format!(
                                    "position:absolute;left:{}%;top:{}%;width:{}px;height:{}px;background:{};opacity:0.95;transform:rotate({}deg);border-radius:2px;",
                                    (i*7)%100,
                                    (i*3)%80,
                                    size,
                                    size,
                                    random_color(i),
                                    (i*37)%360,
                                )).unwrap();
                                container.append_child(&el).ok();
                                // float away using timeout removal
                                let el2 = el.clone();
                                Timeout::new(2500, move || {
                                    el2.remove();
                                }).forget();
                            }
                        }
                    }
                }
                // hide after 2.5s
                let sc2 = show.clone();
                Timeout::new(2600, move || sc2.set(false)).forget();
            }
            || ()
        }, show_confetti.clone());
    }

    html! {
        <div class="shell">
            <header class="app-header">
                <div>
                    <h1>{"🎮 Streakinator3000"}</h1>
                    <p class="small">{"Daily rewards with fun! Try usernames with 'Dragon' or 'Unicorn' 🦄"}</p>
                </div>
                <div style="margin-left:auto;text-align:right">
                    <div class="small">{"Backend: http://127.0.0.1:8080"}</div>
                </div>
            </header>

            <section class="card">
                <div class="controls">
                    <div class="input-row">
                        <input type="text"
                            value={(*username).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                username.set(input.value());
                            })}
                            disabled={*loading}
                        />
                        <button class="primary" onclick={onclaim} disabled={*loading}>{ if *loading { "Claiming..." } else { "Claim Daily Reward! 🎁" } }</button>
                    </div>

                    <div class="status">{(*status).clone()}</div>

                    <div class="ascii-box">
                        {"🎆 Fireworks — Keep your streak alive!"}
                    </div>
                </div>
            </section>

            <aside class="card">
                <h3 style="margin-top:0">{"Leaderboard"}</h3>
                <div class="leaderboard">
                    { for (*leaderboard).iter().map(|it| html!{
                        <div class="lead-item">
                            <div>{ &it.username }</div>
                            <div class="small">{ format!("{} 🔥", it.streak) }</div>
                        </div>
                    }) }
                </div>
                <div style="height:10px"></div>
                <div class="small">{"Top claimers — updated when you claim."}</div>
            </aside>
        </div>
    }
}

fn random_color(i: usize) -> String {
    // deterministic-ish palette
    let palette = ["#ff7ab6","#ffd166","#06d6a0","#84ccff","#b388eb","#ff9f43"];
    palette[i % palette.len()].to_string()
}
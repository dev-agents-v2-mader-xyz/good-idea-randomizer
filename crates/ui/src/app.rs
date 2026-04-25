use yew::prelude::*;

const DO_IT_REASONS: &[&str] = &[
    "Mercury is in retrograde and that means it's time to act.",
    "The universe has been waiting for this exact moment.",
    "A wise man once said: just do it. He was right.",
    "Your horoscope is suspiciously on board with this.",
    "The stars aligned three weeks ago specifically for this.",
    "Every great story starts exactly this way.",
    "You'll regret the chances you didn't take more than the ones you did.",
    "A butterfly in Brazil just flapped its wings to make this happen.",
    "The oracle has spoken. The oracle is never wrong.",
    "This is your villain origin story and it's going to be great.",
    "Statistically, 9 out of 10 people who did this had no regrets.",
    "Your gut said yes before your brain could object.",
    "Fortune favors the bold. Be bold.",
    "The best time to do this was yesterday. The second best time is now.",
    "Someone out there is doing this right now and loving it.",
    "Worst case scenario: it becomes a great story.",
    "Your ancestors survived worse. You've got this.",
    "The omens are unclear, which historically means green light.",
    "Everything you want is on the other side of doing this.",
    "An ancient prophecy foretold this moment. Probably.",
    "Science has shown that people who hesitate lose. Don't lose.",
    "You've been thinking about this too long. That means it matters.",
    "The vibes? Immaculate.",
    "This is literally what YOLO was invented for.",
    "A passing cloud just looked like a thumbs up. Sign accepted.",
    "Your future self is already thanking you.",
    "The oracle checked twice. Still yes.",
    "Comfort zones are overrated. Step outside yours.",
    "Today is the day. It was always going to be today.",
    "Three coin flips, three heads. The universe insists.",
];

const DONT_REASONS: &[&str] = &[
    "Your future self will look back and be thankful you didn't.",
    "The vibes are off.",
    "Three separate omens this week said no.",
    "A suspicious number of things could go wrong here.",
    "Mercury is in retrograde. Not the time.",
    "The oracle had a bad feeling about this one.",
    "Your gut said no before your brain could object.",
    "Some doors, once opened, cannot be closed.",
    "The energy in the room shifted the moment you thought of this.",
    "History has a pattern here. This is the pattern.",
    "A wise elder warned against exactly this.",
    "The stars are misaligned in a very specific way for this.",
    "That thing you're ignoring? That's the sign.",
    "Your horoscope used the word 'caution' four times this week.",
    "Every great cautionary tale starts exactly this way.",
    "Sleep on it. And then sleep on it again.",
    "The oracle checked. The oracle is still no.",
    "Someone out there did this. They no longer speak of it.",
    "Your ancestors faced this exact choice. They chose wisely.",
    "The butterfly effect analysis does not look favorable.",
    "Not every impulse deserves to become an action.",
    "There is a version of this that ends well. That version is not this one.",
    "The universe said 'not yet' and the universe is patient.",
    "An ancient prophecy warned of this moment. Specifically.",
    "A passing cloud just looked like a skull. Sign rejected.",
    "Future you is begging you to stop.",
    "Some ideas are better as ideas.",
    "The math doesn't add up, and neither do the vibes.",
    "Comfort exists for a reason. Stay comfortable.",
    "Three coin flips, three tails. The universe insists.",
];

#[cfg(target_arch = "wasm32")]
fn rand_float() -> f64 {
    js_sys::Math::random()
}

#[cfg(not(target_arch = "wasm32"))]
fn rand_float() -> f64 {
    0.42
}

fn pick_verdict() -> (bool, &'static str) {
    let do_it = rand_float() < 0.5;
    let list = if do_it { DO_IT_REASONS } else { DONT_REASONS };
    let idx = (rand_float() * list.len() as f64) as usize;
    (do_it, list[idx.min(list.len() - 1)])
}

#[derive(Clone, PartialEq)]
struct VerdictResult {
    do_it: bool,
    reason: &'static str,
}

#[function_component(App)]
pub fn app() -> Html {
    let input = use_state(String::new);
    let verdict = use_state(|| Option::<VerdictResult>::None);

    let on_input = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            #[cfg(target_arch = "wasm32")]
            {
                use wasm_bindgen::JsCast;
                if let Some(target) = e.target() {
                    if let Ok(el) = target.dyn_into::<web_sys::HtmlInputElement>() {
                        input.set(el.value());
                    }
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (e, &input);
        })
    };

    let on_submit = {
        let verdict = verdict.clone();
        let input = input.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            if !input.trim().is_empty() {
                let (do_it, reason) = pick_verdict();
                verdict.set(Some(VerdictResult { do_it, reason }));
            }
        })
    };

    let on_again = {
        let verdict = verdict.clone();
        Callback::from(move |_: MouseEvent| {
            let (do_it, reason) = pick_verdict();
            verdict.set(Some(VerdictResult { do_it, reason }));
        })
    };

    let input_empty = input.trim().is_empty();

    html! {
        <div class="page">
            <div class="card">
                <div class="oracle-eye">{"🔮"}</div>
                <h1 class="heading">{"Is this a good idea?"}</h1>
                <p class="subheading">{"Ask the oracle. Receive the truth."}</p>

                <form class="form" onsubmit={on_submit}>
                    <input
                        class="input"
                        type="text"
                        placeholder="What are you thinking of doing?"
                        value={(*input).clone()}
                        oninput={on_input}
                    />
                    <button
                        class="btn-primary"
                        type="submit"
                        disabled={input_empty}
                    >
                        {"Ask the oracle"}
                    </button>
                </form>

                {
                    if let Some(v) = (*verdict).as_ref() {
                        let verdict_class = if v.do_it { "verdict verdict--yes" } else { "verdict verdict--no" };
                        let verdict_label = if v.do_it { "YES — DO IT" } else { "NO — DON'T" };
                        let emoji = if v.do_it { "✨" } else { "🚫" };
                        html! {
                            <div class={verdict_class}>
                                <div class="verdict-emoji">{emoji}</div>
                                <div class="verdict-label">{verdict_label}</div>
                                <p class="verdict-reason">{v.reason}</p>
                                <button class="btn-again" onclick={on_again}>
                                    {"Ask again"}
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_verdict_returns_valid_reason() {
        let (do_it, reason) = pick_verdict();
        if do_it {
            assert!(DO_IT_REASONS.contains(&reason));
        } else {
            assert!(DONT_REASONS.contains(&reason));
        }
    }

    #[test]
    fn reason_lists_have_thirty_entries() {
        assert_eq!(DO_IT_REASONS.len(), 30);
        assert_eq!(DONT_REASONS.len(), 30);
    }
}

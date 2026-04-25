# SPEC — Good Idea Randomizer

## Overview
A purely fun, zero-state single-page web app. The user types in a decision they're considering, clicks a button, and receives a random but confident-sounding verdict — either a reason to do it or a reason not to. No accounts, no persistence, no server logic. Reloading resets everything.

## Data Model
None. Zero state. All randomisation happens client-side in the browser at runtime.

## API Surface
None. This is a fully static frontend application. No backend routes.

## UI Pages

### `/` — Main Page (the only page)
- Large, playful heading: "Is this a good idea?"
- Text input: "What are you thinking of doing?" (placeholder)
- Submit button: "Ask the oracle" (or similar fun label)
- On submit: pick a random verdict (DO IT / DON'T DO IT) and a random confident-sounding reason from a hardcoded list, display both prominently
- "Ask again" button to re-roll without clearing the input
- No auth required, no loading state, instant result

**Verdict generation logic (client-side):**
- 50/50 random split: "YES — DO IT" vs "NO — DON'T"
- ~30 hardcoded reasons per verdict direction (confident, slightly absurd, fun)
- Examples DO IT: "Mercury is in retrograde and that means it's time to act.", "The universe has been waiting for this exact moment.", "A wise man once said: just do it. He was right."
- Examples DON'T: "Your future self will look back and be thankful you didn't.", "The vibes are off.", "Three separate omens this week said no."

## Auth
None.

## Third-Party Services
- Stripe: no
- Email: no
- Notion: no
- CSV import: no

## Target Server
default (IP: 91.98.146.113)
Live URL: https://good-idea-randomizer.mader.xyz

## Tech Stack
- Pure HTML + CSS + vanilla JavaScript (or a lightweight framework like Preact/Vue if the template uses one)
- Deployed as a static site via the existing Docker/nginx reverse-proxy setup on the target server
- No database, no Supabase schema, no environment secrets needed

## Open Questions
- [ASSUMPTION: plain HTML/CSS/JS] The pitch says "zero state" so no framework is strictly needed; a simple static HTML file served by nginx is sufficient. If the project template uses Yew/Rust we will use it, otherwise vanilla JS is preferred for speed.
- [ASSUMPTION: hardcoded reasons] Reason lists are hardcoded in the JS bundle — no AI API call, keeping it instant and free to run.
- [ASSUMPTION: single page] No routing needed; one index.html.

# Shakedown checklist

Run one real (small) piece of work through the whole loop. The goal is to find
friction in the skill docs, not to ship the work fast. Fix docs as you go.

## Delivery half (your daily loop — do this first)

- [ ] `investigate` a real idea in a real repo. Check: topic folder + `.topic`
      marker created under the vault; report is tiered, not a wall; approaches
      are credible (or a single obvious one, stated as such); it waited for
      **your** approach choice; `research.md` + `decisions.md` written.
- [ ] `design` the chosen approach. Check: it spiked the riskiest assumption
      (or explicitly said nothing was risky); acceptance criteria are
      behavior, not steps; PR count is minimal with a clear reason for each split; it required
      an explicit "approved"; `design.md` + stamp in `_state.md`.
- [ ] `execute`. Check: red commit exists **before** implementation commits;
      no test was weakened post-red without a `decisions.md` line; drift
      handled per policy; it reported deviations in chat and opened PRs
      automatically after verification; PR matches `_shared/pr.md`, includes
      accessible behavior evidence, and omits local vault links and design
      deviation commentary.
- [ ] `self-review`. Check: it read `decisions.md` first; deliberate
      deviations appear under "Verified deliberate" in the artifact, not as findings;
      test-integrity pass ran against the red commits; merge-blockers ranked
      above named one-liner nits.
- [ ] `review-fix`. Check: every finding got a verdict before any fix; plain-language fix
      plan gated on you; one commit per fix; PR threads replied to and
      resolved — the trail is visible on GitHub.

## Planning half

- [ ] `wayfinder` with a fuzzy idea. Check: map + decision tickets created on
      the tracker with real blocking edges; tickets carry `wayfinder:<type>`
      labels; resolutions land as comments + close + map index line.
- [ ] Resolve at least one grilling ticket and one research ticket.
- [ ] `to-spec` → `to-tickets` from the cleared map. Check: vertical slices,
      blocking edges wired, `ready-for-agent` labels, Context link to the
      topic folder.

## Cross-cutting

- [ ] Skills read `_shared/style.md` immediately before reports and decision
      requests, including revised reports after further work.
- [ ] Chat reports explained the features, key findings, recommendation, and
      trade-offs without requiring "in simple terms" or "give me an example".
      No wall of sections; full evidence and minor details remained in artifacts.
- [ ] A second session in the same worktree self-located the topic without
      being told.
- [ ] Switching `tracker:` in `config.md` is the only step needed to move
      trackers (spot-check by reading the other recipes file).

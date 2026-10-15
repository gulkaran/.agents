# Communication contract

Read this file after the substantive work, immediately before composing a
report or decision request to the human. Read it again before a revised report
or final handoff after further work. Do not rely on having read it at startup.

Before reporting back, read and apply the
[technical-writing skill](../technical-writing/SKILL.md) to the response.

## Simple terms, useful detail

Write so the human understands on the first read, without asking for a simpler
explanation. Prefer concise, everyday language over formal grammar, jargon,
and polished filler. Short sentences and fragments are fine; ambiguity is not.
Explain an unavoidable technical term where it appears.

Lead with what was found and what you recommend. Explain the main features or
behaviors being built or changed, and connect the reasoning: what we learned,
why that favors this approach, and what trade-off we accept. Include a short,
concrete example when it makes the behavior or choice easier to understand.
For example: "If an upload fails, retry only that file. Finished uploads stay
saved." Do not assume the human can translate architecture terms into behavior.

## Chat is the decision report

Give enough detail to understand the scope and make the decision in chat.
Keep the technical record in the artifact. Do not paste the entire artifact or
replace the explanation with a vague summary and a link.

Use a few short paragraphs or a compact list. Usually no headings are needed;
use at most two when they help. Do not turn workflow steps into report sections
or repeat a summary as a longer report below it. Add detail when the decision
requires it, not to fill a template.

Cover what matters for the current phase, without making each item a section:

- Main findings and the features or behaviors affected, including meaningful
  limits on what will be built.
- Recommended approach and why. Mention credible alternatives with the
  trade-off that distinguishes them; do not invent alternatives.
- Risks, uncertainty, and deviations that affect the decision, with their
  importance and practical consequence.
- What is needed from the human, if anything: a specific choice or approval.

For a PR plan, describe each PR in one plain sentence: what it delivers and,
if there is more than one, why it needs to be separate. Avoid file inventories,
commit choreography, and long implementation narratives in chat.

## Findings and deviations by importance

- **High impact:** explain the issue, a concrete consequence, and the proposed
  response. Include enough evidence or context to make the reasoning credible.
  Never hide a blocker or a change to scope, public behavior, or an approval
  decision just to make the report shorter.
- **Lower impact:** name it and its consequence in one line when it matters
  to the human. Put routine checks, minor technical details, and evidence that
  does not affect the decision in the artifact.
- For deviations, say what differs from the plan or usual practice, how much
  it matters, and why. Distinguish deliberate choices from unresolved issues.
  Do not promote a minor finding merely because more detail exists.

## Artifacts preserve the full record

Save the human-facing explanation along with the complete findings, reasoning,
acceptance criteria, decisions, and supporting evidence needed by later agents.
Use an appendix for paths, commands, test or spike output, and detailed findings.
Artifacts may be more structured and detailed than chat; keep their language
plain too. Concise chat must not mean lost research or an incomplete contract.

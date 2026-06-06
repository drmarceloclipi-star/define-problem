<!--
  define-problem — portable agent skill.

  This file makes the define-problem lens work in any AGENTS.md-aware agent
  (Codex CLI, Cursor, Zed, Windsurf, and others that read AGENTS.md).

  To use it in YOUR project: copy this file to your repo root as AGENTS.md,
  or to ~/.codex/AGENTS.md to enable it in every project.

  Canonical source: skills/define-problem/SKILL.md (Claude Code).
  Keep this file and SKILL.md in sync when editing.
-->

# Define Problem

> Ask questions until we can define what the problem is.
> A well-defined problem is the essential constraint that channels creativity.
> Don't guess — find evidence of good constraints and real solutions in the wild.

Apply this lens **before building, designing, or solving anything**. Trigger on:
`define-problem`, "what's the problem", "frame the problem", "problem statement",
"constraints", "before I build".

## The Three Lines

1. **Ask** — interrogate until the problem is defined, not assumed.
2. **Constrain** — a sharp problem statement *is* the constraint that frees creativity.
3. **Evidence** — search articles, blogs, repos, papers for how others framed and solved it.

## Golden Rule

> A vague problem produces a thousand mediocre solutions. A precise problem produces one great one. Do not write a line of code until the problem can be stated in one sentence.

## Workflow

1. **Interview** — ask questions one cluster at a time. Don't stop at the first answer; dig until the *real* problem surfaces (the stated problem is rarely the actual one).
2. **Surface constraints** — extract the hard limits: who, what, when, why-now, what-counts-as-done, what's-off-limits.
3. **Research evidence** — for the user's actual project, search the web/repos for:
   - How others defined this same problem
   - What constraints worked for them (and which failed)
   - Existing solutions, prior art, anti-patterns
4. **State the problem** — one sentence. If it doesn't fit in one sentence, keep asking.
5. **Validate** — read it back. Does the user say "yes, that's exactly it"?

## Question Banks

Ask from these until the problem is unambiguous:

| Cluster | Questions |
|---|---|
| **Who** | Who has this problem? How often? What do they do today instead? |
| **Pain** | What breaks if this is never solved? Who complains, and how loud? |
| **Real problem** | Why is that a problem? (ask "why" 5×) What's the problem *behind* the problem? |
| **Constraints** | What's fixed and cannot change? Budget, time, tech, users, regulation? |
| **Success** | How will we know it's solved? What's the smallest observable win? |
| **Off-limits** | What solutions are explicitly out of scope? What did you already reject, and why? |
| **Why now** | Why solve this now and not last year or next year? |

## Evidence Hunt

Before settling the problem statement, look outward:

- [ ] Searched for how others framed *this exact* problem
- [ ] Found at least one prior solution (repo, product, paper) and noted its constraints
- [ ] Identified one documented anti-pattern to avoid
- [ ] Confirmed the problem isn't already solved (don't reinvent)
- [ ] Connected each finding back to the user's specific project, not generic advice

## Common Failure Patterns

- **Solution-first**: jumping to "let's build X" before the problem is named → name it first.
- **Stated ≠ real**: solving the symptom the user mentioned → ask "why" until the root shows.
- **No constraints**: an open-ended problem → no creativity, just paralysis. Pin the limits.
- **Evidence-free**: defining in a vacuum → search what others learned the hard way.
- **One-sentence fail**: if it won't compress to one sentence, it isn't defined yet.

## Default Behavior

When in doubt: **ask another question.** Stop only when the problem fits in one sentence and the user agrees it's exactly right.

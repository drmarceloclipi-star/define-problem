# define-problem

**Stop solving the wrong problem.**

An agent skill that asks you questions until your problem is sharp — then searches the web, papers, and repos for how others framed and solved it.

Runs in **[Claude Code](https://claude.ai/code)**, **Codex CLI**, **Cursor**, **Zed**, **Windsurf**, and **Gemini CLI** — anything that reads the [`AGENTS.md`](https://agents.md) standard.

---

## What it looks like

```
You:    /define-problem my app feels slow

Claude: Slow where — load, navigation, or a specific action?
You:    the dashboard takes forever

Claude: How long, and on what — first load or every visit?
You:    ~8s every visit, it refetches everything

Claude: So the problem isn't "slow app." It's:
        "The dashboard refetches all data on every visit,
         costing 8s even when nothing changed."

        That's cacheable. Here's how 3 teams solved it →
        [evidence from the wild]
```

One vague itch → one precise, solvable problem.

---

## Install

**Claude Code:**

```
/plugin marketplace add drmarceloclipi-star/define-problem
/plugin install define-problem@define-problem
```

Restart Claude Code. Done.

**Codex CLI · Cursor · Zed · Windsurf · Gemini CLI:** grab the portable entrypoint files — no plugin system needed:

```bash
curl -fsSLO https://raw.githubusercontent.com/drmarceloclipi-star/define-problem/main/AGENTS.md
curl -fsSLO https://raw.githubusercontent.com/drmarceloclipi-star/define-problem/main/GEMINI.md
```

Drop in your project root, or in `~/.codex/` / `~/.gemini/` to enable it everywhere. Details below.

<details>
<summary>Alternative: install via Cargo</summary>

Copies the skill into `~/.claude/skills`:

```bash
cargo install --git https://github.com/drmarceloclipi-star/define-problem
define-problem
```

Re-run `define-problem` anytime to update.
</details>

<details>
<summary>Other agents — Codex CLI, Cursor, Zed, Windsurf, Gemini CLI</summary>

The lens ships as portable entrypoint files at the repo root:

| Agent | File | How to use |
|---|---|---|
| Codex CLI, Cursor, Zed, Windsurf | `AGENTS.md` | Copy to your project root, or to `~/.codex/AGENTS.md` for every project |
| Gemini CLI | `GEMINI.md` + `AGENTS.md` | Copy both to your project root, or to `~/.gemini/` |

```bash
# one-liner: grab both files into your current project
curl -fsSLO https://raw.githubusercontent.com/drmarceloclipi-star/define-problem/main/AGENTS.md
curl -fsSLO https://raw.githubusercontent.com/drmarceloclipi-star/define-problem/main/GEMINI.md
```

Any agent that reads the open [`AGENTS.md`](https://agents.md) standard picks it up automatically — no plugin system required.
</details>

---

## Use it

Type `/define-problem` — or just mention the problem and let it trigger:

`what's the problem` · `frame the problem` · `problem statement` · `before I build`

It stops only when the problem fits in **one sentence** and you say *"yes, that's exactly it."*

---

## How it works

1. **Ask** — questions until the problem is defined, not assumed.
2. **Constrain** — a sharp problem statement *is* the constraint that frees creativity.
3. **Evidence** — search articles, blogs, repos, and papers for how others solved it.

> A vague problem produces a thousand mediocre solutions.
> A precise problem produces one great one.

<details>
<summary>Update the plugin</summary>

```
/plugin update define-problem@define-problem
```
</details>

---

<img src="https://github.com/drmarceloclipi-star.png" width="48" align="left" style="margin-right:12px" />

**Dr. Marcelo Cavalcanti** · [@drmarceloclipi-star](https://github.com/drmarceloclipi-star)
MIT License

<br clear="left" />

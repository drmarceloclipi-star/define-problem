# Napkin Runbook

## Curation Rules
- Re-prioritize on every read.
- Keep recurring, high-value notes only.
- Max 10 items per category.
- Each item includes date + "Do instead".

## Execution & Validation (Highest Priority)
1. **[2026-06-02] Validate skill frontmatter after editing**
   Do instead: parse `skills/*/SKILL.md` YAML frontmatter before commit or push.

## Shell & Command Reliability
1. **[2026-06-02] Prefer `rg` for repo scans**
   Do instead: use `rg` or `rg --files` before slower recursive shell searches.

## Domain Behavior Guardrails
1. **[2026-06-02] Skill descriptions can contain YAML-hostile punctuation**
   Do instead: use block scalars for long descriptions with colons or quotes.

## User Directives
1. **[2026-06-02] Caveman mode active**
   Do instead: keep user-facing text terse until user says `stop caveman` or `normal mode`.

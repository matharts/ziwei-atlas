# Issue tracker: GitHub

Issues and specs for this repo live as GitHub issues. Use the `gh` CLI for all operations.

## Conventions

- **Create an issue**: `gh issue create --title "..." --body "..."`. Use a heredoc for multi-line bodies.
- **Read an issue**: `gh issue view <number> --comments`, filtering comments by `jq` and also fetching labels.
- **List issues**: `gh issue list --state open --json number,title,body,labels,comments --jq '[.[] | {number, title, body, labels: [.labels[].name], comments: [.comments[].body]}]'` with appropriate `--label` and `--state` filters.
- **Comment on an issue**: `gh issue comment <number> --body "..."`
- **Apply / remove labels**: `gh issue edit <number> --add-label "..."` / `--remove-label "..."`
- **Close**: `gh issue close <number> --comment "..."`

Infer the repo from `git remote -v` — `gh` does this automatically when run inside a clone.

## Pull requests as a triage surface

**PRs as a request surface: no.**

When enabled, PRs run through the same labels and states as issues using the corresponding `gh pr` commands. GitHub shares one number space across issues and PRs, so resolve ambiguous references with `gh pr view <number>` and fall back to `gh issue view <number>`.

## When a skill says "publish to the issue tracker"

Create a GitHub issue.

## When a skill says "fetch the relevant ticket"

Run `gh issue view <number> --comments`.

## Wayfinding operations

Used by `/wayfinder`. The map is a single issue with child issues as decision tickets.

- **Map**: an issue labelled `wayfinder:map`.
- **Child ticket**: a GitHub sub-issue labelled `wayfinder:<type>`, where type is `research`, `prototype`, `grilling`, or `task`.
- **Blocking**: use GitHub native issue dependencies. If unavailable, add a `Blocked by: #<number>` line.
- **Frontier**: the first unassigned open child without open blockers.
- **Claim**: `gh issue edit <number> --add-assignee @me`.
- **Resolve**: comment with the answer, close the child, then add its context pointer to the map.

---
name: fix-main-issue
description: Take an open GitHub issue filed against main (usually by report-main-fix), fix it on a branch in a worktree, verify it, and open a PR onto main. Use when the user asks to "fix an issue", "solve issue #N", "work through the issues", or "pick up the next fix for main". With no issue number, picks one by the selection rule below.
argument-hint: "[issue number]"
---

# Fix a main issue: $ARGUMENTS

This is the other half of `report-main-fix`: tutor mode files problems with `main`, and this
skill turns one of them into a PR. It is **authoring mode**, so every rule in `CLAUDE.md`
holds: `main` never contains solutions, the root `Cargo.toml` is never edited, and released
learner stubs are never touched.

## 1. Pick the issue

If `$ARGUMENTS` names an issue, use it. Otherwise list the candidates:

```sh
gh issue list --repo jHimanen/learning-rust --state open --limit 100 \
  --json number,title,labels,createdAt,assignees
```

Skip issues that are assigned to someone or already have an open PR
(`gh pr list --state open --search "<n> in:body"`). From the rest, pick by:

1. **`bug` before everything else.** Something on `main` is wrong and every learner hits it.
2. **Then the oldest** (lowest `createdAt`). Older issues have waited longest, and fixing in
   filing order keeps later issues (which may build on earlier ones) coherent.

Tell the learner which issue you picked and why in one line, then carry on; they can redirect
you. If there are no candidates, say so and stop.

If several open issues touch **the same file or section** as the pick, mention them. Fix them
together in one PR only if the learner agrees.

## 2. Understand it

```sh
gh issue view <n> --repo jHimanen/learning-rust --comments
```

- Read the whole thread; later comments may narrow or change the ask.
- Check it's still real on `origin/main` (it may already be fixed by a later chapter or PR).
  If it is, comment with where it was fixed, close it (`gh issue close <n> --comment ...`),
  and go back to step 1.
- The issue's "Suggested fix" is a suggestion. You decide the wording, within the style of
  the surrounding chapter.
- If the fix is ambiguous or big (it changes an exercise's contract, reorders concepts,
  touches a test's expected behaviour), stop and ask the learner before writing.

## 3. Worktree and branch

Never touch the learner's working directory or branch:

```sh
git fetch origin
git worktree add <scratchpad>/fix-<n> -b fix/issue-<n>-<short-slug> origin/main
```

Edit only files inside the worktree.

## 4. Fix

- Follow `.claude/skills/write-chapter/chapter-template.md` for tone and structure, and read
  the chapter's README around the fix so it reads as if it was always there.
- **Stay within the concept ledger** (`.claude/reference/concept-ledger.md`). Adding an
  explanation must not pull in a concept from a later chapter; if it has to, preview it
  briefly and say which chapter covers it. If the fix changes what a chapter introduces,
  update the ledger in the same PR.
- **No solutions.** Grounding (e.g. "chars are Unicode scalar values; here's what that means
  for bytes") is fine; the expression or trick an exercise wants is not. Examples in a README
  work on a *different* problem than the exercise.
- **Learner-owned stubs** (`chapters/*/src/**`, `examples/fixme_*.rs`) are released and frozen.
  Change one only for a critical fix, and then say so prominently in the PR body, because the
  learner will get a merge conflict.
- Tests (`chapters/*/tests/**`) are the spec. Changing one changes what learners must build:
  only do it if the issue is that the test is wrong, and call it out in the PR.
- Keep the diff to what the issue asks. Unrelated problems you spot become new issues
  (via `report-main-fix`), not part of this PR.

## 5. Verify

1. Run the worktree's chapter checks for every chapter you touched:
   ```sh
   <wt>/.claude/skills/write-chapter/verify-chapter.sh chNN-topic
   ```
2. Any command or output you added to a README has been run on this machine and matches.
3. If you changed a test, demo or stub: re-solve the chapter in a throwaway reference copy
   (as in `write-chapter` step 4.3), confirm tests pass and clippy is clean, then delete it.
   Never copy a solution into the worktree.
4. Re-read the diff once as the learner would: does it fix what the issue describes?

## 6. PR, then stop

```sh
git -C <wt> add -A
git -C <wt> commit -m "chNN: <what changed> (fixes #<n>)"
git -C <wt> push -u origin fix/issue-<n>-<short-slug>
gh pr create --repo jHimanen/learning-rust --base main --head fix/issue-<n>-<short-slug> \
  --title "chNN: <what changed>" --body "..."
```

PR body: `Closes #<n>.`, a short summary of the change, what was verified, and a
**⚠ Learner stub / test changed** section if either applies. End with the attribution line
from the system reminder.

Then **stop and ask the learner** whether to merge. On a yes:

```sh
gh pr merge fix/issue-<n>-<short-slug> --repo jHimanen/learning-rust --merge --delete-branch
git worktree remove <wt> && git branch -D fix/issue-<n>-<short-slug>
```

Then tell the learner to run `git pull --no-rebase --no-edit origin main` on their work branch
(or run it for them if they ask), and mention how many open issues remain.

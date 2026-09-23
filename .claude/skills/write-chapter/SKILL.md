---
name: write-chapter
description: Author a chapter of this Rust tutorial (authoring mode). Use when the user asks to "write chapter N", author/draft the next chapter, or ship a fix to a released chapter on main. Covers the worktree + PR workflow, chapter layout and README template, exercise design, verification with a reference solution, and updating the concept ledger and canonical minipolars API.
argument-hint: <chapter number>
---

# Write chapter $ARGUMENTS

You are in **authoring mode**. (No number given? Take the first chapter in `SYLLABUS.md` that
isn't marked "available", and confirm it with the learner.) The file-ownership rules in
`CLAUDE.md` still apply: `main` never contains solutions, never edits the root `Cargo.toml`,
and never touches released learner stubs.

Files next to this skill:
- `chapter-template.md`: directory layout, `Cargo.toml` template, README structure, exercise design.
- `verify-chapter.sh`: the mechanical checks.

Shared references (you update these as part of the chapter):
- `.claude/reference/concept-ledger.md`: what each chapter introduces and previews.
- `.claude/reference/canonical-api.md`: the minipolars names milestone tests must use.

## 1. Worktree

Never touch the learner's working directory or branch. Work in a worktree in your scratchpad,
and edit only the worktree's copies of files (including the ledger and API files):

```sh
git fetch origin
git worktree add <scratchpad>/author-chNN -b chapter/chNN-topic origin/main
```

## 2. Before writing

1. Read `SYLLABUS.md` (the entry for this chapter and its neighbours) and the concept ledger.
   Don't use a concept before the chapter that introduces it. If an exercise really needs
   one, preview it briefly and say which chapter covers it properly.
2. Read the previous chapter's README to match tone and continuity.
3. Read the canonical API. Milestone tests must use exactly those names.
4. Optionally, read the learner's branch (their solutions, `project/minipolars`, `notes/`) to
   see what was hard and adapt the explanations. The chapter must still work for any learner
   starting from `main`: never reference their specific code, and use only the canonical API.

## 3. Write

Follow `chapter-template.md`.

## 4. Verify before opening the PR

1. Mechanical checks. Run the **worktree's** copy of the script:
   ```sh
   <wt>/.claude/skills/write-chapter/verify-chapter.sh chNN-topic
   ```
   It checks the manifest, `cargo build`/`cargo clippy --workspace --all-targets` (ch02's 2
   deliberate clippy warnings are expected), that every test fails with `not yet
   implemented`, that every demo and fixme has a README command, runs every `cargo run`
   command in the README and prints the output, checks that every fixme fails to compile
   (printing the error codes), and runs `cargo fmt --check`. Fix everything it reports.
2. Read the printed demo output against what the README claims, and the fixme errors against
   the error each fixme is meant to teach.
3. **Reference solution:** copy the chapter crate to a throwaway directory outside the
   worktree (e.g. `<scratchpad>/ref-chNN/`, as a standalone package with `[workspace]` added
   to its Cargo.toml). Solve the stubs there. `cargo test` passes and
   `cargo clippy --all-targets -- -D warnings` is clean. Also solve the milestone against a
   minimal reference minipolars that follows the canonical API. **Never copy any solution
   into the worktree.** Delete the throwaway directory afterwards.
4. Every other shell command in the README (`otool`, `nm`, `rustc`, ...) has been run on this
   machine and behaves as described.
5. Update `SYLLABUS.md` (mark the chapter "available"), the concept ledger, and the canonical
   API.

## 5. PR, then stop

```sh
git -C <wt> add -A && git -C <wt> commit -m "Add chapter NN: <title>"
git -C <wt> push -u origin chapter/chNN-topic
gh pr create --repo jHimanen/learning-rust --base main --head chapter/chNN-topic --title "..." --body "..."
```

Then **stop and ask the learner** whether to merge. On a yes:

```sh
gh pr merge chapter/chNN-topic --repo jHimanen/learning-rust --merge --delete-branch
git worktree remove <wt> && git branch -D chapter/chNN-topic
```

Then tell the learner to run `git pull --no-rebase --no-edit origin main` on their work branch
(or run it for them if they ask).

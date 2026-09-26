---
name: report-main-fix
description: File a GitHub issue for something on main that should be fixed. Use in tutor mode whenever a chapter README, exercise doc comment, test, demo or fixme turns out to be wrong, unclear or missing context (e.g. the learner is stuck because the README doesn't explain something), or when the learner asks to "note this for main". Never patch main-owned files on a work/* branch; file the issue instead.
---

# Report a fix for main

Tutor mode never edits `main`-owned files on a `work/*` branch (see `CLAUDE.md`). A note left
only in chat is lost when the session ends, so every "this should be fixed on `main`" becomes
a GitHub issue. The learner has asked for this as a standing habit: file the issue without
asking first, then tell them the link.

## When

- Something in a chapter is **wrong**: a README claim, a doc comment, a demo's output, a test.
- Something is **unclear or missing**: the learner is stuck and the cause is the material, not
  them (e.g. a README section doesn't give them what an exercise needs).
- A **concept-ledger violation**: an exercise needs a concept its chapter hasn't taught.
- The learner asks to note something for `main`.

Not for: the learner's own bugs, style preferences, or ideas for new chapters (unless asked).

## Steps

1. **Check for duplicates:**
   ```sh
   gh issue list --state open --search "chNN"
   ```
   If one already covers it, add a comment (`gh issue comment <n> --body ...`) instead.
2. **Write the issue.** Title: `chNN <file or section>: <the problem>`. Body:
   - **Where:** file path, and the section, function or line.
   - **What happened:** what the learner ran into. Quote their words if they said something.
   - **What's wrong / missing:** concrete, so an author can act on it without this chat.
   - **Suggested fix:** short. The author decides the wording.
   - End with `_Reported from tutor mode on <branch>._`
3. **Never include solutions.** Issues are as public as `main`, and `main` never contains
   solutions. Describe the gap (e.g. "the README never shows the ASCII values"), never the
   answer (e.g. the exact expression or trick an exercise wants).
4. **Label:** `bug` for something wrong, `documentation` for unclear or missing explanation,
   `enhancement` for everything else.
5. **File it** with `gh issue create --title ... --label ... --body ...` (use a heredoc for the
   body), then give the learner the URL in one line and go back to tutoring.

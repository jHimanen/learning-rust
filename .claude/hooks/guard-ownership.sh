#!/usr/bin/env bash
# PreToolUse hook (Edit|Write|MultiEdit|NotebookEdit): backs up the file-ownership rules in
# CLAUDE.md. It never blocks outright. Risky edits become a permission prompt ("ask") that
# states the rule, so the human decides. Needs jq.
#
#   work/* branch (tutor mode)  -> every edit in the repo asks
#   any other branch            -> root Cargo.toml, released learner stubs, and learner-only
#                                  paths (project/minipolars, notes, ch01 playground) ask
#
# Files outside a learning-rust checkout (e.g. a reference solution in the scratchpad) pass.

set -uo pipefail
input=$(cat)
file=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")
[[ -n $file ]] || exit 0

# Write can create directories, so resolve against the nearest one that exists.
dir=$(dirname "$file")
while [[ ! -d $dir ]]; do dir=$(dirname "$dir"); done
prefix=$(git -C "$dir" rev-parse --show-prefix 2>/dev/null) || exit 0
top=$(git -C "$dir" rev-parse --show-toplevel)
[[ -f $top/SYLLABUS.md && -d $top/chapters ]] || exit 0
rel=$prefix${file#"$dir"/}
branch=$(git -C "$dir" branch --show-current)

ask() {
    jq -n --arg r "$1" \
        '{hookSpecificOutput: {hookEventName: "PreToolUse", permissionDecision: "ask", permissionDecisionReason: $r}}'
    exit 0
}

learner_owned=false
case $rel in
notes/README.md) ;;
project/minipolars/* | notes/* | chapters/ch01-compiling/playground/* | chapters/*/src/* | chapters/*/examples/fixme_*.rs)
    learner_owned=true ;;
esac

if [[ $branch == work/* ]]; then
    $learner_owned && ask "$rel is the learner's file. Tutor mode edits it only when the learner explicitly asked."
    ask "$rel belongs to main. Editing it on $branch will conflict with the next 'git pull origin main'; note the fix for main instead."
fi

[[ $rel == Cargo.toml ]] &&
    ask "The root Cargo.toml is never edited. Chapters are picked up by members = [\"chapters/*\", \"project/*\"]; dependencies go in the chapter's own Cargo.toml."

if $learner_owned; then
    git -C "$dir" cat-file -e "origin/main:$rel" 2>/dev/null &&
        ask "$rel is a released learner stub. main never modifies it again (critical fixes only, called out in the PR)."
    case $rel in
    chapters/*/src/* | chapters/*/examples/fixme_*.rs) ;; # a new chapter's stubs
    *) ask "$rel is learner-only (project/minipolars, notes, ch01 playground). main must never ship it." ;;
    esac
fi
exit 0

#!/usr/bin/env bash
# Mechanical checks for a chapter crate: workspace build + clippy, stubs fail at todo!(),
# every README `cargo run` command, fixmes fail to compile, rustfmt.
#
# Usage: <worktree>/.claude/skills/write-chapter/verify-chapter.sh chNN-topic
# It checks the checkout it lives in, so run the worktree's copy, not the learner's.
# Exit status = number of problems. Demo output is printed for you to compare with the README.

set -uo pipefail
pkg=${1:?usage: verify-chapter.sh chNN-topic}
cd "$(dirname "$0")/../../.." || exit 99
dir=chapters/$pkg
[[ -f $dir/Cargo.toml ]] || { echo "no $dir/Cargo.toml (run the copy inside the worktree)"; exit 99; }

problems=0
bad() { echo "  ✗ $*"; problems=$((problems + 1)); }
good() { echo "  ✓ $*"; }
section() { printf '\n== %s\n' "$*"; }
indent() { sed 's/^/    │ /'; }

section "Cargo.toml"
name=$(sed -n 's/^name *= *"\(.*\)"/\1/p' "$dir/Cargo.toml" | head -1)
[[ $name == "$pkg" ]] && good "package name == directory name" || bad "package name is '$name', expected '$pkg'"
grep -q '^edition *= *"2024"' "$dir/Cargo.toml" && good "edition 2024" || bad "edition is not 2024"
git diff --quiet origin/main -- Cargo.toml 2>/dev/null && good "root Cargo.toml untouched" || bad "root Cargo.toml differs from origin/main"

section "cargo build --workspace --all-targets"
if out=$(cargo build --workspace --all-targets 2>&1); then good "builds"
else bad "build failed"; tail -30 <<<"$out" | indent; fi

section "cargo clippy --workspace --all-targets"
out=$(cargo clippy --workspace --all-targets 2>&1)
# One `--> file:line:col` per diagnostic. ch02's lib.rs is deliberately clippy-dirty (2 expected).
locs=$(grep -E '^ *--> ' <<<"$out" | sed -E 's/^ *--> ([^:]+):.*/\1/' | sort | uniq -c)
unexpected=$(grep -v ' chapters/ch02-cargo/src/lib.rs$' <<<"$locs" | grep . || true)
[[ -n $locs ]] && echo "$locs" | indent
if [[ -z $unexpected ]]; then good "no warnings beyond ch02's deliberate ones"
else bad "unexpected clippy/rustc diagnostics (see above)"; fi

section "cargo test -p $pkg (every test must fail at todo!())"
out=$(cargo test -p "$pkg" --no-fail-fast 2>&1)
if grep -qE '^error(\[E[0-9]+\])?:' <<<"$out" && ! grep -q '^test result' <<<"$out"; then
    bad "tests don't compile"; grep -E -A5 '^error' <<<"$out" | head -30 | indent
else
    passed=$(grep -E '^test .* \.\.\. ok$' <<<"$out" || true)
    nfailed=$(grep -cE '^test .* \.\.\. FAILED$' <<<"$out" || true)
    ntodo=$(grep -cE '^not yet implemented' <<<"$out" || true)
    [[ -z $passed ]] && good "no test passes against the stubs" || { bad "tests pass without an implementation:"; echo "$passed" | indent; }
    ((nfailed > 0)) && good "$nfailed tests fail" || bad "no failing tests found"
    ((ntodo == nfailed)) && good "all $nfailed fail with 'not yet implemented'" \
        || bad "$nfailed failures but $ntodo 'not yet implemented' panics: some fail for another reason"
fi

section "README cargo run commands"
cmds=$(grep -oE "cargo run -p $pkg --example [A-Za-z0-9_]+( --[a-z]+)*" "$dir/README.md" | sort -u || true)
for f in "$dir"/examples/demo_*.rs "$dir"/examples/fixme_*.rs; do
    [[ -e $f ]] || continue
    ex=$(basename "$f" .rs)
    grep -qE -- "--example $ex( |$)" <<<"$cmds" || bad "$ex has no \`cargo run\` command in README.md"
done
while read -r cmd; do
    [[ -n $cmd ]] || continue
    ex=$(awk '{for (i = 1; i < NF; i++) if ($i == "--example") print $(i + 1)}' <<<"$cmd")
    [[ $ex =~ [A-Z] ]] && continue # a placeholder like fixme_0N, not a real example
    [[ -e $dir/examples/$ex.rs ]] || { bad "README runs '$ex', but examples/$ex.rs doesn't exist"; continue; }
    case $ex in
    fixme_*)
        grep -A1 "name = \"$ex\"" "$dir/Cargo.toml" | grep -q 'required-features = \["fixme"\]' \
            || bad "$ex has no [[example]] entry with required-features = [\"fixme\"]"
        if out=$(cargo build -p "$pkg" --example "$ex" --features fixme 2>&1); then
            bad "$ex compiles, but it should be broken"
        else
            good "$ex fails to compile: $(grep -oE '^error\[E[0-9]+\]' <<<"$out" | sed -E 's/error\[(.*)\]/\1/' | sort -u | paste -sd' ' -)"
            grep -E '^error' <<<"$out" | grep -v 'could not compile' | indent
        fi
        ;;
    *)
        echo "  \$ $cmd"
        out=$($cmd 2>&1); status=$?
        grep -vE '^ +(Compiling|Finished|Running) ' <<<"$out" | indent
        echo "    (exit $status; compare with what the README claims)"
        ;;
    esac
done <<<"$cmds"

section "cargo fmt --check -p $pkg"
if out=$(cargo fmt --check -p "$pkg" 2>&1); then good "formatted"
else bad "not rustfmt-clean"; head -20 <<<"$out" | indent; fi

section "summary"
((problems == 0)) && echo "  all mechanical checks pass" || echo "  $problems problem(s)"
exit "$problems"

---
description: Fix CI failures, review comments, and bot feedback (Greptile etc.) on the current PR, then iterate until ALL checks pass and Greptile confidence is 5/5 (max 5 attempts)
allowed-tools: Bash(gh *), Bash(git *), Bash(npm *), Bash(npx *), Bash(pnpm *), Bash(yarn *), Bash(bun *), Bash(make *), Bash(cargo *), Bash(docker *), Bash(curl -s *), Bash(TZ=* *)
---

Fix the current pull request by addressing CI failures and review comments. Follow these steps precisely:

## Step 1: Read project instructions

Before doing anything else, read the repository's own conventions and instructions. Check for these files (in order of priority) and follow any guidance they contain:

1. `CLAUDE.md` — Claude-specific project instructions (conventions, build commands, commit style, etc.)
2. `.claude/commands/` — project-specific custom commands (check if there's a repo-specific `fix-pr.md` that should take precedence)
3. `AGENTS.md` — agent/automation instructions
4. `CONTRIBUTING.md` — contribution guidelines
5. `.github/CONTRIBUTING.md` — alternative location for contribution guidelines

If a repo-level `fix-pr` command exists in `.claude/commands/`, defer to it entirely — it knows the project better than this generic version.

Absorb any project-specific conventions (commit message format, required checks, formatting tools, test commands, etc.) and apply them throughout the remaining steps.

## Step 2: Identify the PR

Run `gh pr view --json number,title,url,headRefName,state` to get the current PR details. If there is no PR for this branch, stop and tell the user.

## Step 3: Merge main and resolve conflicts

Ensure the branch is up to date with main before fixing anything:

1. Run `git fetch origin main` to get the latest main
2. Run `git merge origin/main` to merge main into the PR branch
3. If there are merge conflicts:
   - Read each conflicted file and resolve the conflicts intelligently (prefer the PR's intent while incorporating main's updates)
   - Stage the resolved files and complete the merge with `git commit`
4. If the merge is clean, proceed to the next step
5. Push the merge commit before continuing: `git push`

## Step 4: Understand the project

Determine what tools and commands are available for local validation:

1. **Package manager**: detect which is used (npm, pnpm, yarn, bun) by checking lock files
2. **Build system**: check for turbo.json, nx.json, Makefile, Cargo.toml, pyproject.toml, etc.
3. **Available scripts**: read `package.json` scripts (if applicable) to understand what lint, test, build, and check commands exist
4. **Language/runtime**: detect if this is Node.js, Rust, Go, Python, etc.

Based on what's available, determine your validation approach. Prefer running lint, type checks, and tests locally before pushing.

## Step 5: Get CI failures

Run `gh pr checks` to see the status of all checks. If there are failing checks:

1. Use `gh run view <run-id> --log-failed` to get the failure logs for each failed run
2. Analyze each failure carefully — distinguish between:
   - **Build/type errors**: read the relevant source files and fix them
   - **Test failures**: read the test and the code under test, then fix the root cause
   - **Lint errors**: run the project's lint fix command to auto-fix, then check if any remain
   - **Flaky/infra failures**: note these but don't try to fix them

## Step 6: Get review comments and PR comments

First, get the repo owner/name from Step 2. Then fetch **both** review threads and general PR comments.

### 6a: Review threads (inline code review comments)

```bash
gh api graphql -f query='
  query {
    repository(owner: "{owner}", name: "{repo}") {
      pullRequest(number: {number}) {
        reviewThreads(first: 100) {
          nodes {
            id
            isResolved
            comments(first: 10) {
              nodes { id databaseId body author { login } path startLine line }
            }
          }
        }
      }
    }
  }'
```

For each **unresolved** review thread:
1. Read the referenced file and understand the context
2. Make the requested change (or a better alternative if the suggestion has issues)
3. If a comment is unclear or you disagree, note it for the user but still make a best-effort fix
4. After fixing, reply to the comment explaining what you changed:
   ```bash
   gh api repos/{owner}/{repo}/pulls/{number}/comments/{comment_databaseId}/replies \
     -f body="Done — <brief description of the change made>"
   ```
5. Resolve the review thread:
   ```bash
   gh api graphql -f query='
     mutation {
       resolveReviewThread(input: { threadId: "<thread_node_id>" }) {
         thread { isResolved }
       }
     }'
   ```

### 6b: General PR comments (issue-style comments, bot reviews)

Fetch all general comments on the PR (these are NOT inline review threads — they appear in the main conversation):

```bash
gh api repos/{owner}/{repo}/issues/{number}/comments --paginate
```

Process these comments looking for **actionable feedback**, particularly from automated review bots like Greptile AI:

1. **Greptile AI reviews**: Look for comments containing a "Confidence Score". If the confidence score is **less than 5/5**, treat the issues listed in the review as bugs to fix:
   - Read the "Key changes" and "Issue found" sections carefully
   - Address each specific issue mentioned (file paths and line numbers are usually provided)
   - Pay special attention to the "Important Files Changed" table for context
   - After fixing all issues, the goal is for the next Greptile review to return **5/5 confidence**
2. **Other bot comments**: Read and address any actionable feedback from other automated tools (CodeQL, Dependabot, SonarCloud, etc.)
3. **Human comments**: Address any unresolved questions or change requests from human reviewers in the conversation thread

For each issue addressed from PR comments, reply to the comment:
```bash
gh api repos/{owner}/{repo}/issues/{number}/comments \
  -f body="Addressed: <brief description of what was fixed>"
```

## Step 7: Local validation

Run validation locally based on the capabilities detected in Step 4. Follow the project's own instructions from Step 1 if they specify how to run checks.

Otherwise, adapt to the project's tooling. Common patterns:

- **Node.js**: `npm run lint`, `npm run build` or `npm run check`, `npm test`
- **Turborepo**: `npx turbo run lint check test`
- **Rust**: `cargo clippy`, `cargo build`, `cargo test`
- **Go**: `go vet ./...`, `go build ./...`, `go test ./...`
- **Python**: `ruff check .`, `mypy .`, `pytest`
- **Make-based**: `make lint`, `make build`, `make test`

Run each step sequentially — stop and fix issues before proceeding to the next.

Fix any issues found during local validation. After fixing, re-run the failing step to confirm the fix before moving on.

## Step 8: Commit and push

1. Stage all changed files with `git add` (be specific — don't use `git add .`)
2. Create a commit with a descriptive message following the project's commit conventions (from Step 1). Default to conventional commits if no convention is specified.
3. Push to the remote branch

## Step 9: Wait for ALL checks to pass

1. Run `gh pr checks --watch` to wait for **all** checks to complete (CI, Greptile, CodeQL, etc.)
2. After `--watch` returns, run `gh pr checks` one more time to verify the final status of every check — some third-party checks (like Greptile AI) may take longer to complete
3. If any checks are still pending, wait 30 seconds and re-check. Repeat until all checks have a final status
4. If **all** checks pass → proceed to **Step 9c (Verify review comments)**
5. If any checks fail → proceed to **Step 9b (Iterate)**

## Step 9b: Iterate on failures (max 5 attempts)

Track your iteration count starting at 1 (the initial fix cycle counts as iteration 1).

If CI is still failing after Step 9, loop back through the fix cycle:

1. **Increment** the iteration counter
2. **Stop if iteration > 5** — you've exhausted your attempts. Proceed to Step 10 and ask for human help
3. **Analyze** the new failures from `gh run view <run-id> --log-failed` — compare with the previous iteration to understand what changed
4. **Learn from prior iterations** — before making changes, review what you already tried. Do NOT repeat a fix that didn't work. If the same failure persists after your fix, the root cause is different from what you assumed — dig deeper
5. **Fix** the issues (go back to Steps 5–7 as needed)
6. **Re-read PR comments** (Step 6b) — automated review bots like Greptile may have posted new feedback after your push. Address any new issues found
7. **Validate locally** again (Step 7) — catch regressions before pushing
8. **Commit and push** (Step 8) — use a commit message that references the iteration, e.g. `fix(ci): address type error in widget schema (attempt 2)`
9. **Wait for ALL checks** again (Step 9)

### Tips for keeping iteration count low

- **Fix all failures in one pass** — don't fix one thing, push, and wait. Look at ALL failures and fix them together
- **Run the full local validation suite** (Step 7) before pushing — don't rely on remote CI to catch things you could catch locally
- **Read error messages carefully** — most CI failures have clear root causes. Resist the urge to make speculative changes
- **Check for regressions** — each fix can introduce new failures. Re-run the relevant local checks after every change
- **If a failure is flaky/infra** — don't waste an iteration on it. Note it and move on
- **Address Greptile issues proactively** — if Greptile flagged issues, fix them in the same pass as CI failures rather than waiting for the next iteration

## Step 9c: Verify review comments after CI passes

Even when all CI checks pass, re-read PR comments one final time:

1. Fetch PR comments again: `gh api repos/{owner}/{repo}/issues/{number}/comments --paginate`
2. Look for **Greptile AI** reviews — check the confidence score:
   - If confidence is **5/5** → all good, proceed to Step 10
   - If confidence is **less than 5/5** → treat the flagged issues as bugs, fix them, and go back to Step 8 (commit/push/wait). This counts as an iteration
3. Check for any other new review comments or unresolved threads that appeared while CI was running

## Step 10: Report results

Report to the user:
- **Status**: whether the PR is fully green (all checks pass + Greptile confidence 5/5), or you've exhausted iterations
- **Iteration count**: how many fix cycles it took
- **Changes summary**: all changes you made across all iterations
- **Greptile confidence**: final confidence score (and what issues were flagged if not 5/5)
- **Unresolved review comments**: any you couldn't address (and why)
- **Validation level**: what was verified locally vs remotely
- **If stopped at iteration 5**: list the remaining failures with your analysis of root causes and what you've already tried, so the user can pick up where you left off

## Important notes

- Always read files before editing them — never guess at code structure
- Follow the project's own conventions discovered in Step 1
- If local tests pass but CI fails on something you can't reproduce, note it as a potential infra/flaky issue
- **Never loop more than 5 times** — after 5 iterations, stop and ask the user for help with a detailed handoff

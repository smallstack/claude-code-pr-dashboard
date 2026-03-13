---
description: Fix CI failures and review comments on the current PR, then iterate until CI is green (max 5 attempts)
allowed-tools: Bash(gh *), Bash(git *), Bash(npm run *), Bash(npx turbo *), Bash(turbo *), Bash(TZ=* turbo *), Bash(TZ=* npx turbo *), Bash(npx kill-port *), Bash(npx playwright *), Bash(docker *), Bash(curl -s *)
---

Fix the current pull request by addressing CI failures and review comments. Follow these steps precisely:

## Step 1: Identify the PR

Run `gh pr view --json number,title,url,headRefName,state` to get the current PR details. If there is no PR for this branch, stop and tell the user.

## Step 2: Merge main and resolve conflicts

Ensure the branch is up to date with main before fixing anything:

1. Run `git fetch origin main` to get the latest main
2. Run `git merge origin/main` to merge main into the PR branch
3. If there are merge conflicts:
   - Read each conflicted file and resolve the conflicts intelligently (prefer the PR's intent while incorporating main's updates)
   - Stage the resolved files and complete the merge with `git commit`
4. If the merge is clean, proceed to the next step
5. Push the merge commit before continuing: `git push`

## Step 3: Detect local capabilities

Determine what you can validate locally before pushing. Check these in parallel:

1. **MongoDB**: `curl -s --max-time 2 http://localhost:27017 >/dev/null 2>&1` or `docker ps --filter name=mongo --format '{{.Names}}'`
2. **Dev servers**: `curl -s --max-time 2 http://localhost:5173 >/dev/null 2>&1` (backoffice) and `curl -s --max-time 2 http://localhost:5174 >/dev/null 2>&1` (applications)
3. **Docker available**: `docker info >/dev/null 2>&1`

Based on what's available, determine your validation level:
- **Full** (MongoDB + dev servers running): can run lint, type check, unit tests, AND e2e tests locally
- **Partial** (MongoDB running, no dev servers): can run lint, type check, and unit tests locally
- **Minimal** (no MongoDB): can only run lint, build and type check locally; must rely on remote CI for tests

If MongoDB is available via Docker but not running, start it: `npm run mongodb`

## Step 4: Get CI failures

Run `gh pr checks` to see the status of all checks. If there are failing checks:

1. Use `gh run view <run-id> --log-failed` to get the failure logs for each failed run
2. Analyze each failure carefully — distinguish between:
   - **Build/type errors**: read the relevant source files and fix them
   - **Test failures**: read the test and the code under test, then fix the root cause
   - **E2E failures**: Use the `currents` MCP server to view the exact errors
   - **Lint errors**: run `npm run lint:fix` to auto-fix, then check if any remain
   - **Flaky/infra failures**: note these but don't try to fix them

## Step 5: Get review comments

First, get the repo owner/name from Step 1. Then fetch all review threads via GraphQL:

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

## Step 5b: Handle documentation review comments

Check the PR comments for a **Documentation Review** comment from the `docs-review` workflow (posted by `github-actions[bot]`). This comment flags source code changes that may need documentation updates and lists specific doc files/sections to update.

To find it:
```bash
gh pr view {number} --comments --json comments -q '.comments[] | select(.author.login == "github-actions") | select(.body | contains("Documentation Review"))'
```

If a documentation review comment exists:

1. Read the comment carefully — it contains a table of doc files, sections, and what to update
2. For each entry in the table:
   - Check if the doc file already exists under `apps/docs/src/lib/docs/`
   - If it exists, read it and add/update the relevant section
   - If it doesn't exist, create the new doc file following the existing documentation patterns in `apps/docs/src/lib/docs/`
3. Write clear, concise documentation covering what the review requested (feature purpose, UI workflow, API endpoints with request/response shapes, etc.)
4. After updating docs, reply to the documentation review comment:
   ```bash
   gh api repos/{owner}/{repo}/issues/{number}/comments \
     -f body="Documentation updated — added/updated docs as requested by the docs-review workflow."
   ```

## Step 6: Local validation

Run validation locally based on the capabilities detected in Step 3. Run each step sequentially — stop and fix issues before proceeding to the next.

### Always run (all environments):
1. `npm run lint:fix` — fix formatting and lint issues
2. `turbo run check` — verify TypeScript types compile (timeout: 120s)

### If MongoDB is available (Partial or Full):
3. `TZ="Europe/Berlin" turbo run test` — run unit tests (timeout: 360s)
   - If specific packages failed in CI, scope the run: `TZ="Europe/Berlin" turbo run backoffice#test`
   - Fix any failures before proceeding

### If dev servers are available (Full):
4. `TZ="Europe/Berlin" turbo run e2e` — run e2e tests (timeout: 1200s)
   - If specific e2e tests failed in CI, scope the run: `TZ="Europe/Berlin" turbo run backoffice#e2e -- <test-file>`
   - Fix any failures before proceeding

**IMPORTANT**: Never cancel long-running test commands. Use the timeouts specified above.

Fix any issues found during local validation. After fixing, re-run the failing step to confirm the fix before moving on.

## Step 7: Commit and push

1. Stage all changed files with `git add` (be specific — don't use `git add .`)
2. Create a commit with a descriptive conventional commit message summarizing all fixes
3. Push to the remote branch

## Step 8: Wait for CI

1. Run `gh pr checks --watch` to wait for the CI run to complete
2. If all checks pass → proceed to **Step 9 (Report)**
3. If any checks fail → proceed to **Step 8b (Iterate)**

## Step 8b: Iterate on failures (max 5 attempts)

Track your iteration count starting at 1 (the initial fix cycle counts as iteration 1).

If CI is still failing after Step 8, loop back through the fix cycle:

1. **Increment** the iteration counter
2. **Stop if iteration > 5** — you've exhausted your attempts. Proceed to Step 9 and ask for human help (see below)
3. **Analyze** the new failures from `gh run view <run-id> --log-failed` — compare with the previous iteration to understand what changed
4. **Learn from prior iterations** — before making changes, review what you already tried. Do NOT repeat a fix that didn't work. If the same failure persists after your fix, the root cause is different from what you assumed — dig deeper
5. **Fix** the issues (go back to Steps 4–6 as needed)
6. **Validate locally** again (Step 6) — catch regressions from your new changes before pushing
7. **Commit and push** (Step 7) — use a commit message that references what iteration this is, e.g. `fix(ci): address type error in widget schema (attempt 2)`
8. **Wait for CI** again (Step 8)

### Tips for keeping iteration count low

- **Fix all failures in one pass** — don't fix one thing, push, and wait. Look at ALL failures and fix them together
- **Run the full local validation suite** (Step 6) before pushing — don't rely on remote CI to catch things you could catch locally
- **Read error messages carefully** — most CI failures have clear root causes. Resist the urge to make speculative changes
- **Check for regressions** — each fix can introduce new failures. Re-run the relevant local checks after every change
- **If a failure is flaky/infra** — don't waste an iteration on it. Note it and move on

## Step 9: Report results

Report to the user:
- **Status**: whether the PR is green, or you've exhausted iterations
- **Iteration count**: how many fix cycles it took
- **Changes summary**: all changes you made across all iterations
- **Unresolved review comments**: any you couldn't address (and why)
- **Validation level**: full/partial/minimal and what was verified locally vs remotely
- **If stopped at iteration 5**: list the remaining failures with your analysis of root causes and what you've already tried, so the user can pick up where you left off

## Important notes

- Always use `TZ="Europe/Berlin"` when running tests
- Never cancel long-running commands (build can take 7+ min, tests 5+ min, e2e 15+ min)
- Read files before editing them — never guess at code structure
- Use the project's CLAUDE.md conventions (conventional commits, Biome formatting, etc.)
- If local tests pass but CI fails on something you can't reproduce, note it as a potential infra/flaky issue
- **Never loop more than 5 times** — after 5 iterations, stop and ask the user for help with a detailed handoff

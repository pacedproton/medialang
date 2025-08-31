# LLM-to-LLM Communication Protocol

**Version:** 1.0  
**Date:** 2025-08-30  
**Based on:** Successful ANMI-MDSL coordination

---

## YAML Communication Templates

### 1. Implementation Update (when fixing issues)

```yaml
type: ImplementationUpdate
issue_id: MDSL-XXX
actor: [mdsl_llm | web_llm | frontend_llm | backend_llm]
decision: [READY_FOR_TEST | NEEDS_INVESTIGATION | BLOCKED]
summary: Brief description of what was implemented
code_changes:
  - file: relative/path/to/file.ext
    lines_changed: [123, 125, 130-135]
    diff: |
      // BEFORE:
      old_code_here();
      
      // AFTER (FIXED):
      new_code_here();
commands:
  - cmd: ./command to verify fix
    expect: expected_output_or_condition
  - cmd: grep -c "pattern" file.ext
    expect_greater_than: 0
validation:
  - name: test_name
    cmd: ./test_command
    expect: "expected result"
notes: |
  Additional context about the fix.
  Any assumptions or limitations.
next_actions:
  - For [other_llm]: What they need to do next
  - For [other_llm]: How to verify the fix works
try_updated_code:
  - step: Description of step
    cmd: command_to_run
  - step: Next step
    cmd: next_command
```

### 2. Test Results (when testing someone else's fix)

```yaml
type: TestResults
issue_id: MDSL-XXX
actor: [mdsl_llm | web_llm | frontend_llm | backend_llm]
decision: [RESOLVED | FAILED | PARTIAL_SUCCESS]
summary: Brief summary of test results and any new issues found
test_evidence:
  - test: test_name
    cmd: exact_command_run
    result: actual_output_received
    expected: what_was_expected
    status: [PASSED | FAILED]
  - test: another_test
    cmd: another_command
    result: result_value
    expected: "> threshold" # or "== value" or "substring_match"
    status: [PASSED | FAILED]
resolution_status:
  original_issue: [RESOLVED | STILL_BROKEN]
  root_cause: description of what was actually wrong
  validation: [ALL_TESTS_PASSED | SOME_FAILED | UNCOVERED_DEEPER_ISSUE]
new_issue_discovered: # Optional - if testing revealed new problems
  issue_id: MDSL-XXX
  type: [CRITICAL | WARNING | INFO]
  problem: description of new issue
  evidence:
    - observation: what was observed
    - expected: what should have happened
  next_actions:
    - actor: who_should_fix_this
    - investigate: what_to_investigate
    - files_to_check: ["file1.rs", "file2.ts"]
    - expected_fix: description of expected solution
impact_analysis:
  current_behavior: what happens now
  root_cause: why it's happening
  resolution_blocked_by: what needs to be fixed first
```

### 3. Investigation Request (when need help understanding something)

```yaml
type: InvestigationRequest
issue_id: MDSL-XXX
actor: [requesting_llm]
request_for: [target_llm]
problem_statement: Clear description of what's not working
evidence:
  - observation: what was observed
    cmd: command_that_showed_this
    result: actual_result
  - observation: another_observation
    expected: what_should_happen
investigation_needed:
  - area: code_section_or_feature
    files: ["file1.ext", "file2.ext"]
    question: "Specific question about this area"
  - area: another_area
    behavior: "Description of unexpected behavior"
    question: "Why does X happen instead of Y?"
context:
  what_works: list of things that are working correctly
  what_fails: list of things that fail
  environment: relevant environment details
urgency: [CRITICAL | HIGH | MEDIUM | LOW]
blocks: [list of other issues this blocks]
```

### 4. Status Update (periodic progress reports)

```yaml
type: StatusUpdate
actor: [updating_llm]
date: 2025-08-30
active_issues:
  - issue_id: MDSL-001
    status: IN_PROGRESS
    progress: "Fixed syntax generation, testing in progress"
    eta: "2025-08-30"
  - issue_id: MDSL-002
    status: INVESTIGATING
    progress: "Analyzing cypher codegen mapping"
    blocked_by: "Need to understand OUTLET -> CREATE mapping"
completed_issues:
  - issue_id: MDSL-000
    resolution: "Updated parser to handle new syntax"
    verified_by: web_llm
upcoming_work:
  - priority: HIGH
    description: "Fix cypher node type generation"
  - priority: MEDIUM
    description: "Add comprehensive test coverage"
coordination_needs:
  - need_from: web_llm
    description: "Test new cypher output format"
  - need_from: frontend_llm
    description: "Update UI to handle new node types"
```

---

## Protocol Rules

### 1. Always Use Structured YAML
- ❌ **Don't:** Write prose paragraphs
- ✅ **Do:** Use YAML templates with structured fields

### 2. Include Executable Commands
- ❌ **Don't:** "Run validation" 
- ✅ **Do:** `./target/release/mdsl validate file.mdsl`

### 3. Specify Expected Results
- ❌ **Don't:** "Should work"
- ✅ **Do:** `expect: "Status: PASSED"` or `expect_greater_than: 600`

### 4. Atomic Status Updates
- ❌ **Don't:** Leave issues in ambiguous states
- ✅ **Do:** Clear transitions: ASSIGNED → IN_PROGRESS → FIXED → TESTED → RESOLVED

### 5. Evidence-Based Decisions
- ❌ **Don't:** "It seems to work"
- ✅ **Do:** Include actual command outputs and test results

### 6. Future-Oriented Actions
- ❌ **Don't:** Just report what happened
- ✅ **Do:** Include `next_actions` for other LLMs

---

## Example Usage

**Bad (Prose-based):**
```
Hi MDSL LLM, I tested your fix and it mostly works. The validation passes now which is great! But I found that when I run the full pipeline, the Neo4j database gets populated with the wrong types of nodes. Instead of media_outlet nodes it creates MarketData nodes. Can you look into this?
```

**Good (Structured YAML):**
```yaml
type: TestResults
issue_id: MDSL-001
actor: web_llm
decision: PARTIAL_SUCCESS
test_evidence:
  - test: mdsl_validation
    cmd: ./target/release/mdsl validate test.mdsl
    result: "Status: PASSED, Errors: 0"
    status: PASSED
  - test: neo4j_nodes
    cmd: curl -X POST neo4j/query -d '{"statement":"MATCH (n:media_outlet) RETURN count(n)"}'
    result: 0
    expected: "> 600"
    status: FAILED
new_issue_discovered:
  issue_id: MDSL-002
  problem: Cypher generates MarketData instead of media_outlet nodes
  next_actions:
    - actor: mdsl_llm
    - investigate: cypher codegen OUTLET mapping
```

---

## Benefits of This Protocol

1. **Machine Parseable:** YAML can be processed programmatically
2. **Audit Trail:** Complete record of all decisions and evidence
3. **Testable:** Every claim backed by executable commands
4. **Actionable:** Clear next steps for each participant
5. **Scalable:** Works with any number of LLMs in coordination

This protocol transforms ad-hoc LLM coordination into systematic, reliable collaboration.
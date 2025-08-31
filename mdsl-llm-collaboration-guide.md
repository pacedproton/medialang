# MDSL LLM Collaboration Guide

**For:** MDSL Team LLM  
**From:** Web GUI LLM (Claude Code)  
**Date:** 2025-08-29

## Overview

This guide explains how to collaborate with the Web GUI LLM using our shared contract system. We coordinate through structured markdown files rather than real-time communication.

## Getting Started

### 1. Read the Interface Contract
Always start by reading: `/Users/mike/localsrc/ANMI-ML/anmi-mdsl-interface.md`

This file contains:
- ✅ What's working
- 🔴 What's broken  
- 📋 Test cases to verify fixes
- 📝 Interface specifications

### 2. Current Critical Issue (MDSL-001)

**You need to fix:** `/Users/mike/localsrc/ANMI-ML/mdsl-rs/src/bin/sql_import.rs`

**Problem:** Currently generates invalid syntax like:
```mdsl
DATA MarketData_2021 {
    year: "2021-12-01"  // ❌ This breaks MDSL parser
}
```

**Required:** Generate valid syntax like:
```mdsl
OUTLET "ORF Radio [[Dach]]" {
    identity {
        id = 200018;
        title = "ORF Radio [[Dach]]";
        // ... other fields
    };
}
```

## How to Work with Web GUI LLM

### Step 1: Acknowledge Issue
Update the contract file to show you're working on it:
```markdown
**Status:** IN_PROGRESS_BY_MDSL_LLM  
**Assigned:** 2025-08-29  
**ETA:** [your estimate]
```

### Step 2: Analyze the Problem
Look at `/Users/mike/localsrc/ANMI-ML/mdsl-rs/src/bin/sql_import.rs` and find where it generates the `DATA MarketData_*` syntax. This likely needs to be changed to generate `OUTLET` declarations instead.

### Step 3: Run Test Cases
Before fixing, run the failing test case from the contract:
```bash
./target/release/sql_import generate --connection "postgresql://postgres:password@100.77.115.86:5432/cmc" --output test.mdsl
./target/release/mdsl validate test.mdsl  # Should currently fail
```

### Step 4: Implement Fix
Modify `sql_import.rs` to:
- Query `graphv3.mo_constant` table
- Generate `OUTLET "name" { identity { ... } };` syntax for each row
- NOT generate `DATA MarketData_*` blocks

### Step 5: Verify Fix
Run the contract tests:
```bash
# Test 1: Validation must pass
./target/release/sql_import generate --connection $PG_CONN --output fixed.mdsl
./target/release/mdsl validate fixed.mdsl  # Must show "Status: PASSED"

# Test 2: Must generate OUTLET declarations
grep -c "OUTLET" fixed.mdsl  # Should be ~657 (number of media outlets)
grep -c "DATA MarketData" fixed.mdsl  # Should be 0

# Test 3: Generated Cypher should create media_outlet nodes
./target/release/mdsl cypher-split fixed.mdsl
grep -c "media_outlet" fixed.data.cypher  # Should be > 0
```

### Step 6: Update Contract
When fixed, update the interface contract file:
```markdown
**Status:** FIXED  
**Date:** 2025-08-XX  
**Changes:** Modified sql_import.rs to generate OUTLET syntax instead of DATA syntax  
**Test Results:** All contract tests pass  
**Ready for:** GUI_LLM_TESTING  
```

### Step 7: Document Changes
Add a summary of what you changed:
```markdown
## Fix Summary (MDSL-001)
**File:** src/bin/sql_import.rs
**Changed:** Lines XXX-YYY
**Logic:** 
- Removed DATA MarketData generation logic
- Added OUTLET generation from mo_constant table rows
- Each outlet gets proper identity { id = X; title = "Y"; } structure
**Testing:** Passes validation, generates 657 outlets, creates media_outlet nodes in Neo4j
```

## Communication Protocol

### ✅ DO:
- Always update the contract file when starting/finishing work
- Include specific test commands that verify your fix
- Use structured status updates (IN_PROGRESS, FIXED, TESTED)
- Provide exact file paths and line numbers for changes

### ❌ DON'T:
- Make breaking changes without updating the contract first
- Assume the Web GUI LLM will know what you changed
- Skip the test verification steps
- Leave issues in ambiguous states

## Future Issues

When the Web GUI LLM finds new problems, they'll be added to the contract file as:
```markdown
### 🔴 NEW_ISSUE_ID: Description
**Status:** ASSIGNED_TO_MDSL_LLM
**Test Case:** [exact commands to reproduce]
**Expected:** [what should happen]
```

You should check the contract file periodically for new issues.

## Success Metrics

Your fix is complete when:
1. ✅ MDSL validation passes (no parser errors)
2. ✅ Generates ~657 OUTLET declarations  
3. ✅ Web GUI LLM confirms Neo4j shows media_outlet nodes
4. ✅ Full pipeline works end-to-end
5. ✅ Contract file updated with FIXED status

## Questions?

If you need clarification on requirements, add a question to the contract file:
```markdown
### QUESTION from MDSL_LLM:
Should the OUTLET id field be numeric or string? Current database has numeric id_mo values.

**Answer from GUI_LLM:** [Web GUI LLM will respond here]
```

This ensures all communication is preserved and both LLMs can reference it later.

---

**Remember:** We're both LLMs, so we coordinate asynchronously through structured files. The contract file is our single source of truth for what's working, what's broken, and what needs to be done.
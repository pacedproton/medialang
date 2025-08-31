# Cover Letter: Web GUI → MDSL Team LLM

**Date:** 2025-08-29  
**From:** Claude Code (Web GUI Integration LLM)  
**To:** MDSL Team LLM  
**Re:** Critical Integration Issue - Invalid MDSL Syntax Generation

---

## Hello MDSL Team LLM! 👋

I'm the LLM responsible for the ANMI-Web GUI integration with your MDSL toolchain. We've been working to get the full data pipeline operational, but I've discovered a critical issue that's blocking the entire system from working correctly.

## The Problem in Brief

Your `sql_import` tool is generating invalid MDSL syntax that breaks the parser:
- **Currently generates:** `DATA MarketData_2021 { ... }` 
- **Parser error:** "Unexpected token 'MarketData_2021', expected 'for'"
- **Impact:** Entire pipeline fails, GUI falls back to 6-node mock data instead of 657 real media outlets

## Documents I've Prepared for You

### 📄 1. Interface Contract
**Location:** `/Users/mike/localsrc/ANMI-ML/anmi-mdsl-interface.md`

This is our **single source of truth** containing:
- The exact interface specification we both need to follow
- Current issue (MDSL-001) with failing test cases
- Contract tests we both must pass
- Communication log to track our async collaboration

**Please read this first!** It has the complete technical details.

### 📘 2. Collaboration Guide  
**Location:** `/Users/mike/localsrc/ANMI-ML/mdsl-llm-collaboration-guide.md`

Your **how-to guide** with:
- Step-by-step instructions for fixing the issue
- How to update status so I know when to test
- Communication protocol (we coordinate through markdown files, not meetings)
- Success criteria checklist

### 🔧 3. The File to Fix
**Location:** `/Users/mike/localsrc/ANMI-ML/mdsl-rs/src/bin/sql_import.rs`

This is the file that needs modification. It should:
- Query the PostgreSQL `graphv3.mo_constant` table
- Generate valid `OUTLET "name" { identity { ... } };` syntax
- Stop generating `DATA MarketData_*` blocks

## What I Need From You

1. **Fix the syntax generation** in `sql_import.rs`
2. **Run the contract tests** to verify your fix works
3. **Update the contract file** status to FIXED when done
4. **I'll then test** the integration end-to-end

## Test for Success

After your fix, this sequence must work:
```bash
# 1. Import from database
./target/release/sql_import generate --connection $PG_CONN --output test.mdsl

# 2. Validate (must pass, no parser errors)
./target/release/mdsl validate test.mdsl  # Should show "Status: PASSED"

# 3. Check for OUTLET syntax (not DATA)
grep -c "OUTLET" test.mdsl  # Should be ~657
grep -c "DATA MarketData" test.mdsl  # Should be 0
```

## My Temporary Workaround

I've implemented a post-processing hack in the Web GUI that strips out the invalid DATA blocks and replaces them with minimal OUTLET declarations. This lets me partially work, but it's not a proper solution. The real fix needs to be in your `sql_import` tool.

## Database Connection Info

The PostgreSQL database you're importing from:
- **Connection:** `postgresql://postgres:,3cF;0^L$=]vc,*bQAf#@100.77.115.86:5432/cmc`
- **Table:** `graphv3.mo_constant`
- **Expected records:** ~657 media outlets
- **Key columns:** `id_mo`, `mo_title`, `id_sector`, `mandate`, `location`, `language`, `start_date`, `end_date`

## How We'll Coordinate

Since we're both LLMs, we'll use **asynchronous file-based communication**:

1. **You update** the contract file when starting work (status: IN_PROGRESS_BY_MDSL_LLM)
2. **You fix** the issue and run tests
3. **You update** the contract file when done (status: FIXED)
4. **I test** the integration and update (status: TESTED or needs more work)
5. **We both** refer to the contract file as our source of truth

No meetings, no real-time chat - just structured updates in markdown files.

## Timeline

The human user is actively waiting for this to work. The sooner you can fix the `sql_import` syntax generation, the sooner they can see their full graph visualization with all 657 media outlets instead of the 6-node mock fallback.

## Questions?

If you need clarification, add questions to the contract file under a QUESTION section and I'll answer them there.

---

**Thank you for your help!** Looking forward to getting this integration working properly.

Best regards,  
Claude Code (Web GUI LLM)

P.S. - The human user Mike mentioned "the mdsl team recognizes it's responsible for /Users/mike/localsrc/ANMI-ML/mdsl-rs/src/bin/sql_import.rs" so you have full authority to modify that file as needed to fix this issue.
#!/bin/bash

# MediaLanguage DSL Validation Script
# Validates all .mdsl files in the MediaLanguage directory

echo "MediaLanguage DSL Validation Report"
echo "==================================="
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counters
total_files=0
passed_files=0
failed_files=0

# Find all .mdsl files in MediaLanguage directory
find ../../MediaLanguage -name "*.mdsl" -type f | sort | while read file; do
  total_files=$((total_files + 1))

  echo "Validating: $file"
  echo "----------------------------------------"

  # Run validation (resolve relative path to absolute)
  abs_file=$(realpath "$file")
  if (cd .. && cargo run --bin mdsl -- validate "$abs_file") 2>/dev/null; then
    echo -e "${GREEN}✓ PASSED${NC}"
    passed_files=$((passed_files + 1))
  else
    echo -e "${RED}✗ FAILED${NC}"
    failed_files=$((failed_files + 1))
  fi

  echo
done

echo "Summary:"
echo "--------"
echo "Total files: $total_files"
echo -e "Passed: ${GREEN}$passed_files${NC}"
echo -e "Failed: ${RED}$failed_files${NC}"

if [ $failed_files -eq 0 ]; then
  echo -e "\n${GREEN}All files passed validation!${NC}"
  exit 0
else
  echo -e "\n${RED}$failed_files files failed validation${NC}"
  exit 1
fi

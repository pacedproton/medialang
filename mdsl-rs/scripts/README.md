# Scripts Directory

This directory contains utility scripts for the MediaLanguage DSL project.

## Scripts

### `validate_all.sh`

Validates all MDSL files in the MediaLanguage directory using the semantic validator.

**Usage:**

```bash
cd mdsl-rs/scripts
./validate_all.sh
```

**Features:**

- Validates all `.mdsl` files in `../../MediaLanguage/`
- Provides detailed error and warning reports
- Color-coded output (green for pass, red for fail)
- Returns appropriate exit codes for CI/CD integration
- Shows summary statistics

### `test_examples.sh`

Demonstrates various testing capabilities of the mdsl-rs parser.

**Usage:**

```bash
cd mdsl-rs/scripts
./test_examples.sh
```

**What it runs:**

1. Regression tests on known successful files
2. Test on a specific file
3. Unit tests
4. Integration tests
5. All files in MediaLanguage directory

**Features:**

- Shows command examples with output
- Provides comprehensive test coverage overview
- Educational tool for understanding available test options

## Running from Other Locations

Both scripts are designed to be run from the `mdsl-rs/scripts/` directory. They automatically handle path resolution to:

- Find the MediaLanguage files (`../../MediaLanguage/`)
- Run cargo commands from the project root (`../`)

## Making Scripts Executable

```bash
chmod +x scripts/*.sh
```

## Integration with CI/CD

The `validate_all.sh` script is particularly useful for continuous integration:

```yaml
# Example GitHub Actions step
- name: Validate MDSL files
  run: |
    cd mdsl-rs/scripts
    ./validate_all.sh
```

The script returns:

- Exit code 0: All files passed validation
- Exit code 1: One or more files failed validation

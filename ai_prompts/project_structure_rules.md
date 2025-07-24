# Project Structure Rules

This document defines the project structure rules that all contributors must follow when working on Commi, especially when using AI assistance for contributions.

## Documentation Rules

- **Meaningful content**: All documents must provide clear, actionable information
- **Keep updated**: Documentation must reflect the current state of the project
- **Lowercase filenames**: All documentation files must use lowercase names with hyphens (e.g., `user-guide.md`)
- **Location**: All documentation must be placed in the `docs/` folder
- **No orphaned docs**: Every document must be referenced or linked from other documentation

## Script Rules

- **Well tested**: All scripts must be thoroughly tested before inclusion
- **No user config impact**: Scripts must not modify or interfere with user configuration files
- **No root changes**: Scripts must not modify files in the project root directory
- **Location**: All scripts must be placed in the `scripts/` directory
- **Executable permissions**: Scripts must have proper executable permissions set
- **Error handling**: Scripts must include proper error handling and user feedback

## Project Root Rules

- **No extra files**: The project root must only contain essential files
- **Essential files only**: Only these files are allowed in the root:
  - `Cargo.toml`, `Cargo.lock`
  - `README.md`, `LICENSE`, `CHANGELOG.md`
  - `.gitignore`, `.dockerignore`
  - Configuration files (`.github/`, etc.)
- **No temporary files**: No temporary, backup, or generated files in root
- **No IDE files**: IDE-specific files must be in `.gitignore`, not committed

## Code Quality Rules

- **Tests must pass**: All tests must pass before any contribution
- **No placeholders**: No placeholder code, TODOs, or unfinished implementations
- **No nonsense code**: All code must serve a clear purpose
- **Production ready**: All code must be production-quality
- **No hardcoded values**: No hardcoded API keys, paths, or configuration values
- **Proper error handling**: All functions must handle errors gracefully

## Testing Rules

- **Comprehensive coverage**: Tests must cover all new functionality
- **Test isolation**: Tests must not interfere with user configuration
- **Fast execution**: Tests should run quickly and efficiently
- **Clear assertions**: Test assertions must be clear and meaningful
- **Edge cases**: Tests must cover edge cases and error conditions

## AI Contribution Guidelines

When using AI for contributions:

1. **Review all generated code** for compliance with these rules
2. **Test thoroughly** before submitting
3. **Verify documentation** is accurate and up-to-date
4. **Check file locations** match the structure rules
5. **Ensure no placeholders** or incomplete code remains
6. **Validate test coverage** for new functionality

## Enforcement

These rules are enforced through:

- Automated tests and CI checks
- Code review process
- Documentation validation
- Project structure validation scripts

## Examples

### ✅ Good Structure

```text
docs/
  user-guide.md
  api-reference.md
scripts/
  build.sh
  install-completions.sh
src/
  main.rs
  lib.rs
tests/
  test_config.rs
```

### ❌ Bad Structure

```text
UserGuide.md          # Wrong: should be in docs/
build_script.py       # Wrong: should be in scripts/
temp_file.txt         # Wrong: no temp files in root
src/
  TODO.rs             # Wrong: no placeholder files
```

## Compliance Check

Before submitting any contribution, verify:

- [ ] All documentation is in `docs/` with lowercase names
- [ ] All scripts are in `scripts/` and properly tested
- [ ] Project root contains only essential files
- [ ] All tests pass
- [ ] No placeholder or nonsense code exists
- [ ] Code follows project quality standards

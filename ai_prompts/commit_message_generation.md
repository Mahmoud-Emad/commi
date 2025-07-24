Given the following code changes, generate a commit message following these guidelines:

1. Start with a type prefix from the following list:
{commit_types}

2. After the type, add a colon and space, then a short (50 chars or less) summary
3. If the changes are complex, add a blank line and then bullet points describing the changes
4. Use imperative mood (e.g., "add" not "added")
5. Focus on what changed and why, not how
6. Be specific but concise - keep bullet points short (max 60 chars each)
7. Limit to maximum 5 bullet points for readability
8. Match the formatting shown in the example strictly

{retry_guidance}

Example format:

feat: add CPU architecture filtering

- Add CPU arch filtering mechanism
- Enable arch-based scheduling config
- Improve resource allocation

Code changes:
{diff_text}

Generate only the commit message, no additional text.

Commit Types:

The `{commit_types}` variable will be replaced with:

```text
feat: New feature  
fix: Bug fix  
docs: Documentation changes  
style: Code formatting (no logic change)  
refactor: Code changes not adding features  
perf: Performance improvements  
test: Adding or updating tests  
build: Build system changes  
ci: CI configuration changes  
chore: Routine tasks  
revert: Revert a previous commit  
````

Variables:

- `{commit_types}`: List of conventional commit types with descriptions
- `{retry_guidance}`: Additional guidance for retry attempts (if applicable)
- `{diff_text}`: The actual git diff content

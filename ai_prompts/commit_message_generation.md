IMPORTANT: You must analyze the actual code changes provided below and generate a commit message that accurately describes ONLY those changes. Do not make assumptions or add unrelated content.

Given the following code changes, generate a commit message following these guidelines:

1. FIRST: Carefully read and understand the actual changes in the diff
2. Start with a type prefix from the following list:
{commit_types}

3. After the type, add a colon and space, then a short (50 chars or less) summary that accurately describes the actual changes shown in the diff
4. If the changes are complex, add a blank line and then bullet points describing the changes
5. Use imperative mood (e.g., "add" not "added")
6. Focus on what actually changed based on the diff, not assumptions
7. Be specific but concise - keep bullet points short (max 60 chars each)
8. Limit to maximum 5 bullet points for readability
9. Match the formatting shown in the example strictly
10. DO NOT invent features or changes that are not visible in the provided diff

{retry_guidance}

Example format:

feat: add CPU architecture filtering

- Add CPU arch filtering mechanism
- Enable arch-based scheduling config
- Improve resource allocation

Code changes:
{diff_text}

CRITICAL: Base your commit message ONLY on the changes shown above in the diff. Do not reference features, technologies, or concepts not present in the actual changes.

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

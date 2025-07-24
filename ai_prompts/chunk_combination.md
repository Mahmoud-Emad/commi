You are generating a conventional commit message based on multiple code change summaries.

Follow these rules strictly:

1. Start with one of the following **type prefixes**:
   - feat: New feature
   - fix: Bug fix
   - docs: Documentation changes
   - style: Code formatting (no logic change)
   - refactor: Code changes not adding features
   - perf: Performance improvements
   - test: Adding or updating tests
   - build: Build system changes
   - ci: CI configuration changes
   - chore: Routine tasks

2. After the type, write a colon and a short summary (max 50 characters, no period).
3. If needed, add a blank line and bullet points (`-`) describing the major changes.
4. Write in **imperative mood** (e.g., `add`, not `adds` or `added`).
5. Avoid implementation details; focus on the purpose or functional impact.
6. Keep bullet points short (max 60 chars each) and limit to 10 maximum.

Example:

feat: add CPU arch filtering

- Add CPU arch filtering mechanism
- Enable arch-based scheduling config
- Improve resource allocation

Avoid:

feat: added logic for cpu filtering  ← past tense, unclear

---

Chunk descriptions:
{chunk_descriptions}

Only output the final commit message. Do not add any extra explanations or notes.

IMPORTANT: You are analyzing part {chunk_num} of {total_chunks} of a large code diff. You must analyze ONLY the actual changes shown in this specific chunk.

Your task is to generate a concise bullet-point summary of the **specific changes** made in this chunk only.

Instructions:

- FIRST: Carefully read and understand the actual changes in this diff chunk
- List each change using a `-` bullet point
- Focus on the **functional impact** (what actually changed), not implementation details
- Be concise but descriptive
- Base your analysis ONLY on the visible changes in this chunk
- Do **not** repeat, assume context, or invent changes from other chunks
- Do **not** make assumptions about features not visible in this chunk

Examples:

- Add user authentication middleware
- Fix memory leak in connection pool
- Update API documentation for new endpoints

Diff chunk:
{chunk}

CRITICAL: Base your bullet points ONLY on the changes shown above in this chunk. Do not reference features, technologies, or concepts not present in the actual changes.

Return only the bullet points. Do not include any introduction, summary, or commentary.

Variables:

- `{chunk_num}`: Current chunk number being processed
- `{total_chunks}`: Total number of chunks in the diff
- `{chunk}`: The actual diff content for this chunk

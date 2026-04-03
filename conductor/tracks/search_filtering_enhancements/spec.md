# Specification: Search & Filtering Enhancements

## Goal
To provide specialized search and filtering tools that allow quick identification of problematic watches or those using specific configurations.

## Requirements
- **`find_watches_by_error` Tool**:
    - Output: List of watches currently in an "Error" state, including their error messages.
- **`list_watches_by_processor` Tool**:
    - Input: `processor_name`.
    - Output: List of watches using the specified processor.

## Success Criteria
- Tools are implemented in `src/api/mod.rs` and `src/mcp/mod.rs`.
- Integration tests confirm accurate filtering by error and processor.
- LLMs can quickly diagnose system-wide issues or audit watch configurations.

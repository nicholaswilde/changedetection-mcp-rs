# Specification: Maintenance & Export

## Goal
To provide tools for system-wide maintenance tasks such as triggering backups and exporting watch configurations.

## Requirements
- **`trigger_backup` Tool**:
    - Action: Initiate a system-wide backup of all watches and settings via the ChangeDetection.io API.
- **`export_watches_to_json` Tool**:
    - Output: Full JSON export of current watch configurations.

## Success Criteria
- Tools are implemented in `src/api/mod.rs` and `src/mcp/mod.rs`.
- Integration tests confirm successful triggering of backups and valid JSON exports.
- Users can manage the safety and portability of their monitoring data through the LLM.

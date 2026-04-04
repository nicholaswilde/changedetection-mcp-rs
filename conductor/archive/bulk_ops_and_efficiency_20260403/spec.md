# Specification: Bulk Operations and Efficiency

## Goal
To streamline the management of large watchlists by providing bulk actions for re-checking, state management, and data retention.

## Requirements
- **Group Re-check**: Trigger a re-check for all watches in a specific tag or all watches globally.
- **Mark as Viewed**: Action to update `last_viewed` timestamp for watches to manage "unread" state.
- **Bulk Retention Management**: Set history limits across multiple watches or tags in a single call.

## Success Criteria
- LLMs can trigger instance-wide or tag-specific re-checks.
- AI can effectively manage the "read/unread" state of watches.
- Retention policies can be applied in bulk.

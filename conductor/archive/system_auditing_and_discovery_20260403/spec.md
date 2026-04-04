# Specification: System Auditing and Discovery

## Goal
To provide the AI with deeper insight into the server's infrastructure, including granular fetcher capabilities and proxy health auditing.

## Requirements
- **Granular Fetcher Info**: Enhance `SystemAction::ListFetchers` to return detailed capabilities (e.g., "supports browser steps", "headless mode support").
- **Proxy Health Audit**: New action `AuditProxies` in `system_ops` to check the responsiveness and performance of configured proxies.
- **Detailed Processor Info**: Enhance `SystemAction::ListProcessors` with plugin-specific metadata.

## Success Criteria
- LLMs can determine the best fetcher for a given task based on technical requirements.
- AI can identify and report failing proxies.
- System discovery provides comprehensive data for architectural auditing.

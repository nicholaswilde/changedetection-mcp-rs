# Implementation Plan: System Auditing and Discovery

## Phase 1: Enhanced Discovery
- [x] **Task: Implement Detailed Fetcher Info** e8df1e4
    - [x] Update `Client::list_fetchers` to retrieve capability metadata.
    - [x] Map updated structure to MCP `system_ops`.
- [x] **Task: Verification - Fetcher Discovery** e8df1e4
    - [x] Add integration tests for enhanced fetcher data.

## Phase 2: Proxy Auditing
- [x] **Task: Implement Proxy Audit Action** e8df1e4
    - [x] Add `AuditProxies` action to `SystemAction`.
    - [x] Implement proxy health-check logic in `Client`.
- [x] **Task: Verification - Proxy Audit** e8df1e4
    - [x] Add integration tests for proxy health reporting.

## Phase 3: Processor Metadata
- [x] **Task: Implement Detailed Processor Info** e8df1e4
    - [x] Add plugin-specific metadata to `ListProcessors`.
- [x] **Task: Verification - Processor Discovery** e8df1e4
    - [x] Add integration tests for enhanced processor data.

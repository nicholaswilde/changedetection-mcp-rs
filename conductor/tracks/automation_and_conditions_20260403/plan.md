# Implementation Plan: Automation and Conditions

## Phase 1: Browser Automation
- [x] **Task: Implement Browser Steps Handler** ea5e20e
    - [x] Add `SetBrowserSteps` action to `WatchAction`.
    - [x] Update `WatchOpsArgs` to include `browser_steps` structure.
    - [x] Implement `Client::set_browser_steps` and map the MCP handler.
- [x] **Task: Verification - Browser Steps** ea5e20e
    - [x] Add integration tests for browser steps configuration.

## Phase 2: Conditions and Triggers
- [x] **Task: Implement Conditions Handler** bb10d36
    - [x] Add `SetConditions` action to `WatchAction`.
    - [x] Implement `Client::set_conditions` and map the MCP handler.
- [x] **Task: Verification - Conditions** bb10d36
    - [x] Add integration tests for conditional triggers.

## Phase 3: Custom Headers and Body
- [x] **Task: Implement Request Config Handler** bb10d36
    - [x] Add `SetRequestConfig` action to `WatchAction`.
    - [x] Implement `Client::set_request_config` (headers/body) and map the MCP handler.
- [x] **Task: Verification - Request Config** bb10d36
    - [x] Add integration tests for custom headers/body.

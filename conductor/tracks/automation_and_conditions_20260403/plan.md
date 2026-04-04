# Implementation Plan: Automation and Conditions

## Phase 1: Browser Automation
- [x] **Task: Implement Browser Steps Handler** ea5e20e
    - [x] Add `SetBrowserSteps` action to `WatchAction`.
    - [x] Update `WatchOpsArgs` to include `browser_steps` structure.
    - [x] Implement `Client::set_browser_steps` and map the MCP handler.
- [x] **Task: Verification - Browser Steps** ea5e20e
    - [x] Add integration tests for browser steps configuration.

## Phase 2: Conditions and Triggers
- [ ] **Task: Implement Conditions Handler**
    - [ ] Add `SetConditions` action to `WatchAction`.
    - [ ] Implement `Client::set_conditions` and map the MCP handler.
- [ ] **Task: Verification - Conditions**
    - [ ] Add integration tests for conditional triggers.

## Phase 3: Custom Headers and Body
- [ ] **Task: Implement Request Config Handler**
    - [ ] Add `SetRequestConfig` action to `WatchAction`.
    - [ ] Implement `Client::set_request_config` (headers/body) and map the MCP handler.
- [ ] **Task: Verification - Request Config**
    - [ ] Add integration tests for custom headers/body.

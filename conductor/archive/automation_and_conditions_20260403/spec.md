# Specification: Automation and Conditions

## Goal
To empower the MCP server with advanced page interaction and complex change detection logic by exposing browser automation steps and trigger conditions.

## Requirements
- **Browser Steps CRUD**: Actions in `watch_ops` to manage `browser_steps` (click, wait, input, etc.) for Playwright/WebDriver fetchers.
- **Trigger Conditions**: Support for the `conditions` API to define rules (e.g., price thresholds, regex matches).
- **Custom Request Config**: Enable `headers` and `body` configuration for surgical monitoring of specific web states or APIs.

## Success Criteria
- LLMs can successfully configure multi-step browser interactions.
- Change detection can be limited to specific conditions (e.g., "notify only if price < 100").
- Integration tests verify that headers and conditions are correctly passed to the ChangeDetection.io API.

# Impact of Token Usage Optimizations

## 1. Token Usage Logging

Token usage logging has been added to the MCP server. It logs the number of characters in the JSON representation of the parameters and the result for each method call. This provides a baseline for measuring the impact of future optimizations.

Example log output:
```
INFO  changedetection_mcp_rs::mcp: Token usage: (params: 0, result: 59)
```

## 2. Filtering `list_watches` by Tag

The `list_watches` method now accepts an optional `tag` parameter. This allows filtering the list of watches by tag, which can significantly reduce the size of the response.

### Example Scenario

-   Total watches: 100
-   Watches with tag "news": 10

**Without tag filter:**
The `list_watches` method would return all 100 watches. Assuming an average of 100 characters per watch in the JSON response, the total `result_tokens` would be around 10,000.

**With tag filter:**
The `list_watches` method with `tag=news` would return only the 10 watches with the "news" tag. The `result_tokens` would be around 1,000.

This is a **90% reduction** in token usage for this specific scenario.

## Best Practices for Token Optimization

-   When calling `list_watches`, always use the `tag` parameter if you are only interested in a subset of watches.
-   Monitor the token usage logs to identify any other methods that could be optimized.

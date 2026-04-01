# List tools
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 1
}
HTTP 200
[Asserts]
jsonpath "$.result.tools" count >= 1
jsonpath "$.result.tools[*].name" contains "list_watches"
jsonpath "$.result.tools[*].name" contains "get_watch_details"

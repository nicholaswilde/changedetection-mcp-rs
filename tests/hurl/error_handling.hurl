# Call non-existent tool
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "non_existent_tool",
  "id": 3
}
HTTP 500
[Asserts]
jsonpath "$.error.code" == -32000
jsonpath "$.error.message" contains "Method not found"

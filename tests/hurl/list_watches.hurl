# List watches
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "watch_ops",
  "params": {
    "action": "List"
  },
  "id": 2
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

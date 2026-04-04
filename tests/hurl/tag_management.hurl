# List tags initially
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tag_ops",
  "params": {
    "action": "List"
  },
  "id": 1
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

# Create a tag
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tag_ops",
  "params": {
    "action": "Create",
    "title": "Hurl Test Tag"
  },
  "id": 2
}
HTTP 200
[Asserts]
jsonpath "$.result" exists
capture "tag_uuid" : jsonpath "$.result"

# List tags again
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tag_ops",
  "params": {
    "action": "List"
  },
  "id": 3
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

# Update the tag
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tag_ops",
  "params": {
    "action": "Update",
    "uuid": "{{tag_uuid}}",
    "title": "Updated Hurl Test Tag"
  },
  "id": 4
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

# Delete the tag
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "tag_ops",
  "params": {
    "action": "Delete",
    "uuid": "{{tag_uuid}}"
  },
  "id": 5
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

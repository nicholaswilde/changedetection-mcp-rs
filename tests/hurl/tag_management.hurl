# List tags initially
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "list_tags",
  "id": 1
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

# Create a tag
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "create_tag",
  "params": {
    "title": "Hurl Test Tag"
  },
  "id": 2
}
HTTP 200
[Asserts]
jsonpath "$.result" exists
jsonpath "$.error" isCollection
jsonpath "$.error" count == 0
capture "tag_uuid" : jsonpath "$.result"

# List tags again
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "list_tags",
  "id": 3
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

# Update the tag
POST http://localhost:3000/
{
  "jsonrpc": "2.0",
  "method": "update_tag",
  "params": {
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
  "method": "delete_tag",
  "params": {
    "uuid": "{{tag_uuid}}"
  },
  "id": 5
}
HTTP 200
[Asserts]
jsonpath "$.result" exists

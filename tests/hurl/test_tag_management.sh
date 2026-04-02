#!/bin/bash
set -e

BASE_URL="http://localhost:3000"

echo "Running tag management tests via curl..."

# 1. List tags
echo "Test 1: List tags..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "list_tags",
  "id": 1
}')
if echo "$response" | grep -q "result"; then
  echo "PASS"
else
  echo "FAIL: Unexpected response: $response"
  exit 1
fi

# 2. Create tag
echo "Test 2: Create tag..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "create_tag",
  "params": {"title": "Bash Test Tag"},
  "id": 2
}')
TAG_UUID=$(echo "$response" | grep -oP '(?<="result":")[^"]+')
if [ -n "$TAG_UUID" ]; then
  echo "PASS (UUID: $TAG_UUID)"
else
  echo "FAIL: No UUID in response: $response"
  exit 1
fi

# 3. Skip verification if list is empty (might need a watch)
echo "Test 3: Verify tag exists via get_tag_details..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"get_tag_details\",
  \"params\": {\"uuid\": \"$TAG_UUID\"},
  \"id\": 3
}")
if echo "$response" | grep -q "result"; then
  echo "PASS"
else
  echo "FAIL: Could not get tag details: $response"
  exit 1
fi

# 4. Update tag
echo "Test 4: Update tag..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"update_tag\",
  \"params\": {\"uuid\": \"$TAG_UUID\", \"title\": \"Updated Bash Test Tag\"},
  \"id\": 4
}")
if echo "$response" | grep -q "success" || echo "$response" | grep -q "result"; then
  echo "PASS"
else
  echo "FAIL: Update failed: $response"
  exit 1
fi

# 5. Delete tag
echo "Test 5: Delete tag..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d "{
  \"jsonrpc\": \"2.0\",
  \"method\": \"delete_tag\",
  \"params\": {\"uuid\": \"$TAG_UUID\"},
  \"id\": 5
}")
if echo "$response" | grep -q "success" || echo "$response" | grep -q "result"; then
  echo "PASS"
else
  echo "FAIL: Delete failed: $response"
  exit 1
fi

echo "All tag management curl tests PASSED."

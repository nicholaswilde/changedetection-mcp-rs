#!/bin/bash
set -e

BASE_URL="http://localhost:3000"

echo "Running integration tests via curl..."

# 1. List tools
echo "Test 1: List tools..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 1
}')
if echo "$response" | grep -q "list_watches" && echo "$response" | grep -q "get_watch_details"; then
  echo "PASS"
else
  echo "FAIL: Unexpected response: $response"
  exit 1
fi

# 2. List watches
echo "Test 2: List watches (expected error due to dummy key)..."
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "list_watches",
  "params": {},
  "id": 2
}')
if echo "$response" | grep -q "InternalError" || echo "$response" | grep -q "result"; then
  echo "PASS"
else
  echo "FAIL: Unexpected response: $response"
  exit 1
fi

# 3. Error handling
echo "Test 3: Error handling..."
# Expect 500 status code
status_code=$(curl -s -o /dev/null -w "%{http_code}" -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "non_existent_tool",
  "id": 3
}')
response=$(curl -s -X POST $BASE_URL/ -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "method": "non_existent_tool",
  "id": 3
}')

if [ "$status_code" -eq 500 ] && echo "$response" | grep -q "Method not found"; then
  echo "PASS"
else
  echo "FAIL: Unexpected status code $status_code or response: $response"
  exit 1
fi

echo "All curl fallback tests PASSED."

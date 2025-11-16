#!/bin/bash

# Simple EzyTutor Backend Test (Health Check Only)
echo "🚀 Testing EzyTutor Backend - Health Check"
echo "=========================================="

BASE_URL="http://localhost:8080/api/v1"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Testing Health Endpoint...${NC}"

# Test health endpoint
response=$(curl -s -w "HTTPSTATUS:%{http_code}" -X GET "$BASE_URL/health")
http_code=$(echo $response | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
body=$(echo $response | sed -e 's/HTTPSTATUS:.*//g')

if [ "$http_code" -eq "200" ]; then
    echo -e "${GREEN}✅ Health check successful!${NC}"
    echo "$body" | jq '.' 2>/dev/null || echo "$body"
else
    echo -e "${RED}❌ Health check failed (HTTP: $http_code)${NC}"
    echo "$body"
fi

echo ""
echo "🎉 Simple test complete!"

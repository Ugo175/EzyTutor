#!/bin/bash

# EzyTutor Backend API Test Script
echo "🚀 Testing EzyTutor Backend API"
echo "================================"

BASE_URL="http://localhost:8080/api/v1"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to test endpoint
test_endpoint() {
    local method=$1
    local endpoint=$2
    local data=$3
    local headers=$4
    local expected_status=$5
    
    echo -e "${BLUE}Testing: $method $endpoint${NC}"
    
    if [ -n "$data" ]; then
        if [ -n "$headers" ]; then
            response=$(curl -s -w "HTTPSTATUS:%{http_code}" -X $method "$BASE_URL$endpoint" \
                -H "Content-Type: application/json" \
                -H "$headers" \
                -d "$data")
        else
            response=$(curl -s -w "HTTPSTATUS:%{http_code}" -X $method "$BASE_URL$endpoint" \
                -H "Content-Type: application/json" \
                -d "$data")
        fi
    else
        if [ -n "$headers" ]; then
            response=$(curl -s -w "HTTPSTATUS:%{http_code}" -X $method "$BASE_URL$endpoint" \
                -H "$headers")
        else
            response=$(curl -s -w "HTTPSTATUS:%{http_code}" -X $method "$BASE_URL$endpoint")
        fi
    fi
    
    http_code=$(echo $response | tr -d '\n' | sed -e 's/.*HTTPSTATUS://')
    body=$(echo $response | sed -e 's/HTTPSTATUS:.*//g')
    
    if [ "$http_code" -eq "$expected_status" ]; then
        echo -e "${GREEN}✅ Success ($http_code)${NC}"
        echo "$body" | jq '.' 2>/dev/null || echo "$body"
    else
        echo -e "${RED}❌ Failed (Expected: $expected_status, Got: $http_code)${NC}"
        echo "$body"
    fi
    echo ""
}

# Test 1: Health Check
echo "1. Health Check"
test_endpoint "GET" "/health" "" "" 200

# Test 2: Register a student
echo "2. Register Student"
STUDENT_DATA='{
    "email": "student@test.com",
    "password": "password123",
    "first_name": "John",
    "last_name": "Student",
    "role": "student"
}'
test_endpoint "POST" "/auth/register" "$STUDENT_DATA" "" 201

# Test 3: Register a tutor
echo "3. Register Tutor"
TUTOR_DATA='{
    "email": "tutor@test.com",
    "password": "password123",
    "first_name": "Jane",
    "last_name": "Tutor",
    "role": "tutor"
}'
test_endpoint "POST" "/auth/register" "$TUTOR_DATA" "" 201

# Test 4: Login as tutor
echo "4. Login as Tutor"
LOGIN_DATA='{
    "email": "tutor@test.com",
    "password": "password123"
}'
login_response=$(curl -s -X POST "$BASE_URL/auth/login" \
    -H "Content-Type: application/json" \
    -d "$LOGIN_DATA")

TOKEN=$(echo $login_response | jq -r '.token' 2>/dev/null)
if [ "$TOKEN" != "null" ] && [ -n "$TOKEN" ]; then
    echo -e "${GREEN}✅ Login successful${NC}"
    echo "Token: ${TOKEN:0:20}..."
else
    echo -e "${RED}❌ Login failed${NC}"
    echo "$login_response"
fi
echo ""

# Test 5: Create tutor profile
echo "5. Create Tutor Profile"
TUTOR_PROFILE_DATA='{
    "bio": "Experienced mathematics tutor with 5 years of teaching experience. Specializing in algebra, calculus, and statistics.",
    "specializations": ["Mathematics", "Statistics", "Calculus"],
    "hourly_rate": 5000,
    "years_experience": 5
}'
test_endpoint "POST" "/tutors/profile" "$TUTOR_PROFILE_DATA" "Authorization: Bearer $TOKEN" 201

# Test 6: Get all tutors
echo "6. Get All Tutors"
test_endpoint "GET" "/tutors" "" "" 200

# Test 7: Create a course
echo "7. Create Course"
COURSE_DATA='{
    "title": "Introduction to Calculus",
    "description": "Learn the fundamentals of calculus including limits, derivatives, and integrals.",
    "price": 10000,
    "duration_minutes": 90,
    "category": "Mathematics",
    "difficulty_level": "intermediate"
}'
test_endpoint "POST" "/courses" "$COURSE_DATA" "Authorization: Bearer $TOKEN" 201

# Test 8: Get all courses
echo "8. Get All Courses"
test_endpoint "GET" "/courses" "" "" 200

# Test 9: Login as student
echo "9. Login as Student"
STUDENT_LOGIN_DATA='{
    "email": "student@test.com",
    "password": "password123"
}'
student_login_response=$(curl -s -X POST "$BASE_URL/auth/login" \
    -H "Content-Type: application/json" \
    -d "$STUDENT_LOGIN_DATA")

STUDENT_TOKEN=$(echo $student_login_response | jq -r '.token' 2>/dev/null)
if [ "$STUDENT_TOKEN" != "null" ] && [ -n "$STUDENT_TOKEN" ]; then
    echo -e "${GREEN}✅ Student login successful${NC}"
else
    echo -e "${RED}❌ Student login failed${NC}"
    echo "$student_login_response"
fi
echo ""

# Test 10: Test error endpoint
echo "10. Test Error Endpoint"
test_endpoint "GET" "/test-error" "" "" 404

echo "🎉 API Testing Complete!"
echo "========================"

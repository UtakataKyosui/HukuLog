# 1. ユーザー登録
curl -X POST http://localhost:5151/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@test.com",
    "password": "secure123"
  }' | jq

# 2. ログイン
curl -X POST http://localhost:5151/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@test.com",
    "password": "secure123"
  }' | jq

# 3. レスポンスからJWTトークンをコピーして、現在のユーザー情報を取得
curl -X GET http://localhost:5151/api/auth/current \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." | jq
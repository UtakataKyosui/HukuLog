# 1. パスワードリセット要求
curl -X POST http://localhost:5151/api/auth/forgot \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@test.com"
  }' | jq

# 2. メールからリセットトークンを取得後、新しいパスワードを設定
curl -X POST http://localhost:5151/api/auth/reset \
  -H "Content-Type: application/json" \
  -d '{
    "token": "RESET_TOKEN_HERE",
    "password": "newsecure456"
  }' | jq
 
# 3. 新しいパスワードでログインテスト
curl -X POST http://localhost:5151/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@test.com",
    "password": "newsecure456"
  }' | jq
# HukuLog Backend :train:

HukuLogのバックエンドAPI。[Loco](https://loco.rs) フレームワークを使用したRustアプリケーション。

## 🔐 認証システム

### Passkey認証 (WebAuthn)
- **WebAuthn Level 2準拠**のPasskey認証を実装
- パスワード不要の安全な認証
- 指紋認証、顔認証、セキュリティキーに対応

### 認証エンドポイント
- `POST /api/passkey/register/begin` - Passkey登録開始
- `POST /api/passkey/register/finish` - Passkey登録完了
- `POST /api/passkey/authenticate/begin` - Passkey認証開始
- `POST /api/passkey/authenticate/finish` - Passkey認証完了

### 従来認証
- `POST /api/auth/login` - メール＋パスワードログイン
- `POST /api/auth/register` - 新規ユーザー登録
- `POST /api/auth/forgot` - パスワードリセット

## 🗄️ データベース

### Passkey関連フィールド
- `passkey_credentials` (JSON) - WebAuthn認証情報
- `passkey_challenge` (String) - 認証チャレンジ
- `passkey_challenge_expiration` (Timestamp) - チャレンジ有効期限

### マイグレーション
```bash
# Passkeyフィールド追加
cargo loco db migrate

# 開発用データベースリセット
cargo loco db reset
```


## Quick Start

```sh
cargo loco start
```

```sh
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

## Full Stack Serving

You can check your [configuration](config/development.yaml) to pick either frontend setup or server-side rendered template, and activate the relevant configuration sections.


## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).

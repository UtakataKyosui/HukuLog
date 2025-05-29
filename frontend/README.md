# HukuLog Frontend

HukuLogのフロントエンドアプリケーション。[Next.js](https://nextjs.org) + TypeScript + Tailwind CSSで構築。

## 🔐 Passkey認証

### 認証コンポーネント
- `PasskeyAuth` - Passkey登録・認証UI
- `LogInForm` - 従来のメール＋パスワードログイン
- `SignUpForm` - 新規ユーザー登録

### 認証フロー
1. **Passkey登録**: メールアドレス入力 → WebAuthn登録 → 認証情報保存
2. **Passkeyログイン**: メールアドレス入力 → WebAuthn認証 → JWTトークン取得

### 技術スタック
- **WebAuthn**: @simplewebauthn/browser
- **UI**: Radix UI + Tailwind CSS
- **アイコン**: Lucide React

## Getting Started

First, run the development server:

```bash
npm run dev
# or
yarn dev
# or
pnpm dev
# or
bun dev
```

Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

You can start editing the page by modifying `app/page.tsx`. The page auto-updates as you edit the file.

This project uses [`next/font`](https://nextjs.org/docs/app/building-your-application/optimizing/fonts) to automatically optimize and load [Geist](https://vercel.com/font), a new font family for Vercel.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js) - your feedback and contributions are welcome!

## Deploy on Vercel

The easiest way to deploy your Next.js app is to use the [Vercel Platform](https://vercel.com/new?utm_medium=default-template&filter=next.js&utm_source=create-next-app&utm_campaign=create-next-app-readme) from the creators of Next.js.

Check out our [Next.js deployment documentation](https://nextjs.org/docs/app/building-your-application/deploying) for more details.

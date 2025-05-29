"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Fingerprint, Mail, UserPlus, LogIn } from "lucide-react"
import { startRegistration, startAuthentication } from '@simplewebauthn/browser'

interface PasskeyAuthProps {
  mode: 'register' | 'login'
}

export default function PasskeyAuth({ mode }: PasskeyAuthProps) {
  const [email, setEmail] = useState("")
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState("")
  const [success, setSuccess] = useState("")

  const handlePasskeyRegistration = async () => {
    if (!email) {
      setError("メールアドレスを入力してください")
      return
    }

    setIsLoading(true)
    setError("")
    setSuccess("")

    try {
      // Step 1: Get registration options from backend
      const optionsResponse = await fetch('/api/passkey/register/begin', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email }),
      })

      if (!optionsResponse.ok) {
        throw new Error('登録オプションの取得に失敗しました')
      }

      const options = await optionsResponse.json()

      // Step 2: Start WebAuthn registration
      const attResp = await startRegistration(options)

      // Step 3: Send credential to backend for verification
      const verificationResponse = await fetch('/api/passkey/register/finish', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          email,
          credential: attResp,
        }),
      })

      if (!verificationResponse.ok) {
        throw new Error('Passkey登録の完了に失敗しました')
      }

      const result = await verificationResponse.json()
      setSuccess("Passkeyが正常に登録されました！")
      
    } catch (err: any) {
      console.error('Passkey registration error:', err)
      setError(err.message || 'Passkey登録中にエラーが発生しました')
    } finally {
      setIsLoading(false)
    }
  }

  const handlePasskeyAuthentication = async () => {
    if (!email) {
      setError("メールアドレスを入力してください")
      return
    }

    setIsLoading(true)
    setError("")
    setSuccess("")

    try {
      // Step 1: Get authentication options from backend
      const optionsResponse = await fetch('/api/passkey/authenticate/begin', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ email }),
      })

      if (!optionsResponse.ok) {
        throw new Error('認証オプションの取得に失敗しました')
      }

      const options = await optionsResponse.json()

      // Step 2: Start WebAuthn authentication
      const authResp = await startAuthentication(options)

      // Step 3: Send credential to backend for verification
      const verificationResponse = await fetch('/api/passkey/authenticate/finish', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          email,
          credential: authResp,
        }),
      })

      if (!verificationResponse.ok) {
        throw new Error('Passkey認証に失敗しました')
      }

      const result = await verificationResponse.json()
      setSuccess("認証が成功しました！")
      
      // Store JWT token and redirect
      if (result.token) {
        localStorage.setItem('auth_token', result.token)
        window.location.href = '/'
      }
      
    } catch (err: any) {
      console.error('Passkey authentication error:', err)
      setError(err.message || 'Passkey認証中にエラーが発生しました')
    } finally {
      setIsLoading(false)
    }
  }

  const isRegisterMode = mode === 'register'

  return (
    <Card className="w-full max-w-md mx-auto">
      <CardHeader className="space-y-1">
        <CardTitle className="text-2xl font-bold text-center flex items-center justify-center gap-2">
          <Fingerprint className="h-6 w-6" />
          {isRegisterMode ? 'Passkey登録' : 'Passkeyログイン'}
        </CardTitle>
        <CardDescription className="text-center">
          {isRegisterMode 
            ? 'Passkeyを使用して安全にアカウントを作成しましょう'
            : 'Passkeyを使用して安全にログインしましょう'
          }
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="email">メールアドレス</Label>
          <div className="relative">
            <Mail className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
            <Input
              id="email"
              type="email"
              placeholder="example@email.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              className="pl-10"
              required
            />
          </div>
        </div>

        {error && (
          <div className="p-3 text-sm text-red-600 bg-red-50 border border-red-200 rounded-md">
            {error}
          </div>
        )}

        {success && (
          <div className="p-3 text-sm text-green-600 bg-green-50 border border-green-200 rounded-md">
            {success}
          </div>
        )}

        <Button
          onClick={isRegisterMode ? handlePasskeyRegistration : handlePasskeyAuthentication}
          disabled={isLoading || !email}
          className="w-full"
        >
          {isLoading ? (
            <div className="flex items-center gap-2">
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
              処理中...
            </div>
          ) : (
            <div className="flex items-center gap-2">
              {isRegisterMode ? <UserPlus className="h-4 w-4" /> : <LogIn className="h-4 w-4" />}
              {isRegisterMode ? 'Passkeyで登録' : 'Passkeyでログイン'}
            </div>
          )}
        </Button>

        <div className="text-xs text-gray-500 text-center">
          Passkeyは指紋認証、顔認証、またはセキュリティキーを使用した
          <br />
          安全でパスワード不要の認証方法です
        </div>
      </CardContent>
    </Card>
  )
}
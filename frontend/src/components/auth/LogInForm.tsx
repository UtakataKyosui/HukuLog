"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Eye, EyeOff, Mail, Lock } from "lucide-react"
import { login } from "@/lib/api"

export default function LogInForm() {
	const [showPassword, setShowPassword] = useState(false)
	const [email, setEmail] = useState("")
	const [password, setPassword] = useState("")
	const [isLoading, setIsLoading] = useState(false)
	const [error, setError] = useState("")
	const router = useRouter()

	const handleSubmit = async (e: React.FormEvent) => {
		e.preventDefault()
		setIsLoading(true)
		setError("")

		try {
			const response = await login({ email, password })
			
			// Store the token in localStorage
			localStorage.setItem('token', response.token)
			localStorage.setItem('user', JSON.stringify({
				id: response.pid,
				name: response.name,
				email: email,
				isVerified: response.is_verified
			}))

			// Redirect to home page
			router.push('/')
		} catch (err: any) {
			console.error('Login error:', err)
			setError(err.message || 'ログインに失敗しました')
		} finally {
			setIsLoading(false)
		}
	}

	return (
		<Card className="w-full max-w-md mx-auto">
			<CardHeader className="space-y-1">
				<CardTitle className="text-2xl font-bold text-center">ログイン</CardTitle>
				<CardDescription className="text-center">
					アカウントにログインしてHukuLogを始めましょう
				</CardDescription>
			</CardHeader>
			<CardContent>
				<form onSubmit={handleSubmit} className="space-y-4">
					{error && (
						<div className="p-3 text-sm text-red-600 bg-red-50 border border-red-200 rounded-md">
							{error}
						</div>
					)}
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
								disabled={isLoading}
							/>
						</div>
					</div>
					<div className="space-y-2">
						<Label htmlFor="password">パスワード</Label>
						<div className="relative">
							<Lock className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
							<Input
								id="password"
								type={showPassword ? "text" : "password"}
								placeholder="パスワードを入力"
								value={password}
								onChange={(e) => setPassword(e.target.value)}
								className="pl-10 pr-10"
								required
								disabled={isLoading}
							/>
							<button
								type="button"
								onClick={() => setShowPassword(!showPassword)}
								className="absolute right-3 top-3 h-4 w-4 text-gray-400 hover:text-gray-600"
								disabled={isLoading}
							>
								{showPassword ? <EyeOff /> : <Eye />}
							</button>
						</div>
					</div>
					<Button type="submit" className="w-full" disabled={isLoading}>
						{isLoading ? "ログイン中..." : "ログイン"}
					</Button>
				</form>
			</CardContent>
		</Card>
	)
}
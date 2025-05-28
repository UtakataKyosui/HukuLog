"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Eye, EyeOff, Mail, Lock } from "lucide-react"

export default function LogInForm() {
	const [showPassword, setShowPassword] = useState(false)
	const [email, setEmail] = useState("")
	const [password, setPassword] = useState("")

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault()
		// TODO: Implement login logic
		console.log("Login attempt:", { email, password })
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
							/>
							<button
								type="button"
								onClick={() => setShowPassword(!showPassword)}
								className="absolute right-3 top-3 h-4 w-4 text-gray-400 hover:text-gray-600"
							>
								{showPassword ? <EyeOff /> : <Eye />}
							</button>
						</div>
					</div>
					<Button type="submit" className="w-full">
						ログイン
					</Button>
				</form>
			</CardContent>
		</Card>
	)
}
"use client"

import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Eye, EyeOff, Mail, Lock, User } from "lucide-react"

export default function SignUpForm() {
	const [showPassword, setShowPassword] = useState(false)
	const [showConfirmPassword, setShowConfirmPassword] = useState(false)
	const [formData, setFormData] = useState({
		username: "",
		email: "",
		password: "",
		confirmPassword: ""
	})

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault()
		if (formData.password !== formData.confirmPassword) {
			alert("パスワードが一致しません")
			return
		}
		// TODO: Implement signup logic
		console.log("Signup attempt:", formData)
	}

	const handleInputChange = (field: string, value: string) => {
		setFormData(prev => ({ ...prev, [field]: value }))
	}

	return (
		<Card className="w-full max-w-md mx-auto">
			<CardHeader className="space-y-1">
				<CardTitle className="text-2xl font-bold text-center">新規登録</CardTitle>
				<CardDescription className="text-center">
					新しいアカウントを作成してHukuLogを始めましょう
				</CardDescription>
			</CardHeader>
			<CardContent>
				<form onSubmit={handleSubmit} className="space-y-4">
					<div className="space-y-2">
						<Label htmlFor="username">ユーザー名</Label>
						<div className="relative">
							<User className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
							<Input
								id="username"
								type="text"
								placeholder="ユーザー名を入力"
								value={formData.username}
								onChange={(e) => handleInputChange("username", e.target.value)}
								className="pl-10"
								required
							/>
						</div>
					</div>
					<div className="space-y-2">
						<Label htmlFor="email">メールアドレス</Label>
						<div className="relative">
							<Mail className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
							<Input
								id="email"
								type="email"
								placeholder="example@email.com"
								value={formData.email}
								onChange={(e) => handleInputChange("email", e.target.value)}
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
								value={formData.password}
								onChange={(e) => handleInputChange("password", e.target.value)}
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
					<div className="space-y-2">
						<Label htmlFor="confirmPassword">パスワード確認</Label>
						<div className="relative">
							<Lock className="absolute left-3 top-3 h-4 w-4 text-gray-400" />
							<Input
								id="confirmPassword"
								type={showConfirmPassword ? "text" : "password"}
								placeholder="パスワードを再入力"
								value={formData.confirmPassword}
								onChange={(e) => handleInputChange("confirmPassword", e.target.value)}
								className="pl-10 pr-10"
								required
							/>
							<button
								type="button"
								onClick={() => setShowConfirmPassword(!showConfirmPassword)}
								className="absolute right-3 top-3 h-4 w-4 text-gray-400 hover:text-gray-600"
							>
								{showConfirmPassword ? <EyeOff /> : <Eye />}
							</button>
						</div>
					</div>
					<Button type="submit" className="w-full">
						アカウント作成
					</Button>
				</form>
			</CardContent>
		</Card>
	)
}
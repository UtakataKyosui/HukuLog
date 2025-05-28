"use client"

import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import LogInForm from "@/components/auth/LogInForm"
import SignUpForm from "@/components/auth/SignUpForm"

export default function AuthPage() {
	return (
		<div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 px-4">
			<div className="w-full max-w-md">
				<div className="text-center mb-8">
					<h1 className="text-3xl font-bold text-gray-900 mb-2">HukuLog</h1>
					<p className="text-gray-600">あなたの服を管理して、毎日をもっとおしゃれに</p>
				</div>
				
				<Tabs defaultValue="login" className="w-full">
					<TabsList className="grid w-full grid-cols-2">
						<TabsTrigger value="login">ログイン</TabsTrigger>
						<TabsTrigger value="signup">新規登録</TabsTrigger>
					</TabsList>
					<TabsContent value="login" className="mt-6">
						<LogInForm />
					</TabsContent>
					<TabsContent value="signup" className="mt-6">
						<SignUpForm />
					</TabsContent>
				</Tabs>
			</div>
		</div>
	)
}
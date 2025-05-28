"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, Shirt, Search } from "lucide-react"
import Link from "next/link"

export default function ClosetPage() {
	return (
		<div className="container mx-auto px-4 py-8">
			<div className="flex items-center justify-between mb-8">
				<div>
					<h1 className="text-3xl font-bold text-gray-900">マイクローゼット</h1>
					<p className="text-gray-600 mt-2">あなたの服を管理しましょう</p>
				</div>
				<Button className="flex items-center gap-2">
					<Plus className="h-4 w-4" />
					服を追加
				</Button>
			</div>

			<div className="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
				{/* Placeholder clothing items */}
				{[1, 2, 3, 4, 5, 6].map((item) => (
					<Card key={item} className="hover:shadow-lg transition-shadow cursor-pointer">
						<CardHeader className="pb-3">
							<div className="w-full h-48 bg-gray-100 rounded-lg flex items-center justify-center">
								<Shirt className="h-12 w-12 text-gray-400" />
							</div>
						</CardHeader>
						<CardContent>
							<CardTitle className="text-lg">アイテム {item}</CardTitle>
							<CardDescription className="mt-1">
								カテゴリー • ブランド名
							</CardDescription>
							<div className="flex items-center justify-between mt-3">
								<span className="text-sm text-gray-500">着用回数: 5回</span>
								<span className="text-sm text-gray-500">春・夏</span>
							</div>
						</CardContent>
					</Card>
				))}
			</div>

			{/* Empty state when no items */}
			<div className="text-center py-16">
				<Shirt className="h-16 w-16 text-gray-300 mx-auto mb-4" />
				<h3 className="text-xl font-semibold text-gray-900 mb-2">
					まだ服が登録されていません
				</h3>
				<p className="text-gray-600 mb-6">
					最初の服を追加して、デジタルクローゼットを始めましょう
				</p>
				<Button asChild>
					<Link href="/auth">
						ログインして始める
					</Link>
				</Button>
			</div>
		</div>
	)
}
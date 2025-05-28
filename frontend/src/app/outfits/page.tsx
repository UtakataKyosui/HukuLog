"use client"

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, Palette, Heart } from "lucide-react"
import Link from "next/link"

export default function OutfitsPage() {
	return (
		<div className="container mx-auto px-4 py-8">
			<div className="flex items-center justify-between mb-8">
				<div>
					<h1 className="text-3xl font-bold text-gray-900">マイコーデ</h1>
					<p className="text-gray-600 mt-2">お気に入りのコーディネートを管理しましょう</p>
				</div>
				<Button className="flex items-center gap-2">
					<Plus className="h-4 w-4" />
					コーデを作成
				</Button>
			</div>

			<div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
				{/* Placeholder outfit items */}
				{[1, 2, 3, 4, 5, 6].map((item) => (
					<Card key={item} className="hover:shadow-lg transition-shadow cursor-pointer">
						<CardHeader className="pb-3">
							<div className="w-full h-64 bg-gradient-to-br from-blue-100 to-purple-100 rounded-lg flex items-center justify-center">
								<Palette className="h-16 w-16 text-gray-400" />
							</div>
						</CardHeader>
						<CardContent>
							<div className="flex items-center justify-between">
								<div>
									<CardTitle className="text-lg">コーデ {item}</CardTitle>
									<CardDescription className="mt-1">
										カジュアル • 春コーデ
									</CardDescription>
								</div>
								<Button variant="ghost" size="sm">
									<Heart className="h-4 w-4" />
								</Button>
							</div>
							<div className="flex items-center justify-between mt-3">
								<span className="text-sm text-gray-500">3アイテム</span>
								<span className="text-sm text-gray-500">2024/05/20</span>
							</div>
						</CardContent>
					</Card>
				))}
			</div>

			{/* Empty state when no outfits */}
			<div className="text-center py-16">
				<Palette className="h-16 w-16 text-gray-300 mx-auto mb-4" />
				<h3 className="text-xl font-semibold text-gray-900 mb-2">
					まだコーデが作成されていません
				</h3>
				<p className="text-gray-600 mb-6">
					最初のコーディネートを作成して、おしゃれを楽しみましょう
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
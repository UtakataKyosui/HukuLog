"use client"

import { useState, useEffect } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, Palette, Heart, Shirt } from "lucide-react"
import Link from "next/link"
import { apiClient, type Outfit } from "@/lib/api"

export default function OutfitsPage() {
	const [outfits, setOutfits] = useState<Outfit[]>([])
	const [loading, setLoading] = useState(true)

	useEffect(() => {
		loadOutfits()
	}, [])

	const loadOutfits = async () => {
		try {
			// Set token for API client
			const token = localStorage.getItem('token')
			if (token) {
				apiClient.setToken(token)
			}
			
			const response = await apiClient.getOutfits()
			setOutfits(response.outfits)
		} catch (error) {
			console.error('Failed to load outfits:', error)
		} finally {
			setLoading(false)
		}
	}

	const formatDate = (dateString: string) => {
		return new Date(dateString).toLocaleDateString('ja-JP', {
			year: 'numeric',
			month: 'numeric',
			day: 'numeric'
		})
	}

	if (loading) {
		return (
			<div className="container mx-auto px-4 py-8">
				<div className="text-center">
					<div className="animate-spin rounded-full h-32 w-32 border-b-2 border-gray-900 mx-auto"></div>
					<p className="mt-4 text-gray-600">読み込み中...</p>
				</div>
			</div>
		)
	}

	return (
		<div className="container mx-auto px-4 py-8">
			<div className="flex items-center justify-between mb-8">
				<div>
					<h1 className="text-3xl font-bold text-gray-900">マイコーデ</h1>
					<p className="text-gray-600 mt-2">お気に入りのコーディネートを管理しましょう</p>
				</div>
				<Button className="flex items-center gap-2" asChild>
					<Link href="/outfits/create">
						<Plus className="h-4 w-4" />
						コーデを作成
					</Link>
				</Button>
			</div>

			{outfits.length > 0 ? (
				<div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
					{outfits.map((outfit) => (
						<Card key={outfit.id} className="hover:shadow-lg transition-shadow cursor-pointer">
							<CardHeader className="pb-3">
								<div className="w-full h-64 bg-gradient-to-br from-blue-100 to-purple-100 rounded-lg flex items-center justify-center">
									{outfit.image_url ? (
										<img
											src={outfit.image_url}
											alt={outfit.name}
											className="w-full h-full object-cover rounded-lg"
										/>
									) : (
										<Palette className="h-16 w-16 text-gray-400" />
									)}
								</div>
							</CardHeader>
							<CardContent>
								<div className="flex items-center justify-between">
									<div>
										<CardTitle className="text-lg">{outfit.name}</CardTitle>
										<CardDescription className="mt-1">
											{outfit.description || 'コーディネート'}
										</CardDescription>
									</div>
									<Button variant="ghost" size="sm">
										<Heart className="h-4 w-4" />
									</Button>
								</div>
								<div className="flex items-center justify-between mt-3">
									<span className="text-sm text-gray-500">
										{outfit.clothings?.length || 0}アイテム
									</span>
									<span className="text-sm text-gray-500">
										{formatDate(outfit.created_at)}
									</span>
								</div>
							</CardContent>
						</Card>
					))}
				</div>
			) : (
				<div className="text-center py-16">
					<Palette className="h-16 w-16 text-gray-300 mx-auto mb-4" />
					<h3 className="text-xl font-semibold text-gray-900 mb-2">
						まだコーデが作成されていません
					</h3>
					<p className="text-gray-600 mb-6">
						最初のコーディネートを作成して、おしゃれを楽しみましょう
					</p>
					<Button asChild>
						<Link href="/outfits/create">
							コーデを作成
						</Link>
					</Button>
				</div>
			)}
		</div>
	)
}
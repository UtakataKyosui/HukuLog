"use client"

import { useState, useEffect } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Plus, Shirt } from "lucide-react"
import Link from "next/link"
import { getClothings, type Clothing } from "@/lib/api"

export default function ClosetPage() {
  const [clothings, setClothings] = useState<Clothing[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState("")

  useEffect(() => {
    const fetchClothings = async () => {
      try {
        const data = await getClothings()
        setClothings(data)
      } catch (err) {
        setError(err instanceof Error ? err.message : "服の取得に失敗しました")
      } finally {
        setIsLoading(false)
      }
    }

    fetchClothings()
  }, [])

  if (isLoading) {
    return (
      <div className="container mx-auto px-4 py-8">
        <div className="flex items-center justify-center py-16">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mx-auto mb-4"></div>
            <p className="text-gray-600">読み込み中...</p>
          </div>
        </div>
      </div>
    )
  }

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">マイクローゼット</h1>
          <p className="text-gray-600 mt-2">あなたの服を管理しましょう</p>
        </div>
        <Button asChild className="flex items-center gap-2">
          <Link href="/closet/add">
            <Plus className="h-4 w-4" />
            服を追加
          </Link>
        </Button>
      </div>

      {error && (
        <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded mb-6">
          {error}
        </div>
      )}

      {clothings.length > 0 ? (
        <div className="grid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {clothings.map((clothing) => (
            <Card key={clothing.id} className="hover:shadow-lg transition-shadow cursor-pointer">
              <CardHeader className="pb-3">
                <div className="w-full h-48 bg-gray-100 rounded-lg flex items-center justify-center overflow-hidden">
                  {clothing.image_url ? (
                    <img 
                      src={clothing.image_url} 
                      alt={clothing.name}
                      className="w-full h-full object-cover"
                    />
                  ) : (
                    <Shirt className="h-12 w-12 text-gray-400" />
                  )}
                </div>
              </CardHeader>
              <CardContent>
                <CardTitle className="text-lg">{clothing.name}</CardTitle>
                <CardDescription className="mt-1">
                  {clothing.color && clothing.material ? `${clothing.color} • ${clothing.material}` : 
                   clothing.color || clothing.material || "詳細なし"}
                </CardDescription>
                {clothing.description && (
                  <p className="text-sm text-gray-600 mt-2 line-clamp-2">{clothing.description}</p>
                )}
                <div className="flex items-center justify-between mt-3">
                  {clothing.purchase_price && (
                    <span className="text-sm text-gray-500">¥{clothing.purchase_price.toLocaleString()}</span>
                  )}
                  {clothing.purchase_date && (
                    <span className="text-sm text-gray-500">
                      {new Date(clothing.purchase_date).toLocaleDateString('ja-JP')}
                    </span>
                  )}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      ) : (
        <div className="text-center py-16">
          <Shirt className="h-16 w-16 text-gray-300 mx-auto mb-4" />
          <h3 className="text-xl font-semibold text-gray-900 mb-2">
            まだ服が登録されていません
          </h3>
          <p className="text-gray-600 mb-6">
            最初の服を追加して、デジタルクローゼットを始めましょう
          </p>
          <Button asChild>
            <Link href="/closet/add">
              服を追加
            </Link>
          </Button>
        </div>
      )}
    </div>
  )
}
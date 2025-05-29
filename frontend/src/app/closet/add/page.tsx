"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Textarea } from "@/components/ui/textarea"
import { ArrowLeft } from "lucide-react"
import Link from "next/link"
import { createClothing } from "@/lib/api"

export default function AddClothingPage() {
  const router = useRouter()
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState("")
  
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    color: "",
    material: "",
    purchase_date: "",
    purchase_price: "",
    image_url: ""
  })

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)
    setError("")

    try {
      const clothingData = {
        ...formData,
        purchase_price: formData.purchase_price ? parseFloat(formData.purchase_price) : undefined
      }
      
      await createClothing(clothingData)
      router.push("/closet")
    } catch (err) {
      setError(err instanceof Error ? err.message : "服の追加に失敗しました")
    } finally {
      setIsLoading(false)
    }
  }

  const handleInputChange = (field: string, value: string) => {
    setFormData(prev => ({ ...prev, [field]: value }))
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-2xl">
      <div className="flex items-center gap-4 mb-8">
        <Button variant="ghost" size="sm" asChild>
          <Link href="/closet">
            <ArrowLeft className="h-4 w-4" />
            戻る
          </Link>
        </Button>
        <div>
          <h1 className="text-3xl font-bold text-gray-900">服を追加</h1>
          <p className="text-gray-600 mt-2">新しいアイテムをクローゼットに追加しましょう</p>
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>アイテム情報</CardTitle>
          <CardDescription>
            服の詳細情報を入力してください
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form onSubmit={handleSubmit} className="space-y-6">
            {error && (
              <div className="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                {error}
              </div>
            )}

            <div className="space-y-2">
              <Label htmlFor="name">アイテム名 *</Label>
              <Input
                id="name"
                value={formData.name}
                onChange={(e) => handleInputChange("name", e.target.value)}
                placeholder="例: 白いTシャツ"
                required
              />
            </div>

            <div className="space-y-2">
              <Label htmlFor="description">説明</Label>
              <Textarea
                id="description"
                value={formData.description}
                onChange={(e) => handleInputChange("description", e.target.value)}
                placeholder="アイテムの詳細説明"
                rows={3}
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="color">色</Label>
                <Input
                  id="color"
                  value={formData.color}
                  onChange={(e) => handleInputChange("color", e.target.value)}
                  placeholder="例: 白"
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="material">素材</Label>
                <Input
                  id="material"
                  value={formData.material}
                  onChange={(e) => handleInputChange("material", e.target.value)}
                  placeholder="例: コットン"
                />
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div className="space-y-2">
                <Label htmlFor="purchase_date">購入日</Label>
                <Input
                  id="purchase_date"
                  type="date"
                  value={formData.purchase_date}
                  onChange={(e) => handleInputChange("purchase_date", e.target.value)}
                />
              </div>

              <div className="space-y-2">
                <Label htmlFor="purchase_price">購入価格</Label>
                <Input
                  id="purchase_price"
                  type="number"
                  value={formData.purchase_price}
                  onChange={(e) => handleInputChange("purchase_price", e.target.value)}
                  placeholder="例: 2980"
                />
              </div>
            </div>

            <div className="space-y-2">
              <Label htmlFor="image_url">画像URL</Label>
              <Input
                id="image_url"
                value={formData.image_url}
                onChange={(e) => handleInputChange("image_url", e.target.value)}
                placeholder="https://example.com/image.jpg"
              />
            </div>

            <div className="flex gap-4 pt-4">
              <Button type="submit" disabled={isLoading} className="flex-1">
                {isLoading ? "追加中..." : "服を追加"}
              </Button>
              <Button type="button" variant="outline" asChild>
                <Link href="/closet">キャンセル</Link>
              </Button>
            </div>
          </form>
        </CardContent>
      </Card>
    </div>
  )
}
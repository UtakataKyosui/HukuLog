"use client"

import { useState, useEffect } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { Label } from "@/components/ui/label"
import { Checkbox } from "@/components/ui/checkbox"
import { ArrowLeft, Shirt, Plus, Check } from "lucide-react"
import Link from "next/link"
import { useRouter } from "next/navigation"
import { getClothings, apiClient, type Clothing, type CreateOutfitRequest } from "@/lib/api"

export default function CreateOutfitPage() {
  const router = useRouter()
  const [clothings, setClothings] = useState<Clothing[]>([])
  const [selectedClothings, setSelectedClothings] = useState<Set<string>>(new Set())
  const [outfitName, setOutfitName] = useState("")
  const [outfitDescription, setOutfitDescription] = useState("")
  const [loading, setLoading] = useState(true)
  const [creating, setCreating] = useState(false)

  useEffect(() => {
    loadClothings()
  }, [])

  const loadClothings = async () => {
    try {
      const clothingsData = await getClothings()
      setClothings(clothingsData)
    } catch (error) {
      console.error('Failed to load clothings:', error)
    } finally {
      setLoading(false)
    }
  }

  const toggleClothingSelection = (clothingId: string) => {
    const newSelection = new Set(selectedClothings)
    if (newSelection.has(clothingId)) {
      newSelection.delete(clothingId)
    } else {
      newSelection.add(clothingId)
    }
    setSelectedClothings(newSelection)
  }

  const handleCreateOutfit = async () => {
    if (!outfitName.trim() || selectedClothings.size === 0) {
      alert('コーデ名と少なくとも1つのアイテムを選択してください')
      return
    }

    setCreating(true)
    try {
      // Set token for API client
      const token = localStorage.getItem('token')
      if (token) {
        apiClient.setToken(token)
      }

      const outfitData: CreateOutfitRequest = {
        name: outfitName,
        description: outfitDescription || undefined,
        clothing_ids: Array.from(selectedClothings),
      }

      await apiClient.createOutfit(outfitData)
      router.push('/outfits')
    } catch (error) {
      console.error('Failed to create outfit:', error)
      alert('コーデの作成に失敗しました')
    } finally {
      setCreating(false)
    }
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
      <div className="flex items-center gap-4 mb-8">
        <Button variant="ghost" size="sm" asChild>
          <Link href="/outfits">
            <ArrowLeft className="h-4 w-4" />
          </Link>
        </Button>
        <div>
          <h1 className="text-3xl font-bold text-gray-900">コーデを作成</h1>
          <p className="text-gray-600 mt-2">お気に入りのアイテムを組み合わせてコーディネートを作りましょう</p>
        </div>
      </div>

      <div className="grid lg:grid-cols-2 gap-8">
        {/* Outfit Details */}
        <Card>
          <CardHeader>
            <CardTitle>コーデ情報</CardTitle>
            <CardDescription>コーディネートの基本情報を入力してください</CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div>
              <Label htmlFor="outfit-name">コーデ名 *</Label>
              <Input
                id="outfit-name"
                placeholder="例: カジュアル春コーデ"
                value={outfitName}
                onChange={(e) => setOutfitName(e.target.value)}
              />
            </div>
            <div>
              <Label htmlFor="outfit-description">説明</Label>
              <Textarea
                id="outfit-description"
                placeholder="このコーディネートについて説明してください"
                value={outfitDescription}
                onChange={(e) => setOutfitDescription(e.target.value)}
                rows={3}
              />
            </div>
            <div className="pt-4">
              <h3 className="font-semibold mb-2">選択中のアイテム</h3>
              <p className="text-sm text-gray-600 mb-3">
                {selectedClothings.size}個のアイテムが選択されています
              </p>
              <Button 
                onClick={handleCreateOutfit} 
                disabled={!outfitName.trim() || selectedClothings.size === 0 || creating}
                className="w-full"
              >
                {creating ? '作成中...' : 'コーデを作成'}
              </Button>
            </div>
          </CardContent>
        </Card>

        {/* Clothing Selection */}
        <Card>
          <CardHeader>
            <CardTitle>アイテムを選択</CardTitle>
            <CardDescription>コーディネートに含めるアイテムを選んでください</CardDescription>
          </CardHeader>
          <CardContent>
            {clothings.length === 0 ? (
              <div className="text-center py-8">
                <Shirt className="h-12 w-12 text-gray-300 mx-auto mb-4" />
                <p className="text-gray-600 mb-4">まだアイテムが登録されていません</p>
                <Button asChild>
                  <Link href="/closet">
                    アイテムを追加
                  </Link>
                </Button>
              </div>
            ) : (
              <div className="grid grid-cols-2 gap-4 max-h-96 overflow-y-auto">
                {clothings.map((clothing) => (
                  <div
                    key={clothing.id}
                    className={`relative border rounded-lg p-3 cursor-pointer transition-all ${
                      selectedClothings.has(clothing.id)
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-200 hover:border-gray-300'
                    }`}
                    onClick={() => toggleClothingSelection(clothing.id)}
                  >
                    <div className="absolute top-2 right-2">
                      <Checkbox
                        checked={selectedClothings.has(clothing.id)}
                        onChange={() => toggleClothingSelection(clothing.id)}
                      />
                    </div>
                    <div className="w-full h-24 bg-gray-100 rounded mb-2 flex items-center justify-center">
                      {clothing.image_url ? (
                        <img
                          src={clothing.image_url}
                          alt={clothing.name}
                          className="w-full h-full object-cover rounded"
                        />
                      ) : (
                        <Shirt className="h-8 w-8 text-gray-400" />
                      )}
                    </div>
                    <h4 className="font-medium text-sm truncate">{clothing.name}</h4>
                    <p className="text-xs text-gray-500 truncate">
                      {clothing.category || 'カテゴリー未設定'}
                    </p>
                  </div>
                ))}
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  )
}
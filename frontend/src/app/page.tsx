"use client"

import Image from "next/image";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@/components/ui/card"
import BackgroundLinearImage from "@/components/background/BackgroundLinearImage";
import { Button } from "@/components/ui/button";
import { useEffect } from "react";
import Link from "next/link";
import { Shirt, Sparkles, Shield, Zap, ArrowRight } from "lucide-react";

export default function Home() {

  return (
	<main className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-indigo-50">
		{/* Hero Section */}
		<section className="container mx-auto px-4 pt-20 pb-16 text-center">
			<div className="max-w-4xl mx-auto">
				<h1 className="text-5xl md:text-6xl font-bold text-gray-900 mb-6">
					服の困りごと、
					<span className="text-blue-600">ありませんか？</span>
				</h1>
				<p className="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
					HukuLogは、あなたのクローゼットを整理し、毎日のコーディネートを楽しくする
					デジタルクローゼットアプリです
				</p>
				<div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
					<Button asChild size="lg" className="text-lg px-8 py-3">
						<Link href="/auth" className="flex items-center gap-2">
							使ってみる！
							<ArrowRight className="h-5 w-5" />
						</Link>
					</Button>
					<Button asChild variant="outline" size="lg" className="text-lg px-8 py-3">
						<Link href="#features">
							詳しく見る
						</Link>
					</Button>
				</div>
			</div>
		</section>

		<BackgroundLinearImage />

		{/* Features Section */}
		<section id="features" className="container mx-auto px-4 py-16">
			<div className="text-center mb-16">
				<h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
					HukuLogの特徴
				</h2>
				<p className="text-lg text-gray-600 max-w-2xl mx-auto">
					あなたのファッションライフをより豊かにする機能をご紹介します
				</p>
			</div>

			<div className="grid md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-6xl mx-auto">
				<Card className="border-0 shadow-lg hover:shadow-xl transition-shadow duration-300">
					<CardHeader className="text-center">
						<div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<Shirt className="h-8 w-8 text-blue-600" />
						</div>
						<CardTitle className="text-xl">
							服の見える化
						</CardTitle>
						<CardDescription>
							クローゼットの中身を整理して管理
						</CardDescription>
					</CardHeader>
					<CardContent className="text-center">
						クローゼットの奥に眠ってる服、もったいないですよね？ HukuLogで全部の服を「見える化」しちゃいましょう！
					</CardContent>	
				</Card>

				<Card className="border-0 shadow-lg hover:shadow-xl transition-shadow duration-300">
					<CardHeader className="text-center">
						<div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<Sparkles className="h-8 w-8 text-green-600" />
						</div>
						<CardTitle className="text-xl">
							コーディネート提案
						</CardTitle>
						<CardDescription>
							AIがあなたにぴったりのコーデを提案
						</CardDescription>
					</CardHeader>
					<CardContent className="text-center">
						持ってる服を最大限に活かして、ムダな衝動買いも減らせるかも？ 今日からあなたのファッションライフをアップデート！
					</CardContent>
				</Card>

				<Card className="border-0 shadow-lg hover:shadow-xl transition-shadow duration-300">
					<CardHeader className="text-center">
						<div className="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
							<Shield className="h-8 w-8 text-purple-600" />
						</div>
						<CardTitle className="text-xl">
							お手入れ管理
						</CardTitle>
						<CardDescription>
							洗濯方法やケア方法を記録
						</CardDescription>
					</CardHeader>
					<CardContent className="text-center">
						「あっ！この服のタグ、捨てちゃった…どうやって洗えばいいんだっけ？」HukuLogなら、服ごとにお手入れ方法をメモできます。
					</CardContent>
				</Card>
			</div>
		</section>

		{/* CTA Section */}
		<section className="bg-blue-600 text-white py-16">
			<div className="container mx-auto px-4 text-center">
				<h2 className="text-3xl md:text-4xl font-bold mb-4">
					今すぐHukuLogを始めよう
				</h2>
				<p className="text-xl mb-8 opacity-90">
					無料でアカウントを作成して、あなたのデジタルクローゼットを作りましょう
				</p>
				<Button asChild size="lg" variant="secondary" className="text-lg px-8 py-3">
					<Link href="/auth" className="flex items-center gap-2">
						<Zap className="h-5 w-5" />
						無料で始める
					</Link>
				</Button>
			</div>
		</section>
	</main>
  )
}


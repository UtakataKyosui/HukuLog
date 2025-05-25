"use client"

import Image from "next/image";
import {
	Card,
	CardContent,
	CardDescription,
	CardFooter,
	CardHeader,
	CardTitle,
  } from "@/components/ui/card"
import BackgroundLinearImage from "@/components/background/BackgroundLinearImage";
import { Button } from "@/components/ui/button";
import { useEffect } from "react";
import Link from "next/link";

export default function Home() {
	useEffect(() => {
		document.body.classList.add('overflow-hidden');
		return () => {
			document.body.classList.remove('overflow-hidden');
		}
	},[])
  return (
	<main className="container mx-auto px-[5vw] flex flex-col justify-center items-center gap-10 my-5"> 
		<section className="w-[40%] border rounded-xl bg-white/80 p-1 shadow-md">
			<div className="text-center">
				<h3 className="text-xl">服の困りごと、ありませんか？</h3>
				<p className="text-sm text-slate-500">HukuLogは、あなたの困りごとを解決します</p>
			</div>
		</section>

		<BackgroundLinearImage />
		<section className="flex gap-6 w-[80%]">
			<Card className="w-full">
				<CardHeader>
					<CardTitle>
						最適な服の管理方法をあなたに
					</CardTitle>
					<CardDescription>
						服の管理をして、毎日おしゃれな着こなしをしてみよう。
					</CardDescription>
				</CardHeader>
				<CardContent>
					クローゼットの奥に眠ってる服、もったいないですよね？ HukuLogで全部の服を「見える化」しちゃいましょう！ 持ってる服を最大限に活かして、ムダな衝動買いも減らせるかも？ 今日からあなたのファッションライフをアップデート！
				</CardContent>	
			</Card>
			<Card className="w-full">
				<CardHeader>
					<CardTitle>
						服の正しい洗濯方法がなんだったかを思い出そう
					</CardTitle>
					<CardDescription>
						服についてたタグ、処分してから洗濯方法忘れちゃってない？
					</CardDescription>
				</CardHeader>
				<CardContent>
					「あっ！この服のタグ、捨てちゃった…どうやって洗えばいいんだっけ？」って、あるあるですよね？ HukuLogなら、服ごとに洗濯方法やお手入れのコツをメモっておけるんです。これで、お気に入りの服をうっかりダメにしちゃう心配もなし！
				</CardContent>
			</Card>
		</section>
		<Button asChild variant="default" className="w-[8vw] h-[5vh] font-bold">
			<Link href="/auth">
				使ってみる！
			</Link>
		</Button>
	</main>
    
  )
}


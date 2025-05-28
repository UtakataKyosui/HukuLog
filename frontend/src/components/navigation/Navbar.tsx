"use client"

import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Shirt, User, Home } from "lucide-react"

export default function Navbar() {
	return (
		<header className="border-b bg-white/80 backdrop-blur-sm sticky top-0 z-50">
			<div className="container mx-auto px-4 py-3">
				<div className="flex items-center justify-between">
					<Link href="/" className="flex items-center space-x-2">
						<Shirt className="h-8 w-8 text-blue-600" />
						<h1 className="text-2xl font-bold text-gray-900">HukuLog</h1>
					</Link>
					
					<nav className="hidden md:flex items-center space-x-6">
						<Link 
							href="/" 
							className="flex items-center space-x-1 text-gray-600 hover:text-gray-900 transition-colors"
						>
							<Home className="h-4 w-4" />
							<span>ホーム</span>
						</Link>
						<Link 
							href="/closet" 
							className="text-gray-600 hover:text-gray-900 transition-colors"
						>
							クローゼット
						</Link>
						<Link 
							href="/outfits" 
							className="text-gray-600 hover:text-gray-900 transition-colors"
						>
							コーデ
						</Link>
					</nav>

					<div className="flex items-center space-x-2">
						<Button variant="ghost" size="sm" asChild>
							<Link href="/auth">
								<User className="h-4 w-4 mr-1" />
								ログイン
							</Link>
						</Button>
						<Button size="sm" asChild>
							<Link href="/auth">
								始める
							</Link>
						</Button>
					</div>
				</div>
			</div>
		</header>
	)
}
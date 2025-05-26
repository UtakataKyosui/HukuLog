import { Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselPrevious } from "../ui/carousel"
import { Card, CardContent } from "../ui/card"
import AutoScroll from "embla-carousel-auto-scroll"

export default function BackgroundLinearImage() {
	return (
		<span className="w-screen z-[-5] top-3 absolute grid gap-12 opacity-30 [&>div]:h-[30%] pointer-events-none">
			<Carousel  opts={{
				loop: true,
			}} plugins={[AutoScroll({
				stopOnInteraction: false, // ユーザー操作で停止しない
				stopOnMouseEnter: false,  // マウスオーバーで停止しない
				stopOnFocusIn: false,      // フォーカスで停止しない
				direction: "forward"
			})]}>
				<CarouselContent>
					{Array.from({ length: 4 }).map((_, index) => (
					<CarouselItem key={index} className="basis-1/3">
						<div className="p-1">
						<Card>
							<CardContent className="flex aspect-square items-center justify-center p-6">
								<img src={`/Huku${index}.png`} />
							</CardContent>
						</Card>
						</div>
					</CarouselItem>
					))}
				</CarouselContent>
				{/* <CarouselPrevious />
				<CarouselNext /> */}
			</Carousel>
			<Carousel className="h-[15vh]" opts={{
				loop: true,
			}} plugins={[AutoScroll({
				stopOnInteraction: false, // ユーザー操作で停止しない
				stopOnMouseEnter: false,  // マウスオーバーで停止しない
				stopOnFocusIn: false,      // フォーカスで停止しない
				direction: "backward"
			})]}>
				<CarouselContent>
					{Array.from({ length: 4 }).map((_, index) => (
					<CarouselItem key={index} className="basis-1/3">
						<div className="p-1">
						<Card>
							<CardContent className="flex aspect-square items-center justify-center p-6">
								<img src={`/Huku${index + 5}.png`} />
							</CardContent>
						</Card>
						</div>
					</CarouselItem>
					))}
				</CarouselContent>
				{/* <CarouselPrevious />
				<CarouselNext /> */}
			</Carousel>
		</span>
		
	)
}
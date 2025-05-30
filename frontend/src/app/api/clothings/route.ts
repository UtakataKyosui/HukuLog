import { NextRequest, NextResponse } from 'next/server';
import { Clothing, CreateClothingRequest } from '@/lib/api';

// 認証トークンの検証
async function verifyToken(token: string | null): Promise<string | null> {
  if (!token) return null;
  
  try {
    // TODO: Rustバックエンドに認証トークンを検証するリクエストを送信
    // 例: const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/auth/verify`, {...});
    return 'dummy_user_id'; // 仮の実装
  } catch {
    return null;
  }
}

// GET: 服一覧の取得
export async function GET(request: NextRequest) {
  try {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
    const userId = await verifyToken(token);
    
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    // TODO: 実際のデータベースからの取得ロジックを実装
    const clothings: Clothing[] = [];
    
    return NextResponse.json({
      clothings,
      total: clothings.length,
      page: 1,
      page_size: 20,
      total_pages: 1
    });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to fetch clothings' },
      { status: 500 }
    );
  }
}

// POST: 新規服の作成
export async function POST(request: NextRequest) {
  try {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
    const userId = await verifyToken(token);
    
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body: CreateClothingRequest = await request.json();
    
    // TODO: 実際のデータベースへの保存ロジックを実装
    const clothing: Clothing = {
      id: 'dummy_id',
      name: body.name,
      description: body.description,
      image_url: body.image_url,
      color: body.color,
      material: body.material,
      purchase_date: body.purchase_date,
      purchase_price: body.purchase_price,
      wear_count: 0,
      user_id: userId,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    };

    return NextResponse.json(clothing);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to create clothing' },
      { status: 500 }
    );
  }
} 
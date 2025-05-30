import { NextRequest, NextResponse } from 'next/server';
import { Outfit, CreateOutfitRequest } from '@/lib/api';

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

// GET: コーディネート一覧の取得
export async function GET(request: NextRequest) {
  try {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
    const userId = await verifyToken(token);
    
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    // TODO: 実際のデータベースからの取得ロジックを実装
    const outfits: Outfit[] = [];
    
    return NextResponse.json({
      outfits,
      total: outfits.length,
      page: 1,
      page_size: 20,
      total_pages: 1
    });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to fetch outfits' },
      { status: 500 }
    );
  }
}

// POST: 新規コーディネートの作成
export async function POST(request: NextRequest) {
  try {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
    const userId = await verifyToken(token);
    
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const body: CreateOutfitRequest = await request.json();
    
    // TODO: 実際のデータベースへの保存ロジックを実装
    const outfit: Outfit = {
      id: 'dummy_id',
      name: body.name,
      description: body.description,
      image_url: body.image_url,
      user_id: userId,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString()
    };

    return NextResponse.json(outfit);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to create outfit' },
      { status: 500 }
    );
  }
}

// DELETE: コーディネートの削除
export async function DELETE(request: NextRequest) {
  try {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '') ?? null;
    const userId = await verifyToken(token);
    
    if (!userId) {
      return NextResponse.json({ error: 'Unauthorized' }, { status: 401 });
    }

    const { searchParams } = new URL(request.url);
    const id = searchParams.get('id');

    if (!id) {
      return NextResponse.json(
        { error: 'Outfit ID is required' },
        { status: 400 }
      );
    }

    // TODO: 実際のデータベースからの削除ロジックを実装

    return NextResponse.json({ message: 'Outfit deleted successfully' });
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to delete outfit' },
      { status: 500 }
    );
  }
} 
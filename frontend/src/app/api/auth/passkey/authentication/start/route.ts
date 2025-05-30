import { NextRequest, NextResponse } from 'next/server';
import { RequestChallengeResponse } from '@/lib/api';

export async function POST(request: NextRequest) {
  try {
    const { email } = await request.json();
    
    // TODO: 実際のパスキー認証開始ロジックを実装
    const response: RequestChallengeResponse = {
      challenge: 'dummy_challenge'
    };

    return NextResponse.json(response);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to start passkey authentication' },
      { status: 500 }
    );
  }
} 
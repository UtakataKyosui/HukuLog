import { NextRequest, NextResponse } from 'next/server';
import { CreationChallengeResponse } from '@/lib/api';

export async function POST(request: NextRequest) {
  try {
    const { email } = await request.json();
    
    // TODO: 実際のパスキー登録開始ロジックを実装
    const response: CreationChallengeResponse = {
      challenge: 'dummy_challenge',
      user: {
        id: 'dummy_id',
        name: 'User Name'
      }
    };

    return NextResponse.json(response);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to start passkey registration' },
      { status: 500 }
    );
  }
} 
import { NextRequest, NextResponse } from 'next/server';
import { AuthenticationFinishResponse } from '@/lib/api';

export async function POST(request: NextRequest) {
  try {
    const { email, credential } = await request.json();
    
    // TODO: 実際のパスキー認証完了ロジックを実装
    const response: AuthenticationFinishResponse = {
      status: 'success',
      message: 'Authentication successful',
      token: 'dummy_token',
      user: {
        id: 1,
        email: email,
        name: 'User Name',
        verified: true
      }
    };

    return NextResponse.json(response);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to complete passkey authentication' },
      { status: 500 }
    );
  }
} 
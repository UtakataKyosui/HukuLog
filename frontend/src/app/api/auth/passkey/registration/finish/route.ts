import { NextRequest, NextResponse } from 'next/server';
import { RegistrationFinishResponse } from '@/lib/api';

export async function POST(request: NextRequest) {
  try {
    const { email, credential } = await request.json();
    
    // TODO: 実際のパスキー登録完了ロジックを実装
    const response: RegistrationFinishResponse = {
      status: 'success',
      message: 'Passkey registration completed successfully'
    };

    return NextResponse.json(response);
  } catch (error) {
    return NextResponse.json(
      { error: 'Failed to complete passkey registration' },
      { status: 500 }
    );
  }
} 
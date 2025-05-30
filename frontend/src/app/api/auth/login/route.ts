import { NextRequest, NextResponse } from 'next/server';
import { LoginRequest, LoginResponse } from '@/lib/api';

export async function POST(request: NextRequest) {
  try {
    const body: LoginRequest = await request.json();
    
    // TODO: 実際の認証ロジックを実装
    const response: LoginResponse = {
      token: 'dummy_token',
      pid: 'dummy_pid',
      name: 'User Name',
      is_verified: true
    };

    return NextResponse.json(response);
  } catch (error) {
    return NextResponse.json(
      { error: 'Authentication failed' },
      { status: 401 }
    );
  }
} 
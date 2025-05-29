const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5151/api';

export interface Clothing {
  id: string;
  name: string;
  description?: string;
  image_url?: string;
  category?: string;
  brand?: string;
  color?: string;
  size?: string;
  material?: string;
  season?: string;
  condition?: string;
  purchase_date?: string;
  purchase_price?: number;
  wear_count: number;
  last_worn?: string;
  notes?: string;
  user_id: string;
  created_at: string;
  updated_at: string;
}

export interface Outfit {
  id: string;
  name: string;
  description?: string;
  image_url?: string;
  user_id: string;
  created_at: string;
  updated_at: string;
  clothings?: Clothing[];
}

export interface CreateOutfitRequest {
  name: string;
  description?: string;
  image_url?: string;
  clothing_ids: string[];
}

export interface CreateClothingRequest {
  name: string;
  description?: string;
  image_url?: string;
  color?: string;
  material?: string;
  purchase_date?: string;
  purchase_price?: number;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  pid: string;
  name: string;
  is_verified: boolean;
}

class ApiClient {
  private baseUrl: string;
  private token: string | null = null;

  constructor(baseUrl: string) {
    this.baseUrl = baseUrl;
  }

  setToken(token: string) {
    this.token = token;
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${this.baseUrl}${endpoint}`;
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (this.token) {
      headers.Authorization = `Bearer ${this.token}`;
    }

    const response = await fetch(url, {
      ...options,
      headers,
    });

    if (!response.ok) {
      throw new Error(`API request failed: ${response.status} ${response.statusText}`);
    }

    return response.json();
  }

  // Clothing API methods
  async getClothings(): Promise<{ clothings: Clothing[]; total: number; page: number; page_size: number; total_pages: number }> {
    return this.request('/clothings');
  }

  async getClothing(id: string): Promise<Clothing> {
    return this.request(`/clothings/${id}`);
  }

  async createClothing(clothing: CreateClothingRequest): Promise<Clothing> {
    return this.request('/clothings', {
      method: 'POST',
      body: JSON.stringify(clothing),
    });
  }

  // Outfit API methods
  async getOutfits(): Promise<{ outfits: Outfit[]; total: number; page: number; page_size: number; total_pages: number }> {
    return this.request('/outfits');
  }

  async getOutfit(id: string): Promise<Outfit> {
    return this.request(`/outfits/${id}`);
  }

  async createOutfit(outfit: CreateOutfitRequest): Promise<Outfit> {
    return this.request('/outfits', {
      method: 'POST',
      body: JSON.stringify(outfit),
    });
  }

  async updateOutfit(id: string, outfit: Partial<CreateOutfitRequest>): Promise<Outfit> {
    return this.request(`/outfits/${id}`, {
      method: 'PUT',
      body: JSON.stringify(outfit),
    });
  }

  async deleteOutfit(id: string): Promise<void> {
    return this.request(`/outfits/${id}`, {
      method: 'DELETE',
    });
  }

  // Auth API methods
  async login(credentials: LoginRequest): Promise<LoginResponse> {
    return this.request('/auth/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });
  }
}

export const apiClient = new ApiClient(API_BASE_URL);

// Export convenience functions
export const login = (credentials: LoginRequest) => apiClient.login(credentials);
export const getClothings = async (): Promise<Clothing[]> => {
  // Get token from localStorage
  const token = localStorage.getItem('token');
  if (token) {
    apiClient.setToken(token);
  }
  const response = await apiClient.getClothings();
  return response.clothings;
};
export const createClothing = async (clothing: CreateClothingRequest): Promise<Clothing> => {
  // Get token from localStorage
  const token = localStorage.getItem('token');
  if (token) {
    apiClient.setToken(token);
  }
  return apiClient.createClothing(clothing);
};
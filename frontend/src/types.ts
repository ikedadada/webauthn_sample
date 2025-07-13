export interface RegisterRequest {
  username: string;
}

export interface LoginRequest {
  username: string;
}

export interface ApiResponse<T = any> {
  success?: boolean;
  error?: string;
  data?: T;
}

export interface AuthResult {
  success: boolean;
  username?: string;
}
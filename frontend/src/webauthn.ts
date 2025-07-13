import { 
  startRegistration,
  startAuthentication
} from '@simplewebauthn/browser';

// Define types locally since they're not exported in this version
type RegistrationResponseJSON = any;
type AuthenticationResponseJSON = any;
type PublicKeyCredentialCreationOptionsJSON = any;
type PublicKeyCredentialRequestOptionsJSON = any;

import type { RegisterRequest, LoginRequest, ApiResponse, AuthResult } from './types';

const API_BASE_URL = window.location.origin;

class WebAuthnService {
  private async makeRequest<T>(url: string, data: any): Promise<T> {
    const response = await fetch(`${API_BASE_URL}${url}`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      credentials: 'include',
      body: JSON.stringify(data),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`HTTP ${response.status}: ${errorText}`);
    }

    return response.json();
  }

  async register(username: string): Promise<AuthResult> {
    try {
      // Step 1: Request registration challenge
      const registrationOptions = await this.makeRequest<PublicKeyCredentialCreationOptionsJSON>(
        '/register/request',
        { username } as RegisterRequest
      );

      // Step 2: Start registration with user's authenticator
      const registrationResponse: RegistrationResponseJSON = await startRegistration(registrationOptions);

      // Step 3: Send registration response to server for verification
      const result = await this.makeRequest<ApiResponse>(
        '/register/response',
        registrationResponse
      );

      return {
        success: true,
        username
      };
    } catch (error) {
      console.error('Registration failed:', error);
      throw new Error(error instanceof Error ? error.message : 'Registration failed');
    }
  }

  async authenticate(username: string): Promise<AuthResult> {
    try {
      // Step 1: Request authentication challenge
      const authenticationOptions = await this.makeRequest<PublicKeyCredentialRequestOptionsJSON>(
        '/login/request',
        { username } as LoginRequest
      );

      // Step 2: Start authentication with user's authenticator
      const authenticationResponse: AuthenticationResponseJSON = await startAuthentication(authenticationOptions);

      // Step 3: Send authentication response to server for verification
      const result = await this.makeRequest<AuthResult>(
        '/login/response',
        authenticationResponse
      );

      return result;
    } catch (error) {
      console.error('Authentication failed:', error);
      throw new Error(error instanceof Error ? error.message : 'Authentication failed');
    }
  }

  isSupported(): boolean {
    return typeof window !== 'undefined' && 
           typeof window.PublicKeyCredential !== 'undefined' &&
           typeof navigator.credentials !== 'undefined';
  }

  async isAvailable(): Promise<boolean> {
    if (!this.isSupported()) {
      return false;
    }

    try {
      return await window.PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable();
    } catch {
      return false;
    }
  }
}

export const webAuthnService = new WebAuthnService();
import { webAuthnService } from './webauthn';

class UIManager {
  private getElement<T extends HTMLElement>(id: string): T {
    const element = document.getElementById(id) as T;
    if (!element) {
      throw new Error(`Element with id '${id}' not found`);
    }
    return element;
  }

  private showResult(elementId: string, message: string, type: 'success' | 'error' | 'info'): void {
    const resultDiv = this.getElement(elementId);
    const className = type === 'success' ? 'success' : type === 'error' ? 'error' : 'info';
    const icon = type === 'success' ? '✅' : type === 'error' ? '❌' : 'ℹ️';
    
    resultDiv.innerHTML = `<div class="${className}">${icon} ${message}</div>`;
  }

  private setButtonLoading(button: HTMLButtonElement, loading: boolean, originalText?: string): void {
    if (loading) {
      button.setAttribute('data-original-text', button.textContent || '');
      button.textContent = '⏳ 処理中...';
      button.disabled = true;
      button.classList.add('loading');
    } else {
      button.textContent = originalText || button.getAttribute('data-original-text') || button.textContent;
      button.disabled = false;
      button.classList.remove('loading');
      button.removeAttribute('data-original-text');
    }
  }

  async initializeApp(): Promise<void> {
    // Check WebAuthn support
    if (!webAuthnService.isSupported()) {
      this.showResult('app-status', 'このブラウザはWebAuthnをサポートしていません', 'error');
      return;
    }

    const isAvailable = await webAuthnService.isAvailable();
    if (!isAvailable) {
      this.showResult('app-status', '生体認証が利用できません。デバイスの設定を確認してください。', 'error');
    } else {
      this.showResult('app-status', '生体認証が利用可能です', 'success');
    }

    this.setupEventListeners();
  }

  private setupEventListeners(): void {
    // Registration form
    const registerBtn = this.getElement<HTMLButtonElement>('register-btn');
    const registerUsernameInput = this.getElement<HTMLInputElement>('register-username');
    
    registerBtn.addEventListener('click', () => this.handleRegistration());
    registerUsernameInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        this.handleRegistration();
      }
    });

    // Login form
    const loginBtn = this.getElement<HTMLButtonElement>('login-btn');
    const loginUsernameInput = this.getElement<HTMLInputElement>('login-username');
    
    loginBtn.addEventListener('click', () => this.handleLogin());
    loginUsernameInput.addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        this.handleLogin();
      }
    });
  }

  private async handleRegistration(): Promise<void> {
    const usernameInput = this.getElement<HTMLInputElement>('register-username');
    const registerBtn = this.getElement<HTMLButtonElement>('register-btn');
    const username = usernameInput.value.trim();

    if (!username) {
      this.showResult('register-result', 'ユーザー名を入力してください', 'error');
      return;
    }

    try {
      this.setButtonLoading(registerBtn, true);
      this.showResult('register-result', '登録チャレンジを取得中...', 'info');

      const result = await webAuthnService.register(username);
      
      if (result.success) {
        this.showResult('register-result', '登録が完了しました！ログインをお試しください。', 'success');
        usernameInput.value = '';
      } else {
        this.showResult('register-result', '登録に失敗しました', 'error');
      }
    } catch (error) {
      console.error('Registration error:', error);
      this.showResult('register-result', `登録に失敗しました: ${error instanceof Error ? error.message : '不明なエラー'}`, 'error');
    } finally {
      this.setButtonLoading(registerBtn, false);
    }
  }

  private async handleLogin(): Promise<void> {
    const usernameInput = this.getElement<HTMLInputElement>('login-username');
    const loginBtn = this.getElement<HTMLButtonElement>('login-btn');
    const username = usernameInput.value.trim();

    if (!username) {
      this.showResult('login-result', 'ユーザー名を入力してください', 'error');
      return;
    }

    try {
      this.setButtonLoading(loginBtn, true);
      this.showResult('login-result', '認証チャレンジを取得中...', 'info');

      const result = await webAuthnService.authenticate(username);
      
      if (result.success) {
        this.showResult('login-result', `ログイン成功！ようこそ、${result.username}さん`, 'success');
        usernameInput.value = '';
      } else {
        this.showResult('login-result', 'ログインに失敗しました', 'error');
      }
    } catch (error) {
      console.error('Login error:', error);
      this.showResult('login-result', `ログインに失敗しました: ${error instanceof Error ? error.message : '不明なエラー'}`, 'error');
    } finally {
      this.setButtonLoading(loginBtn, false);
    }
  }
}

export const uiManager = new UIManager();
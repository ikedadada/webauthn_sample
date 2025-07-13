# WebAuthn 生体認証サンプルアプリケーション

TypeScript + Rust + MySQL を使用した WebAuthn（FIDO2）対応の生体認証システムです。

## 🚀 クイックスタート

```bash
# プロジェクトをクローン
git clone <repository-url>
cd webauthn_sample

# Docker Compose でアプリケーションを起動
docker-compose up --build

# ブラウザで http://localhost:3000 にアクセス
```

## 📋 機能

- **新規ユーザー登録**: 生体認証（指紋、顔認証等）での登録
- **ログイン**: 生体認証による安全なログイン
- **WebAuthn 準拠**: FIDO2 規格に準拠したセキュアな認証

## 🛠️ 技術スタック

- **フロントエンド**: TypeScript + Vite + HTML
- **バックエンド**: Rust (actix-web)
- **認証ライブラリ**: @simplewebauthn/browser, webauthn-rs
- **データベース**: MySQL 8.0
- **コンテナ**: Docker & Docker Compose

## 📁 プロジェクト構成

```
.
├── frontend/                # TypeScript フロントエンドアプリケーション
│   ├── src/
│   │   ├── index.ts        # メインエントリーポイント
│   │   ├── webauthn.ts     # WebAuthn 処理ロジック
│   │   ├── ui.ts          # UI 管理
│   │   ├── types.ts       # TypeScript型定義
│   │   └── styles.css     # スタイル
│   ├── index.html         # メインHTML
│   ├── package.json       # Node.js依存関係
│   ├── vite.config.ts     # Vite設定
│   ├── Dockerfile         # フロントエンド用Dockerfile
│   └── nginx.conf         # Nginx設定
├── backend/               # Rust サーバーアプリケーション
│   ├── src/
│   │   ├── main.rs       # メインエントリーポイント
│   │   ├── handlers.rs   # WebAuthn API ハンドラー
│   │   ├── models.rs     # データモデル
│   │   └── db.rs        # データベース操作
│   ├── Cargo.toml       # Rust依存関係
│   └── Dockerfile       # Rustアプリ用Dockerfile
├── db/
│   └── init.sql         # データベース初期化スクリプト
├── docker-compose.yml   # Docker Compose設定
└── README.md
```

## 🔧 開発環境での実行

### 前提条件

- Docker & Docker Compose
- HTTPS環境（本格運用時）

### 起動手順

1. **アプリケーション起動**
   ```bash
   docker-compose up --build
   ```

2. **ブラウザアクセス**
   - http://localhost:3000 にアクセス

3. **生体認証テスト**
   - ユーザー名を入力して「生体認証で登録」をクリック
   - ブラウザの生体認証プロンプトに従って認証
   - 登録後、同じユーザー名でログインをテスト

## 🗄️ データベーススキーマ

```sql
-- ユーザー情報
CREATE TABLE users (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  username VARCHAR(255) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 認証情報（公開鍵等）
CREATE TABLE credentials (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  credential_id VARBINARY(255) NOT NULL,
  public_key VARBINARY(512) NOT NULL,
  sign_count BIGINT NOT NULL DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## 🔒 セキュリティ要件

### 開発環境
- HTTP での動作確認が可能
- ローカルホスト環境での WebAuthn 使用

### 本番環境への適用時
- **HTTPS 必須**: WebAuthn は HTTPS 環境でのみ動作
- **適切なドメイン設定**: `rp_id` と `rp_origin` を本番ドメインに変更
- **セッション管理強化**: 本番用の安全なセッション管理実装
- **CORS 設定**: 適切な CORS ポリシーの設定

## 🚨 注意事項

- このサンプルは実験・学習目的です
- 本番環境での使用前にセキュリティレビューが必要です
- 生体情報はローカルデバイスでのみ管理され、サーバーには保存されません

## 📡 API エンドポイント

| エンドポイント | メソッド | 用途 |
|---------------|---------|------|
| `/` | GET | メインページ表示 |
| `/register/request` | POST | 登録チャレンジ取得 |
| `/register/response` | POST | 登録レスポンス検証 |
| `/login/request` | POST | 認証チャレンジ取得 |
| `/login/response` | POST | 認証レスポンス検証 |

## 🐛 トラブルシューティング

### よくある問題

1. **生体認証が利用できない**
   - デバイスに生体認証が設定されているか確認
   - ブラウザが WebAuthn をサポートしているか確認

2. **データベース接続エラー**
   ```bash
   docker-compose down
   docker-compose up --build
   ```

3. **ポート競合**
   - `docker-compose.yml` のポート設定を変更
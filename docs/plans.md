# プロジェクト計画書: HTMX + Rust + MySQL による WebAuthn 実験環境構築

## 🎯 目的

このプロジェクトは、WebAuthn（FIDO2）規格に準拠した生体認証機能（指紋認証・顔認証等）を Rust バックエンドと HTMX フロントエンドを用いて実装し、ユーザー登録〜ログインまでの最小構成を Docker 環境で迅速に試作・検証することを目的とする。

---

## 📐 技術スタック

| 層             | 技術               | 説明                                                            |
| -------------- | ------------------ | --------------------------------------------------------------- |
| フロントエンド | HTMX + HTML + JS   | JS は WebAuthn 呼び出しのみ。ページ更新・部分描画は HTMX で処理 |
| バックエンド   | Rust (`actix-web`) | WebAuthn 処理と API のエンドポイントを担当                      |
| 認証ロジック   | `webauthn-rs`      | FIDO2 準拠の WebAuthn 実装ライブラリ                            |
| データベース   | MySQL              | ユーザー情報・公開鍵・チャレンジ情報の保存に使用                |
| 環境構築       | Docker Compose     | 全コンポーネントを統一環境で起動・連携可能にする                |

---

## 📌 主な機能

### 🔐 ユーザー登録（生体認証登録）

- ユーザー名を入力し、WebAuthn 登録チャレンジをサーバーに要求
- `navigator.credentials.create()` を呼び出し、認証器で署名
- 登録レスポンスを Rust サーバーに送信し、公開鍵などを DB に保存

### 🔓 ログイン（生体認証による認証）

- 登録済みユーザー名を入力し、チャレンジを取得
- `navigator.credentials.get()` を呼び出し、認証器で署名
- サーバー側で署名を検証し、セッションを確立（または結果を返却）

---

## 🧩 API 仕様

| エンドポイント       | メソッド | 用途                     |
| -------------------- | -------- | ------------------------ |
| `/register/request`  | POST     | 登録チャレンジの取得     |
| `/register/response` | POST     | 認証器レスポンスの検証   |
| `/login/request`     | POST     | 認証チャレンジの取得     |
| `/login/response`    | POST     | 認証結果の検証とログイン |

---

## 🗃️ データベース構成

```sql
CREATE TABLE users (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  username VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE credentials (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  user_id BIGINT NOT NULL,
  credential_id VARBINARY(255) NOT NULL,
  public_key VARBINARY(512) NOT NULL,
  sign_count BIGINT NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id)
);
```

---

## 🐳 Docker 構成（概要）

```yaml
services:
  web:
    build: ./backend
    ports:
      - 8000:8000
    depends_on:
      - db

  db:
    image: mysql:8
    environment:
      MYSQL_ROOT_PASSWORD: password
      MYSQL_DATABASE: webauthn
    volumes:
      - ./db/init.sql:/docker-entrypoint-initdb.d/init.sql
```

---

## 📄 ディレクトリ構成（予定）

```
.
├── backend/                # Rust + Actix Web サーバー
│   ├── src/
│   ├── templates/          # HTMX用テンプレート
│   └── Dockerfile
├── db/
│   └── init.sql            # 初期スキーマ
├── docker-compose.yml
├── README.md
```

---

## 🚀 想定される開発ステップ

1. `webauthn-rs`を使って Rust で登録/認証ロジックを実装
2. HTMX + テンプレートエンジン（Askama 等）で最低限の UI を構築
3. JS で `navigator.credentials.create/get()` を呼び出す処理を組み込む
4. Docker で MySQL と Rust アプリを一体管理できるよう構成
5. 登録〜ログインまでの流れをローカルで検証

---

## ✅ 成果物

- WebAuthn 生体認証対応の最低限のユーザー登録・ログイン機能
- Docker で完結するローカル開発環境
- HTML ベース UI（JS 最小限）で WebAuthn の基本挙動を体験できるシステム

---

## 📝 備考

- 本プロジェクトは実験的なものであり、本番導入前には HTTPS やトークンベースのセッション管理などが必要です。
- 生体情報自体はローカル認証器（OS/ハードウェア）で管理され、サーバー側には保存されません。

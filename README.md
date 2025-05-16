# 🛡️ 0Byte Proof Backend (Rust)

The **0Byte backend** is a high-performance Rust service that anchors cryptographic proofs of AI-generated media to the **Solana blockchain**, and embeds that proof into the media file itself. It forms the backend of a broader protocol for verifiable media provenance.

---

## 📦 What It Does (Current MVP)

* Accepts base64-encoded images and metadata via REST API (`/generate-proof`)
* Computes perceptual hash (`pHash`) of the image
* Anchors a zero-knowledge mock proof + hash to **Solana Devnet**
* Embeds the Solana transaction ID + platform name into the image's metadata:

  * `tEXt` chunk for PNG
  * `COM` comment for JPEG
* Returns the modified image (with metadata) as binary in the API response

---

## 🔮 What It Will Become (Full Protocol Vision)

This backend will evolve into a full **proof layer for all AI-generated media**, supporting:

### ✅ Features in Development

* ⚙️ Real ZK Proofs via Circom or Noir
* 🎞️ Support for image sequences & video media
* 🧠 AI-generated fingerprinting & model attestation
* 🔐 Identity linking for creators
* ⛓️ Anchoring on multiple chains (Solana, Arweave, Filecoin)
* 💡 Trustless verification APIs for consumers & platforms

### 💡 Final Product Vision

> Any image or media generated via AI will carry a verifiable, tamper-resistant, on-chain fingerprint — cryptographically proving its origin, platform, and even model used — without exposing its contents.

---

## 🛠️ Tech Stack

| Component          | Tech                     |
| ------------------ | ------------------------ |
| Language           | Rust 🦀                  |
| Web Framework      | `actix-web`              |
| Image Processing   | `image` crate            |
| Metadata Embedding | `png`, `jpeg-decoder`    |
| Blockchain Client  | `solana-client` (Rust)   |
| ZK Proof (Mock)    | SHA256 placeholder       |
| Deployment         | Docker / Cloud Providers |

---

## 🚀 API: `/generate-proof`

### Request

`POST /generate-proof`

```json
{
  "image_bytes": "<base64-encoded image>",
  "model_name": "stable-diffusion-xl",
  "platform_name": "0byte",
  "input_token_count": 100,
  "output_token_count": 150
}
```

### Response

* **200 OK**

  * Returns **raw image bytes** with embedded metadata
  * Response header `X-Transaction-Id: <solana_txn_id>`

---

## 🧪 Running Locally

### Requirements

* Rust 1.70+
* Solana CLI configured for `devnet`
* `libjpeg` and `libpng` system libraries

### Build & Run

```bash
cargo build --release
./target/release/0byte-backend
```

By default, the server listens on `http://localhost:8000`.

---

## 🧩 Directory Structure

```
src/
├── main.rs            # Entry point (actix-web server)
├── handlers/          # API route handlers
│   └── proof.rs       # /generate-proof implementation
├── services/
│   ├── embedd.rs      # Metadata embedding logic
│   ├── phash.rs       # Perceptual hash calculator
│   └── zkp.rs         # Proof stub (mock or real)
├── solana/
│   └── anchor.rs      # Solana transaction handler
├── models/
│   └── proof.rs       # Request/response DTOs
├── config.rs          # App configuration loader
└── utils.rs           # Common utilities
```

---

## 🔐 Example Embedded Metadata (JPEG)

```
Comment: 0byte_txn:3UnCdYzzxX...XYZ|0byte
```

This allows any consumer to:

1. Extract the transaction ID + platform from the image
2. Verify the transaction on-chain via Solana explorers or RPC

---

## 🤝 Contributing

We welcome contributions! To propose a feature or fix:

1. Fork the repo
2. Create a new branch
3. Submit a pull request with a clear description

---

## 🪪 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 📬 Contact

Built with ❤️ by the 0Byte team — [GitHub](https://github.com/0byte-lab)

For questions or support, email: [nitinmewar28@gmail.com](mailto:nitinmewar28@gmail.com)

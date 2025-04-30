# ❌ No-as-a-Service Rust 🦀

<p align="center">
  <img src="https://raw.githubusercontent.com/ZAZPRO/no-as-a-service-rust/main/assets/imgs/image.png" width="800" alt="No-as-a-Service Banner"/>
</p>


Ever needed a graceful way to say “no”?  
This tiny API returns random, generic, creative, and sometimes hilarious rejection reasons — perfectly suited for any scenario: personal, professional, student life, dev life, or just because.

Built for humans, excuses, and humor.

---

## 🚀 API Usage

**Base URL**
```
https://naas.isalman.dev/no
```

**Method:** `GET`  
**Rate Limit:** `10 requests per minute per IP`

### 🔄 Example Request
```http
GET /no
```

### ✅ Example Response
```json
{
  "reason": "This feels like something Future Me would yell at Present Me for agreeing to."
}
```

Use it in apps, bots, landing pages, Slack integrations, rejection letters, or wherever you need a polite (or witty) no.

---

## 🛠️ Self-Hosting

Want to run it yourself? It’s lightweight and simple.

### 1. Clone this repository
```bash
git clone https://github.com/hotheadhacker/no-as-a-service.git
cd no-as-a-service
```

### 2. Run
```bash
cargo run --release
```

The API will be live at:
```
http://localhost:3000/no
```

You can also change the port and ip using an environment variable:
```bash
NOAAS_PORT=5000 NOAAS_IP=0.0.0.0 npm start
```

---

## 📁 Project Structure

```
└── no-as-a-service-rust
    ├── README.md
    ├── reasons.json # 200+ reasons
    └── src
        └── main.rs # Axum API
```

---

---

## 👤 Author

Created with creative stubbornness by [hotheadhacker](https://github.com/hotheadhacker)

Ported to rust by [ZAZPRO](https://github.com/ZAZPRO)

---

## 📄 License

MIT - as original project.

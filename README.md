# wp_xmlrpc_attck

`wp_xmlrpc_attck` is a **Rust-based asynchronous command-line tool** for testing **WordPress XML-RPC pingback SSRF behavior** in parallel.

It sends crafted `pingback.ping` requests to multiple WordPress targets and observes whether outbound requests are triggered toward a controlled callback endpoint.

> ⚠️ This tool is intended **only for defensive security testing and research** on systems you own or are explicitly authorized to test.

---

## 🔍 Purpose

WordPress's `xmlrpc.php` supports the `pingback.ping` method, which has historically been abused for:

- Server-Side Request Forgery (SSRF)
- Port scanning
- Reflection / amplification attacks

This tool helps security teams and researchers to:

- Identify exposed `xmlrpc.php` endpoints
- Test whether pingback functionality is enabled
- Validate SSRF protections
- Audit large numbers of WordPress sites efficiently

---

## ⚠️ Legal & Ethical Notice

- This tool **actively sends XML-RPC requests**
- Use **only on systems you own or have explicit permission to test**
- Unauthorized testing against third-party systems may be illegal
- The author assumes no responsibility for misuse

---

## ✨ Features

- Parallel XML-RPC scanning using Tokio async runtime
- Tests `pingback.ping` method specifically
- Configurable concurrency level
- Reads targets from file (1 URL per line)
- Custom callback (listener) and source URLs
- TLS certificate verification disabled (for testing environments)
- Clear per-target HTTP status output

---

## 📦 Requirements

- Rust (edition 2021 recommended)
- Cargo
- Tokio-compatible environment

### Key Dependencies

- `reqwest` (async)
- `tokio`
- `futures`
- `clap`

---

## 🚀 Installation

```bash
git clone https://github.com/your-org/wp_xmlrpc_attck.git
cd wp_xmlrpc_attck
cargo build --release
```

## the Binary will be Located
```
target/release/wp_xmlrpc_attck
```

## usage
```
wp_xmlrpc_attck --list <FILE> --callback <CALLBACK> --source <SOURCE> [OPTIONS]
```
## Target list (targets.txt)
```
https://example.com/xmlrpc.php
https://wp.example.org/xmlrpc.php
```
### Command
```
wp_xmlrpc_attck \
  --list targets.txt \
  --callback attacker.example.com \
  --source https://source.example.com \
  --concurrency 20
```
## Output
```
[*] Loaded 2 targets.
[*] Callback URL: attacker.example.com
[*] Source URL:   https://source.example.com
[!] [Suspicious!] SSRF Potentially Successful: https://example.com/xmlrpc.php
    -> Check your webhook
[+] All tests completed.
```

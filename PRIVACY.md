# Privacy Policy — Hart's Anti-Venom Desktop

**Effective Date:** November 2026

Hart's Anti-Venom operates under a strict
**Zero-Telemetry Architecture**. Your Web3 transaction
activity, address book entries, and cryptographic keys
remain exclusively on your local machine.

## 1. Zero Data Collection

We do not collect, transmit, store, log, or sell personal
information, IP addresses, hardware identifiers, browsing
history, keystrokes, or blockchain addresses.

## 2. Strict Local Sandbox Confinement

When installed via the Ubuntu App Center (Snap Store),
Hart's Anti-Venom runs inside a strictly confined AppArmor
and seccomp sandbox (`confinement: strict`). All middle-hex
address verification, Keccak-256 visual hash generation,
and RPC sanitization execute locally on your CPU.

## 3. Encrypted Local Storage

Custom counterparty labels and whitelisted addresses are
encrypted locally using AES-256-GCM authenticated
encryption backed by your system keyring (`libsecret`).
Plaintext keys never touch disk or external networks.

## 4. Official Portal & Contact

* **Website:** [hartsantivenom.com](https://hartsantivenom.com)
* **Online Privacy Policy:**
  [hartsantivenom.com/privacy](https://hartsantivenom.com/privacy)
* **Support Email:**
  [x401.tech@gmail.com](mailto:x401.tech@gmail.com)

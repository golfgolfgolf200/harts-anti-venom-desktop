# Hart's Anti-Venom Desktop Guard

Zero-telemetry, client-side desktop defense against EVM
address poisoning, zero-value ERC-20 transfer pollution,
and vanity collision traps—packaged for Ubuntu 24.04 /
26.04 LTS under strict Snap confinement.

## 5-Layer Defense Architecture

1. **Inbound Dust & Zero-Value Log Scrubbing:** Filters
   deceptive zero-value `Transfer` events and micro-dust
   logs before interface rendering.
2. **Outbound Signature & History Pairing:** Separates
   user-signed outbound counterparties from unsolicited
   inbound burner pollution.
3. **Anti-Vanity Visual Hashing & Middle-Hex Focus:**
   Dims easily spoofed outer hex boundaries, highlights
   center hex characters, and renders a deterministic
   5-block Keccak-256 color hash.
4. **Tamper-Evident Local Keystore:** Protects
   whitelisted counterparties using AES-256-GCM backed
   by Linux Secret Service (`libsecret`).
5. **Pre-Flight Nonce & Topology Oracle:** Flags
   zero-nonce EOAs sharing boundary characters with
   known contacts.

## Security & Confinement Model

* **Strict Snap Confinement (`confinement: strict`):**
  Enforced by Linux kernel AppArmor and seccomp profiles.
* **Zero Host Dotfile Access:** Completely isolated from
  `~/.ssh`, `~/.gnupg`, and unrelated host directories.
* **Wayland Native:** Prevents cross-window X11 clipboard
  eavesdropping.

## Legal & Links

* **License:** [Apache 2.0](./LICENSE)
* **Privacy Policy:** [PRIVACY.md](./PRIVACY.md) |
  [hartsantivenom.com/privacy](https://hartsantivenom.com/privacy)
* **Terms of Service:** [TERMS.md](./TERMS.md)
* **Official Portal:** [hartsantivenom.com](https://hartsantivenom.com)
* **Support:** [x401.tech@gmail.com](mailto:x401.tech@gmail.com)

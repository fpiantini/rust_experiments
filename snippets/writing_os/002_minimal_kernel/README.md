# Minimal Rust Kernel

https://os.phil-opp.com/minimal-rust-kernel/

## Versioni dei compilatori Rust

Rust ha tre release channels: _stable_, _beta_ e _nightly_.

Per installare rust su una macchina host il metodo consigliato è utilizzare `rustup`. Se all'interno di un progetto si vuole usare una versione specifica, per esempio _nightly_ si possono fare due cose:

- dare il comando: `rustup override set nightly`
- Creare un file `rust-toolchain` con contenuto uguale alla versione che si vuole usare, p.e. `nightly`

In questa directory si usa per esempio questo secondo metodo.

## Comando di build

```
cargo build --target x86_64-mikru.json
```

Per evitare di passare il target continuamente, è possibile specificarlo in `.cargo/config.toml` (vedere linea `target = `).


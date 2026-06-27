# Soluzioni esami di Rust

Qui raccolgo le mie soluzioni agli appelli delle varie sessioni d'esame di Rust.
Ogni cartella corrisponde a un appello (la data è nel nome) ed è strutturata sempre allo stesso modo:

- **`src/`** → la soluzione della parte di **programmazione** (il codice vero e proprio).
- **`exercise.md`** → la risoluzione della parte di **teoria** dell'appello.

Il tutto è organizzato come un unico Cargo workspace, quindi puoi compilare/testare il singolo appello entrando nella sua cartella oppure tutto insieme dalla radice con `cargo test`.

## Appelli presenti

- `_2022-06-20_looper`
- `_2022-07-08_execution-limiter`
- `_2022-09-08_dispatcher`
- `_2023-01_barrier`
- `_2023-06-20_mpmc`
- `_2023-07-07_delayed-queue`
- `_2023-09-04_cache`
- `_2024-01-22_multi-channel`
- `_2024-06-25_exchanger`
- `_2025-06-17_token_manager`
- `_2025-07-03_aggregatore_di_misure`
- `_2025-07_count_down_lock`
- `_2026-01-xx_cache_lru`
- `_2026-06-15_forgettable_cometuttolappello`

## ⚠️ Nota su `forgettable`

Nell'appello `_2026-06-15_forgettable` ho aggiunto un vincolo `Clone` sul tipo `T` della funzione
`forgettable_channel` (quella che crea la coppia sender/receiver), per poter clonare i messaggi
internamente.

Le specifiche dicevano di **non toccare le firme dei tratti**, Sulle *altre* funzioni (come `forgettable_channel`) il testo non diceva nulla, quindi mi sono preso una piccola libertà rispetto al testo originale.
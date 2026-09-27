# Contributing to NØR

NØR is an experimental Solana-native protocol. Contributions should prioritize deterministic behavior, explicit account constraints, and clear security documentation.

## Before opening a pull request

1. Explain the protocol or developer problem being solved.
2. Add or update Anchor tests for instruction behavior and failure cases.
3. Document account layout changes and migration implications.
4. Run `anchor test` locally.
5. Do not include secret keys, wallet files, generated build artifacts, or unreviewed dependency updates.

## Scope

NØR is inspired by NEAR's developer experience but is an independent implementation for Solana. Avoid copying code or trademarks from external projects. Changes that affect fees, ownership, expiry, authority, or program IDs require maintainer review.

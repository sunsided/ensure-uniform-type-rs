---
applyTo: "**/*.md,docs/**"
---

# Documentation

- README files are rendered on crates.io and docs.rs. Every link to a repo file and every
  image uses an absolute URL (`https://github.com/...`, `https://raw.githubusercontent.com/...`).
  Only in-page anchors may be relative.
- Code blocks are tagged with their language. Untagged or `rust` blocks in included Markdown
  run as doctests, so they must compile. Use `rust,no_run` or `rust,ignore` only with a
  stated reason.
- Keep badges, the License section and the MSRV statement consistent with `Cargo.toml`.
- `README.md` is the crate documentation (`#![doc = include_str!("../README.md")]`), so its
  Rust blocks are doctests. The failing example stays `rust,compile_fail`.

# rescript-rs changelog

## 0.1.0 (fork)

rescript-rs is a fork of [ts-rs](https://github.com/Aleph-Alpha/ts-rs) v12, adapted to generate ReScript types instead of TypeScript.

### Changes from ts-rs
- Derive macro renamed from `TS` to `ReScript` (`TS` still available as alias)
- Attributes use `#[rescript(...)]` instead of `#[ts(...)]`
- Output files use `.res` extension instead of `.ts`
- Generated types use ReScript syntax (records, variants) instead of TypeScript
- Type mappings: `i32`/`u32` -> `int`, `f64` -> `float`, `String` -> `string`, `bool` -> `bool`
- Enum variants generate ReScript variant types
- Serde `tag`/`content` support generates `@tag` annotations for ReScript tagged variants
- Support for `repr(enum)` with integer discriminants via `@as(N)`

# Zcash quest tools — DeSaint007 / the_grand_puppeteer

Independent developer quest implementations with AI assistance. No real funds.

## Developer verification
```sh
python3 verification.py
```
One execution prints the participant name, current UTC timestamp, and a fresh
128-bit cryptographically random value. Git history includes three initial
commits plus a feature branch merged with an explicit merge commit.

## Build and run
Requires Rust 1.85.1 or later (tested using 1.95.0).
```sh
cargo build --locked
./target/debug/zaddrcheck '<address>'
./target/debug/zpayreq '<address>' '1.25'
./target/debug/zpayreq '<address>' '1.25' --shielded
cargo test --locked --all-targets
```
`zaddrcheck` emits exactly one JSON object with `input`, `valid`, and `type`.
Missing/empty input returns `valid: false` without crashing. Types supported
are transparent P2PKH/P2SH, Sapling and Unified addresses on supported networks.
It validates full address encodings through the official `zcash_address` crate,
including their checksums and Unified Address decoding, before classification.
It does not check whether a destination exists, is controlled by anyone, or
whether shielded receiver bytes correspond to spendable cryptographic keys.
Sprout and TEX are outside this quest's output contract and return invalid.

`zpayreq` emits a single-recipient ZIP-321 `zcash:<address>?amount=<decimal>` URI.
It constructs a request only; it never sends funds. `--shielded` rejects
transparent addresses. Amounts use integer zatoshis, up to eight decimal places,
positive and at most 21 million ZEC. Exponent notation and non-finite values
are rejected; trailing decimal zeroes are normalized.

| Error code | Meaning |
| --- | --- |
| ERR_USAGE | Wrong argument count |
| ERR_INVALID_ADDRESS | Unsupported or malformed address/checksum |
| ERR_SHIELDED_REQUIRED | Transparent-only recipient with --shielded |
| ERR_AMOUNT_FORMAT | Not an unsigned fixed-point decimal |
| ERR_AMOUNT_NON_POSITIVE | Negative or zero amount |
| ERR_AMOUNT_PRECISION | More than eight fractional digits |
| ERR_AMOUNT_RANGE | Overflow or above 21 million ZEC |

Errors are JSON on stderr with a nonzero exit code; success is on stdout.
Validation order is address, shielded policy, then amount.

## Tests and CI
CI runs formatting, Clippy and all unit/integration tests. Tests exercise real
transparent, Sapling and Unified vectors, checksum failures, empty input, all
amount failure classes, shielded-policy rejection, and the URI happy path.
Vectors come from official zcash_address docs and zcash_client_backend tests.
No quest-provided workflow or hidden suite was available on the visible quest
page; this repository's green CI is not a claim of passing an unseen grader.

References: https://docs.rs/zcash_address/0.13.0/zcash_address/ and
https://zips.z.cash/zip-0321 .

The feature branch changes this README and is merged into main for the Git challenge.

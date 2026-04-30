# PixelAnalyzer WASM Core
## Technical Specification and Architecture Document (DETA)

**Version:** 2.2  
**Technologies:** Rust (v1.85+), WebAssembly, CIELAB / CIELCh  
**Status:** Final Specification — Ready for Implementation

---

## 1. Engineering Rules
The following rules are the canonical engineering standards for this project.

### 1.1 Principles & Architecture
- **Priority**: KISS → YAGNI → DRY → SOLID → Clean Code → Performance (only when measurable).
- Build only what’s needed today; future-proof by writing replaceable code. Abstract only after 3+ duplications.
- Prefer explicit configuration and well-documented defaults over hidden behavior.
- Layered design: domain → use-cases → integration → interface.
- Favor composition over inheritance; depend on abstractions/traits at module boundaries.

### 1.2 Code Quality & Formatting
- **Zero internal comments**: No inline, block, or line comments (`//`, `/* */`). Clarity must come from naming and structure. 
  - Exception: Public API documentation (`///`, `/** */`) is required for public-facing functions/types.
- No file headers describing file name or purpose. No `TODO`, `FIXME`, or commented-out code.
- **Naming**: Explain intent. No abbreviations, no generic suffixes like `data`/`info`/`util`.
- **Structure**: Max 300 lines per file. Max 40 lines per function. Max 4 parameters per function.
- No stringly-typed interfaces; use newtypes, enums, or discriminated unions.
- No boolean control parameters — use enums or separate functions.
- **Errors**: No `unwrap()`, `expect()`, or `panic!()` in production library code. Use typed errors (`Result`, `thiserror`). Never silently swallow errors.

### 1.3 Separation of Concerns
- I/O and side effects live at the edge — domain stays pure, stateless, and testable.
- Domain logic must not depend on infrastructure (DB drivers, HTTP clients).
- Parsing, validation, and mapping are distinct steps. Validate all external inputs at system boundaries.

### 1.4 Documentation & Changelog
- **Single Source of Truth**: Documentation lives in `README.md`, `CHANGELOG.md`, and `docs/`. Mismatches between documentation and code (e.g., type signatures) are treated as critical bugs.
- **No Aspirational Docs**: Document only what exists and is tested today.
- **No Version Duplication**: Version numbers belong ONLY in `Cargo.toml`/`package.json` and `CHANGELOG.md`. Never hardcode versions in README or other doc files.
- **Prose for Why, Tables for What**: Use prose for rationale/policy and tables for API contracts, options, and error taxonomies.
- **Synchronisation is Mandatory**: Code changes that affect public APIs, types, or build targets must include documentation updates in the same commit.

### 1.5 Testing
- Every public function must have ≥1 happy path test and ≥1 edge case test.
- Naming format: `[function]When[condition]Then[outcome]` or describe behavior (e.g., `returnsErrorWhenInputEmpty`).
- No test-only logic in production code paths.

### 1.6 Dependencies
- Justify every dependency. Prefer zero-dependency solutions for trivial needs.
- Pin exact versions in applications; use semver ranges for libraries.
- No "maybe later" dependencies; remove unused dependencies immediately.

### 1.7 Security
- Never commit secrets, tokens, or credentials. Use `.env` or secrets managers.
- Validate and sanitize all external inputs. Escape all outputs based on context.
- Never leak stack traces, internal paths, or DB schemas to clients.
- `unsafe` blocks in Rust require a `SAFETY:` doc comment on the same block.

### 1.8 Stack-Specific Conventions
- **Rust**: Idiomatic ownership, `thiserror`/`anyhow` for errors, traits at boundaries.
- **Node/TS**: Async/await only, ESM imports, Valibot over Zod, specific error classes.
- **Python**: Type hints everywhere, `dataclass` or Pydantic.

### 1.9 AI Agent Behavior
- **Responses**: Ultra-terse. Cut explanations unless needed for correctness. No fluff. Use diff format for changes.
- **Planning**: Execute trivial tasks immediately. For complex tasks (3+ steps), explicitly state the plan before executing.
- **CI/CD**: After every change, run format, lint, and tests. All must pass with zero warnings.

### 1.10 Changelog Convention
The changelog is a human-readable history of decisions, not a diff.

### 1.11 README Rules
The README must be current, tested, and concise.

### 1.12 Repo Documentation Rules (`docs/`)
Rules defined in Section 12 of the core engineering rules are applied here.

---

## 2. Architecture

### 2.1 Module Map
| Module | Responsibility |
|---|---|
| `lib.rs` | WASM entry point, lifecycle management (`init`, `analyze`, `terminate`) |
| `decoder.rs` | Format detection and image decoding (PNG, JPEG, WebP) |
| `sampler.rs` | Adaptive downsampling and stratified spatial sampling |
| `color.rs` | Perceptual space transformations (sRGB → XYZ → Lab → LCh) |
| `kmeans.rs` | K-Means++ clustering engine with ΔE convergence |
| `report.rs` | Assembly of the final `AnalysisReport` |
| `accessibility.rs` | WCAG 2.1 contrast ratio evaluation |
| `metrics.rs` | Hasler-Suesstrunk colorfulness and Shannon entropy |
| `color_theory.rs` | Hue group classification and harmony generation |

### 2.2 Data Flow
`Buffer validation` → `format detection (decoder.rs)`
  → `pixel decode + RGBA expansion`
  → `spatial downsample (32×32 stratified grid)`
  → `K-Means++ clustering (ΔE convergence)`
  → `palette assembly + accent selection`
  → `WCAG contrast + font color recommendation`
  → `report serialisation` → `JsValue`

---

## 3. Type Contracts

### `ColorEntry`
| Rust field | Rust type | TypeScript type | Nullable |
|---|---|---|---|
| `hex` | `String` | `string` | No |
| `population` | `f32` | `number` | No |
| `is_dark` | `bool` | `boolean` | No |
| `lab` | `LabColor` | `interface { l: number; a: number; b: number }` | No |
| `lch` | `LchColor` | `interface { l: number; c: number; h: number }` | No |

### `MainPalette`
| Rust field | Rust type | TypeScript type | Nullable |
|---|---|---|---|
| `dominant` | `ColorEntry` | `ColorEntry` | No |
| `accent` | `Option<ColorEntry>` | `ColorEntry \| null` | Yes |
| `background_suggestion` | `String` | `string` | No |
| `foreground_suggestion` | `String` | `string` | No |

---

## 4. Doc-Comment Policy
- All public items (`pub fn`, `pub struct`, `pub enum`, all variants and fields) require a `///` doc-comment.
- Doc-comments describe the item's contract: what it accepts, what it returns, what it cannot handle.
- Doc-comments must not describe implementation details. Implementation is the code. The comment is the contract.
- `//` and `/* */` are banned in `src/` per Rule 2.

---

## 5. Binary Footprint Targets

| Metric | Target | Current Status | CI Gate |
|---|---|---|---|
| Optimised WASM (`*_bg.wasm`) | < 500 KB | ~480 KB | Hard exit-code-1 |
| Development WASM (no `wasm-opt`) | No gate | ~1.2 MB | Informational only |

---

## 6. Project Identity

### 6.1 Mission
> To provide a zero-dependency, memory-safe WASM core that extracts perceptually accurate chromatic intelligence and accessibility metrics from images in any JavaScript environment.

### 6.2 Vision
> To become the definitive standard for browser-side visual analysis by merging academic color science with high-performance systems engineering.

---

## 7. Mathematical Foundations
(Refer to previous versions of DETA.md for exhaustive ΔE and K-Means++ formulas)

---

## 8. Bibliography
1. CIE (1976). *Colorimetry*, Publication 15.2. Bureau Central de la CIE, Vienna.
2. Arthur, D., & Vassilvitskii, S. (2007). "k-means++: The advantages of careful seeding". *Proc. ACM-SIAM SODA*.
3. W3C (2018). *Web Content Accessibility Guidelines (WCAG) 2.1*. W3C Recommendation.
4. Hasler, D., & Suesstrunk, S. E. (2003). "Measuring colorfulness in natural images". *IS&T/SPIE Electronic Imaging*.
5. Fitzgerald, A. et al. (2019–2025). *wasm-bindgen*. Rust and WebAssembly Working Group.

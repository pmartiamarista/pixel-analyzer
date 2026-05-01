# Changelog — pixel-analyzer Demo

## [0.0.2]

### Added
- [feat]: implement comprehensive WCAG 2.1 accessibility matrix (AA/AAA × Normal/Large text)
- [feat]: implement live legibility preview using dominant/accent color combinations
- [feat]: add descriptive footnotes for WCAG threshold definitions and Large Text criteria
- [docs]: create `docs/FRONTEND.md` covering application architecture and WCAG panel logic

### Accessibility
- [a11y]: implement `role="list/listitem"` and dynamic `aria-label` values for the WCAG compliance matrix
- [a11y]: implement `aria-label` and `aria-hidden="false"` for the legibility preview container

## [0.0.1]

### Added
- [feat]: implement multi-input image loading: file upload, HTTPS URL fetch, and clipboard paste
- [feat]: implement native drag-and-drop on the file upload panel
- [feat]: implement full-UI lock during WASM initialization and analysis — all inputs and tabs disabled
- [feat]: implement analysis loading state with centered spinner and `role="status"` announcement
- [feat]: implement palette display — vibrant and muted grids with click-to-copy hex swatches
- [feat]: implement keyboard-accessible color swatches (`role="button"`, `tabindex="0"`, Enter/Space handlers)
- [feat]: implement color theory chips (`inline-color-group`) with click and keyboard copy support
- [feat]: implement image stats panel — brightness, colorfulness, hue group, pixel count
- [feat]: implement accessibility panel — contrast ratio, WCAG AA status, recommended font color
- [feat]: implement color theory panel — complementary, triadic, analogous harmonies

### Fixed
- [fix]: replace `FileReader` DataURL approach with `URL.createObjectURL` for zero-copy image previews
- [fix]: implement `isValidImageUrl` guard to enforce HTTPS-only fetches and prevent SSRF
- [fix]: route all status messages through single `#global-status` element for cross-tab visibility
- [fix]: set `aria-invalid="true"` and `aria-describedby` on URL input on validation failure

### Infrastructure
- [infra]: Vite 8 build with `esbuild` peer dependency for explicit minification
- [infra]: COOP/COEP headers in dev server and preview server for WASM SharedArrayBuffer compatibility
- [infra]: `build.target: es2022`, `assetsInlineLimit: 0` to prevent WASM base64 inlining
- [infra]: GitHub Pages deployment via CI on push to `main`

### Accessibility (WCAG 2.2 AA)
- [a11y]: skip link (`Skip to main content`) per 2.4.1 Bypass Blocks
- [a11y]: `role="tablist/tab/tabpanel"` with `aria-selected`, `aria-controls`, `aria-labelledby`
- [a11y]: `aria-busy` toggled on analyze button during processing per 4.1.2
- [a11y]: `loader.focus()` on analysis start for correct focus order per 2.4.3
- [a11y]: `3px solid` focus outlines on all interactive elements per 2.4.13 Focus Appearance
- [a11y]: `scroll-margin-top` on all elements per 2.4.11 Focus Not Obscured
- [a11y]: `min-height: 36px` on tabs, `44px` on URL input per 2.5.8 Target Size
- [a11y]: status message `✕`/`✓` prefix icons — no color-only distinction per 1.4.1
- [a11y]: `--text-muted` lightened to `#7a7a96`, border opacity raised to 14% per 1.4.3/1.4.11
- [a11y]: `@media (prefers-reduced-motion: reduce)` collapses all animations/transitions
- [a11y]: upload zone `[aria-disabled]` visual state during UI lock

### Design
- [style]: dark theme — `#0a0a0c` background, `#6366f1` accent, Outfit + Inter typography
- [style]: responsive layout — single column mobile, 640px tablet, 1024px desktop with sticky sidebar
- [style]: enumerate composited-only CSS transitions (`transform`, `opacity`, `border-color`, etc.)
- [style]: `clamp(2rem, 8vw, 3.2rem)` fluid h1 with gradient fill

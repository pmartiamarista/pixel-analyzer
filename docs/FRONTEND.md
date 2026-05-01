# Frontend Documentation — pixel-analyzer Demo

This document provides a reference for the `pixel-analyzer` demo application.

## Overview
The demo is a high-performance, accessible web application built to showcase the perceptual color intelligence of the `pixel-analyzer` WASM library.

### Technology Stack
- **Framework**: [Vite 8](https://vitejs.dev/) (Vanilla JS mode)
- **Logic**: Vanilla ES2022 JavaScript
- **Styling**: Vanilla CSS with Design Tokens
- **Core**: Rust/WebAssembly (`pixel-analyzer`)

### Running Locally
1. Navigate to the `demo` directory: `cd demo`
2. Install dependencies: `npm install`
3. Start dev server: `npm run dev`

## Architecture
The application logic is centralized in `main.js`, following a linear lifecycle:

1. **Bootstrap**: Initializes the WASM module via `init()`.
2. **Event Listeners**: Sets up handlers for file uploads, URL fetching, and clipboard pasting.
3. **Analysis Pipeline**:
   - Captures image bytes as a `Uint8Array`.
   - Calls the `analyze()` function with user-defined `AnalysisOptions`.
4. **Rendering**:
   - `renderReport(report)`: Orchestrates the display of all metrics.
   - `renderAccessibility(report)`: Specifically handles the WCAG 2.1 matrix and legibility preview.

## Input Modes
- **File**: Supports drag-and-drop or browsing for JPEG, PNG, and WebP files.
- **URL**: Fetches images from HTTPS URLs (Note: CORS must be allowed by the remote server).
- **Paste**: Allows direct pasting of image data from the clipboard.

## WCAG Accessibility Panel
The accessibility panel provides a detailed evaluation of color contrast between the **Dominant** and **Accent** colors of the image.

### Compliance Matrix
The frontend calculates four distinct compliance levels based on the `contrast_ratio`:
- **AA Normal**: Threshold 4.5:1
- **AA Large**: Threshold 3.0:1
- **AAA Normal**: Threshold 7.0:1
- **AAA Large**: Threshold 4.5:1

### Legibility Preview
A live preview box demonstrates how text looks on the dominant background.
- **Background**: Dominant color.
- **Foreground**: Accent color (fallback to Black/White safe choice if no accent is found).

## Changelog
For a history of changes to the demo application, see [demo/CHANGELOG.md](../demo/CHANGELOG.md).

# Round 15 Plan: Operation QA

## Overview
This round focuses on **testing, validation, and bug fixing** for the `messenger-desktop` Tauri 2.x app. The goal is to:
- Fix all compilation errors.
- Expand thin files to meet target line counts.
- Add missing functionality.
- Validate the app's core features.

---

## Current State
### Workspace Configuration
- **Status**: Fixed. `projects/messenger-desktop/src-tauri` is already in the `workspace.members` array.

### Dependency Issues
- **Issue**: `specta` crate version `^2.0` is not available. Must use `2.0.0-rc.22`.
- **Fix**: Update `Cargo.toml` in `src-tauri` to specify `specta = { version = "2.0.0-rc.22" }`.

### File Structure
| File Path | Lines | Status |
|-----------|-------|--------|
| `src-tauri/src/main.rs` | 10 | **Too thin** |
| `src-tauri/src/lib.rs` | 170 | OK |
| `src-tauri/src/window_manager.rs` | 614 | OK |
| `src-tauri/src/notifications.rs` | 548 | OK |
| `src-tauri/src/platform/*.rs` | 48-77 | **Thin** |
| `src/injection/*.ts` | 16-96 | **Too thin** |
| `src/settings/settings.ts` | 191 | OK |

### Compilation Errors
- **Dependency resolution**: `specta` crate version mismatch.
- **Rust errors**: Not yet visible due to dependency issue.

---

## Wave Structure
### Wave 1: Dependency Fix
- **Agent**: `devstral` (cloud)
- **Task**: Update `src-tauri/Cargo.toml` to specify `specta = { version = "2.0.0-rc.22" }`.
- **Output**: Updated `src-tauri/Cargo.toml`.
- **Validation**: Run `cargo check` successfully.

### Wave 2: Rust Compilation Fixes
- **Agent**: `coder` (cloud)
- **Task**: Fix all Rust compilation errors in `src-tauri/src/*.rs`.
- **Output**: Compilation succeeds with no errors.
- **Validation**: Run `cargo check` and `cargo build`.

### Wave 3: Expand Thin Files
- **Agents**: `qwen35`, `mistral`, `devstral` (cloud, parallel)
- **Tasks**:
  - Expand `src/injection/*.ts` files to 100-200 lines.
  - Expand `src-tauri/src/platform/*.rs` files to 100+ lines.
  - Expand `src-tauri/src/main.rs` to 50+ lines.
- **Output**: Updated files with additional functionality.
- **Validation**: Manual review and line count check.

### Wave 4: Add Missing Functionality
- **Agents**: `coder`, `research` (cloud, parallel)
- **Tasks**:
  - Implement missing features in `src-tauri/src/*.rs` (e.g., `accounts.rs`, `media.rs`).
  - Add unit tests for critical components.
  - Validate Tauri commands and IPC communication.
- **Output**: Updated files with new functionality and tests.
- **Validation**: Run `cargo test` and manual testing.

### Wave 5: Final Validation
- **Agent**: `devstral` (cloud)
- **Task**:
  - Run `cargo check`, `cargo build`, and `cargo test`.
  - Validate Tauri app startup and core features.
  - Generate a summary report.
- **Output**: Summary of fixes, test results, and final status.

---

## Definition of Done
1. **Dependencies**: Fixed and `cargo check` runs successfully.
2. **Compilation**: No errors in `cargo check` or `cargo build`.
3. **File Structure**: All files meet target line counts (100-200 lines for thin files).
4. **Functionality**: Core features implemented and tested.
5. **Tests**: Unit tests added for critical components.
6. **Report**: Summary posted to #swarm-ops with:
   - What was fixed.
   - Test results.
   - Final `cargo check` status.
   - Lines added.
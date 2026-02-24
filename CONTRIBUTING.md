# Contributing

Thank you for considering contributing to Messenger Desktop! Here’s how you can help:

## Development Setup

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Node.js 18+](https://nodejs.org/)
- Linux: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`, `patchelf`

### Steps

1. Fork the repository and clone your fork:
   ```sh
   git clone https://github.com/your-username/messenger-desktop.git
   cd messenger-desktop
   ```

2. Install dependencies:
   ```sh
   npm install
   ```

3. Run in development mode:
   ```sh
   npm run tauri dev
   ```

4. Build for production:
   ```sh
   npm run tauri build
   ```

## Code Style

- **Rust**: Use `rustfmt` for formatting.
  ```sh
  cargo fmt
  ```

- **TypeScript**: Use `prettier` for formatting.
  ```sh
  npx prettier --write src/
  ```

## Pull Request Process

1. Create a new branch for your feature or bugfix:
   ```sh
   git checkout -b feature/your-feature-name
   ```

2. Commit your changes with a descriptive message:
   ```sh
   git commit -m "feat: add new theme support"
   ```

3. Push your branch to your fork:
   ```sh
   git push origin feature/your-feature-name
   ```

4. Open a Pull Request (PR) against the `main` branch of the upstream repository.

5. Wait for review and address any feedback.

## Adding a New Feature

### Example: Adding a New Theme

1. Create a new theme file in `src/themes/` (e.g., `src/themes/dracula.ts`).
2. Define the theme colors and styles:
   ```ts
   export const dracula = {
     background: "#282a36",
     text: "#f8f8f2",
     // ...
   };
   ```
3. Register the theme in `src/themes/index.ts`.
4. Test the theme locally:
   ```sh
   npm run tauri dev
   ```

## Reporting Issues

- Use the [GitHub Issues](https://github.com/example/messenger-desktop/issues) page.
- Include steps to reproduce, screenshots, and logs if applicable.

## License

By contributing, you agree that your contributions will be licensed under the **MIT License**.
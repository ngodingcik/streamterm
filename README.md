<a id="readme-top"></a>

<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![License][license-shield]][license-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h3 align="center">streamterm</h3>

  <p align="center">
    Real-time display capture rendered as braille ASCII art directly in your terminal, with 24-bit color.
    <br />
    <a href="https://github.com/NgodingCik/streamterm/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/NgodingCik/streamterm/issues/new?labels=enhancement&template=feature-request---.md">Request Feature</a>
  </p>
</div>

<!-- ABOUT THE PROJECT -->
## About The Project

streamterm captures your primary display in real time and streams it into your terminal as Unicode braille characters with full 24-bit color. Every terminal cell covers a 2x4 pixel block, mapped to one of the 256 braille characters in the U+2800–U+28FF range, giving an effective resolution of twice the terminal width by four times its height.

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

## Built With

* [![Rust][Rust-badge]][Rust-url]
* [![Rayon][Rayon-badge]][Rayon-url]
* [![Crossterm][Crossterm-badge]][Crossterm-url]
* [![Scrap][Scrap-badge]][Scrap-url]

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

To run streamterm locally, follow the steps below.

### Prerequisites

* **Rust 1.80+**
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source ~/.cargo/env
  ```

* **[Linux only]** - X11 headers required by `scrap`:
  ```sh
  sudo apt install libx11-dev libxext-dev libxcb1-dev
  ```

  > [!NOTE]
  > macOS and Windows ship the required native APIs, no extra system packages needed.

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/NgodingCik/streamterm.git
   ```
2. Move into the project directory:
   ```sh
   cd streamterm
   ```
3. Build the release binary:
   ```sh
   cargo build --release
   ```
4. Run it:
   ```sh
   # Linux / macOS
   ./target/release/streamterm

   # Windows
   target\release\streamterm.exe
   ```

> [!TIP]
> Maximize your terminal and use a small font size before launching, the larger the terminal grid, the higher the effective render resolution.

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- HOW IT WORKS -->
## How It Works

Each frame goes through four stages:

1. **Capture**: `scrap` grabs the primary display as a raw BGRA byte buffer.
2. **Scale**: nearest-neighbor lookup tables map every braille sub-pixel to a source coordinate without division inside the inner loop.
3. **Render**: rayon processes rows in parallel; each cell samples 8 sub-pixels, computes BT.601 luminance, classifies pixels into foreground/background, picks the matching braille character, and writes ANSI 24-bit color escapes only when the color actually changes.
4. **Output**: the resulting byte buffer is flushed to stdout in one shot via `BufWriter`.

Press `Ctrl+C` to exit cleanly.

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions of any kind are welcome.

1. Fork the repository
2. Create an isolated branch for your change (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'feat: add YourFeature'`)
4. Push to your fork (`git push origin feature/YourFeature`)
5. Open a Pull Request

> [!IMPORTANT]
> Run `cargo clippy -- -D warnings` and `cargo test` before opening a Pull Request. PRs that introduce new warnings will not be merged.

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the GNU General Public License v3.0. See `LICENSE` for details.

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- CONTACT -->
## Contact

Project link: [https://github.com/NgodingCik/streamterm](https://github.com/NgodingCik/streamterm)
Discord server: [https://discord.ngodingcik.my.id](https://discord.ngodingcik.my.id)

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [scrap](https://github.com/quadrupleslap/scrap): cross-platform screen capture
* [rayon](https://github.com/rayon-rs/rayon): data parallelism
* [crossterm](https://github.com/crossterm-rs/crossterm): cross-platform terminal control
* [Unicode Braille Patterns](https://www.unicode.org/charts/PDF/U2800.pdf): U+2800-U+28FF

<p align="right">(<a href="#readme-top">Back to top</a>)</p>

<!-- MARKDOWN LINKS & BADGES -->
[contributors-shield]: https://img.shields.io/github/contributors/NgodingCik/streamterm.svg?style=for-the-badge
[contributors-url]: https://github.com/NgodingCik/streamterm/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/NgodingCik/streamterm.svg?style=for-the-badge
[forks-url]: https://github.com/NgodingCik/streamterm/network/members
[stars-shield]: https://img.shields.io/github/stars/NgodingCik/streamterm.svg?style=for-the-badge
[stars-url]: https://github.com/NgodingCik/streamterm/stargazers
[issues-shield]: https://img.shields.io/github/issues/NgodingCik/streamterm.svg?style=for-the-badge
[issues-url]: https://github.com/NgodingCik/streamterm/issues
[license-shield]: https://img.shields.io/github/license/NgodingCik/streamterm.svg?style=for-the-badge
[license-url]: https://github.com/NgodingCik/streamterm/blob/main/LICENSE

[Rust-badge]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white
[Rust-url]: https://www.rust-lang.org/
[Rayon-badge]: https://img.shields.io/badge/Rayon-000000?style=for-the-badge&logo=rust&logoColor=white
[Rayon-url]: https://github.com/rayon-rs/rayon
[Crossterm-badge]: https://img.shields.io/badge/Crossterm-000000?style=for-the-badge&logo=rust&logoColor=white
[Crossterm-url]: https://github.com/crossterm-rs/crossterm
[Scrap-badge]: https://img.shields.io/badge/Scrap-000000?style=for-the-badge&logo=rust&logoColor=white
[Scrap-url]: https://github.com/quadrupleslap/scrap
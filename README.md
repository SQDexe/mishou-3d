# Mishou 3D

![Application's logo](/assets/icon_160x160.png)

### 見小
Stylised kanji writing - neologism for "small view"

#### Mishou 3D is a high-performance rendering application built with Rust. It leverages the raw power of the Vulkan API for graphics processing and provides a seamless, immediate-mode graphical user interface for interacting with the environment. Designed for speed and simplicity, it allows users to load, view, and inspect 3D models with real-time configuration tweaking.

## Features

* **Vulkan-Powered Rendering:** Utilises modern hardware acceleration for fast and efficient 3D rendering.
* **Real-Time Customisation:** An on-screen GUI allows immediate tweaking of background colours, model tints, and global lighting direction.
* **Advanced Camera System:** Free-fly spatial camera with adjustable speed, sensitivity, field of view, and dynamic clipping planes.
* **Rendering Controls:** Ability to switch between solid polygon rendering and wireframe views, or select various framerate modes.
* **Robust CLI:** Highly configurable command-line interface for initialising the application state straight from the terminal.

## Installation, Building, and Running

Ensure you have [Rust and Cargo](https://rust-lang.org/) installed on your system, alongside a Vulkan-compatible graphics driver.

```bash
git clone https://github.com/SQDexe/mishou-3d.git
cd mishou-3d
cargo build --release
cargo run --release
```

The whole application can be run both from console, and as a standalone executable.

When using console, check out the `--help` flag for more details.

Check the `About` window under `Help` tab for instructions on moving through the editor space.

## Prominent Dependencies

- **[`vulkano`](https://vulkano.rs/)**: Safe, Rusty wrapper for the Vulkan graphics API.
- **[`winit`](https://docs.rs/winit/latest/winit/)**: Cross-platform window creation and event loop handling.
- **[`egui`](https://docs.rs/egui/0.31.1/egui/)**: Highly responsive immediate-mode GUI framework.
- **[`clap`](https://docs.rs/clap/latest/clap/)**: Command-line argument parsing.
- **[`glam`](https://docs.rs/glam/latest/glam/)**: Fast and lightweight 3-D math library.
- **and many other useful crates** ...

## License

This project is licensed under the `GNU General Public License v3.0`.

## Sources & Acknowledgements
 
- [Vulkan API documentation](https://docs.vulkan.org/refpages/latest/refpages/)
- [Vulkano tutorial](https://vulkano.rs/)
- [Vulkanalia tutorial](https://kylemayes.github.io/vulkanalia/introduction.html)
- [wgpu tutorial](https://sotrh.github.io/learn-wgpu/)
- [egui site](https://www.egui.rs/)
- Rust-doc comments, and parts of this file were generated using `Gemini Pro 3.1`

### Logo inspirations

- https://www.magnific.com/pl/premium-wektory/brown-bear-pixel-art-slodkie-zwierzeta-dla-aktywow-gry-w-ilustracjach-wektorowych_88525873.htm
- https://www.magnific.com/premium-vector/brown-bear-head-8-bit-pixel-art-cute-animal-game-assets-vector-illustration_379956591.htm
- https://logo-icons.com/products/bear-logo-design-4
- https://scalebranding.com/product/181599
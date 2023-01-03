# FOX Engine

A fast, extensive game engine written in Rust.

Example of a game made with FOX Engine
### Features
* Fast and asynchronous ECS design, with a dedicated ECS section for internal rendering
* OpenGL-based rendering, with support for both 2D and 3D operations
* Uses the glium crate for rendering
* Minimal codebase with minimal dependencies

## Fast and asynchronous ECS design

FOX Engine uses an Entity-Component-System (ECS) architecture to manage game objects and their behavior. The ECS design of FOX Engine is optimized for speed and asynchrony, allowing your game to run smoothly and efficiently.
## OpenGL-based rendering

FOX Engine uses OpenGL as the underlying graphics library, allowing it to support both 2D and 3D rendering. The engine includes an extensive vertex system that supports a wide range of rendering operations.
## glium crate for rendering

FOX Engine uses the `glium` crate for rendering, which provides a safe and convenient wrapper around OpenGL. This allows you to take advantage of the power of OpenGL while still writing safe Rust code.
## Minimal codebase with minimal dependencies

FOX Engine has a minimal codebase and few dependencies, making it easy to integrate into your project and reducing the risk of conflicts with other crates. This also means that the engine has a small footprint and can be compiled and run quickly.
## Getting Started

To use FOX Engine in your own project, you can add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
fox_engine = "0.1.0"
```

# Documentation

You can find more detailed documentation for FOX Engine on the documentation website.
# Contributions

We welcome contributions to FOX Engine! If you have an idea for a new feature or have found a bug, please open an issue on GitHub. If you would like to work on implementing a new feature or fixing a bug yourself, please fork the repository and open a pull request with your changes.
# License

FOX Engine is licensed under the MIT License. See LICENSE for more details.
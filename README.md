# Coat

A pure Rust GUI library mainly inspired by Conrod, React, Elm and Flutter.

In the `concepts` folder are examples of what I imagine this library to be like.

# Goals

- Keep things as simple as possible without abstraction.
- Be cross platform (Linux, Mac, Windows, Android and maybe others).
- Intermediate mode style, like React.
- *Everything* will be Rust (or macros) - no css and other text file parsing.
- You *can* use 'rsx' (like jsx) or 'rml' (like qml) with macros or just Rust.
- Easy to use builtin futures to keep things responsive.
- Never crash, only complain (no panics except you do something really dumb).
- Builtin back end (Webrender), nothing 'agnostic' that wont work well in the end.
- Try to stick with this project until it can be used productive (this will be the hardest part).

# State
It can open a window. And close it. While using a lot of RAM.

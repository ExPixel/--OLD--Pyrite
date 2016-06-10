# rust-imgui [![Build Status](https://travis-ci.org/ExPixel/rust-imgui.svg?branch=master)](https://travis-ci.org/ExPixel/rust-imgui)

Thin Rust wrapper for ImGui

Use `cargo run --example imgui_demo` to run the demo.  

This library can be used just like imgui for the most part. Just change pascal case to camel case.  
e.g. `ImGui::GetVersion()` -> `imgui::get_version()`

Example Usage _(not including setup)_:  
```rust
#[macro_use] extern crate rust_imgui;
use rust_imgui as imgui;

pub fn imgui_example_draw() {
	let mut opened = true;
	imgui::begin(imstr!("Example Window"), &mut opened, imgui::ImGuiWindowFlags::None);
	imgui::text(imstr!("Hello World"));
	imgui::end();
}
```

If you're using opengl, feel free to just copy the code in the example in `examples/imgui_demo.rs`. 
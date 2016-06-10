extern crate gcc;

pub fn main() {
	gcc::Config::new()
		.cpp(true)
		.file("cimgui/imgui/imgui.cpp")
		.file("cimgui/imgui/imgui_draw.cpp")
		.file("cimgui/imgui/imgui_demo.cpp")
		.file("cimgui/cimgui/cimgui.cpp")
		.file("cimgui/cimgui/fontAtlas.cpp")
		.file("cimgui/cimgui/drawList.cpp")
		.compile("libimgui.a");
}
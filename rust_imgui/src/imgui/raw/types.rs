use libc::c_float;
use libc::c_char;
use libc::c_uchar;
use libc::c_int;
use libc::c_uint;
use libc::c_short;
use libc::c_ushort;
use libc::c_void;
use libc::size_t;

use std::ops::{Index, IndexMut};

#[allow(non_camel_case_types)]
pub type c_string = *const c_char;

#[allow(non_camel_case_types)]
pub type c_bool = c_char; // #TODO ARM >:(

pub type ImU32 = c_uint;
pub type ImWchar = c_ushort;
pub type ImTextureID = *mut c_void;
pub type ImGuiID = ImU32;
pub type ImDrawIdx = c_ushort;
// pub type ImGuiCol = c_int;
// pub type ImGuiStyleVar = c_int;
// pub type ImGuiKey_ = c_int;
// pub type ImGuiAlign_ = c_int;
// pub type ImGuiColorEditMode = c_int;
// pub type ImGuiMouseCursor = c_int;
// pub type ImGuiWindowFlags_ = c_int;
// pub type ImGuiSetCond = c_int;
// pub type ImGuiInputTextFlags_ = c_int;
// pub type ImGuiSelectableFlags = c_int;
// pub type ImGuiTreeNodeFlags = c_int;

pub type ImGuiTextEditCallback = Option<extern "C" fn(data: *mut ImGuiTextEditCallbackData)>;
pub type ImGuiSizeConstraintCallback = Option<extern "C" fn(data: *mut ImGuiSizeConstraintCallbackData)>;
pub type ImDrawCallback = Option<extern "C" fn(parent_list: *const ImDrawList, cmd: *const ImDrawCmd)>;


#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImVec2 {
	pub x: c_float,
	pub y: c_float,
}

impl ImVec2 {
	pub fn zero() -> ImVec2 {
		ImVec2::new(0.0, 0.0)
	}

	pub fn new(x: f32, y: f32) -> ImVec2 {
		ImVec2 {
			x: x as c_float,
			y: y as c_float
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImVec4 {
	pub x: c_float,
	pub y: c_float,
	pub z: c_float,
	pub w: c_float
}

impl ImVec4 {
	pub fn zero() -> ImVec4 {
		ImVec4::new(0.0, 0.0, 0.0, 0.0)
	}

	pub fn new(x: f32, y: f32, z: f32, w: f32) -> ImVec4 {
		ImVec4 {
			x: x as c_float,
			y: y as c_float,
			z: z as c_float,
			w: w as c_float
		}
	}
}

#[repr(C)]
pub struct ImGuiStyle {
	pub alpha: c_float,
	pub window_padding: ImVec2,
	pub window_min_size: ImVec2,
	pub window_rounding: c_float,
	pub window_title_align: ImGuiAlign_::Type,
	pub child_window_rounding: c_float,
	pub frame_padding: ImVec2,
	pub frame_rounding: c_float,
	pub item_spacing: ImVec2,
	pub item_inner_spacing: ImVec2,
	pub touch_extra_padding: ImVec2,
	pub indent_spacing: c_float,
	pub columns_min_spacing: c_float,
	pub scrollbar_size: c_float,
	pub scrollbar_rounding: c_float,
	pub grab_min_size: c_float,
	pub grab_rounding: c_float,
	pub display_window_padding: ImVec2,
	pub display_safe_area_padding: ImVec2,
	pub anti_aliased_lines: c_bool,
	pub anti_aliased_shapes: c_bool,
	pub curve_tesselation_tol: c_float,
	pub colors: [ImVec4; ImGuiCol_::COUNT as usize]

	// IMGUI_API ImGuiStyle();
}

#[repr(C)]
pub struct ImGuiIO {
	// Settings:
	pub display_size: ImVec2,
	pub delta_time: c_float,
	pub ini_saving_rate: c_float,
	pub ini_filename: c_string,
	pub log_filename: c_string,
	pub mouse_double_click_time: c_float,
	pub mouse_double_click_max_dist: c_float,
	pub mouse_drag_threshold: c_float,
	pub key_map: [c_int; ImGuiKey_::COUNT as usize],
	pub key_repeat_delay: c_float,
	pub key_repeat_rate: c_float,
	pub user_data: *mut c_void,

	pub fonts: &'static mut ImFontAtlas,
	pub float_global_scale: c_float,
	pub font_allow_user_scaling: c_bool,
	pub display_framebuffer_scale: ImVec2,
	pub display_visible_min: ImVec2,
	pub display_visible_max: ImVec2,

	// Advanced/subtle behaviors
	pub word_movement_uses_alt_key: c_bool,
	pub shortcut_uses_super_key: c_bool,
	pub double_click_selects_word: c_bool,
	pub multiselect_uses_super_key: c_bool,

	// User Functions
	pub render_draw_list_fn: extern "C" fn(data: *const ImDrawData),
	pub get_clipboard_text_fn: extern "C" fn() -> c_string,
	pub set_clipboard_text_fn: extern "C" fn(text: c_string),

	pub mem_alloc_fn: extern "C" fn(sz: size_t) -> *mut c_void,
	pub mem_free_fn: extern "C" fn(ptr: *mut c_void),

	pub ime_set_input_screen_pos_fn: extern "C" fn(x: c_int, y: c_int),
	pub im_window_handle: *mut c_void,

	// Input - Fill before calling NewFrame()
	pub mouse_pos: ImVec2,
	pub mouse_down: [c_bool; 5],
	pub mouse_wheel: c_float,
	pub mouse_draw_cursor: c_bool,
	pub key_ctrl: c_bool,
	pub key_shift: c_bool,
	pub key_alt: c_bool,
	pub key_super: c_bool,
	pub keys_down: [c_bool; 512],
	pub input_characters: [ImWchar; 16 + 1],

	// // Functions
	// IMGUI_API void AddInputCharacter(ImWchar c);                        // Helper to add a new character into InputCharacters[]
	// IMGUI_API void AddInputCharactersUTF8(const char* utf8_chars);      // Helper to add new characters into InputCharacters[] from an UTF-8 string
	// inline void    ClearInputCharacters() { InputCharacters[0] = 0; }   // Helper to clear the text input buffer

	// Output - Retrieve after calling NewFrame(), you can use them to discard inputs or hide them from the rest of your application
	pub want_capture_mouse: c_bool,
	pub want_capture_keyboard: c_bool,
	pub want_text_input: c_bool,
	pub framerate: c_float,
	pub metrics_allocs: c_int,
	pub metrics_render_vertices: c_int,
	pub metrics_render_indices: c_int,
	pub metrics_active_windows: c_int,

	// [Internal] ImGui will maintain those fields for you
	pub mouse_pos_prev: ImVec2,
	pub mouse_delta: ImVec2,
	pub mouse_clicked: [c_bool; 5],
	pub mouse_clicked_pos: [ImVec2; 5],
	pub mouse_clicked_time: [c_float; 5],
	pub mouse_double_clicked: [c_bool; 5],
	pub mouse_released: [c_bool; 5],
	pub mouse_down_owned: [c_bool; 5],
	pub mouse_down_duration: [c_float; 5],
	pub mouse_down_duration_prev: [c_float; 5],
	pub mouse_drag_max_distance_sqr: [c_float; 5],
	pub keys_down_duration: [c_float; 512],
	pub keys_down_duration_prev: [c_float; 512],

	// IMGUI_API   ImGuiIO();
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiWindowFlags_ {
	use libc::c_int;
	pub type Type = c_int;

	// Default: 0
	pub const NoTitleBar: c_int             = 1 << 0;
	pub const NoResize: c_int               = 1 << 1;
	pub const NoMove: c_int                 = 1 << 2;
	pub const NoScrollbar: c_int            = 1 << 3;
	pub const NoScrollWithMouse: c_int      = 1 << 4;
	pub const NoCollapse: c_int             = 1 << 5;
	pub const AlwaysAutoResize: c_int       = 1 << 6;
	pub const ShowBorders: c_int            = 1 << 7;
	pub const NoSavedSettings: c_int        = 1 << 8;
	pub const NoInputs: c_int               = 1 << 9;
	pub const MenuBar: c_int                = 1 << 10;
	pub const HorizontalScrollbar: c_int    = 1 << 11;
	pub const NoFocusOnAppearing: c_int     = 1 << 12;
	pub const NoBringToFrontOnFocus: c_int  = 1 << 13;
	pub const AlwaysVerticalScrollbar: c_int= 1 << 14;
	pub const AlwaysHorizontalScrollbar: c_int=1<< 15;
	pub const AlwaysUseWindowPadding: c_int = 1 << 16;
	
	// [Internal]
	pub const ChildWindow: c_int            = 1 << 20;
	pub const ChildWindowAutoFitX: c_int    = 1 << 21;
	pub const ChildWindowAutoFitY: c_int    = 1 << 22;
	pub const ComboBox: c_int               = 1 << 23;
	pub const Tooltip: c_int                = 1 << 24;
	pub const Popup: c_int                  = 1 << 25;
	pub const Modal: c_int                  = 1 << 26;
	pub const ChildMenu: c_int              = 1 << 27;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiInputTextFlags_ {
	use libc::c_int;
	pub type Type = c_int;

	// Default: 0
	pub const CharsDecimal: c_int        = 1 << 0;
	pub const CharsHexadecimal: c_int    = 1 << 1;
	pub const CharsUppercase: c_int      = 1 << 2;
	pub const CharsNoBlank: c_int        = 1 << 3;
	pub const AutoSelectAll: c_int       = 1 << 4;
	pub const EnterReturnsTrue: c_int    = 1 << 5;
	pub const CallbackCompletion: c_int  = 1 << 6;
	pub const CallbackHistory: c_int     = 1 << 7;
	pub const CallbackAlways: c_int      = 1 << 8;
	pub const CallbackCharFilter: c_int  = 1 << 9;
	pub const AllowTabInput: c_int       = 1 << 10;
	pub const CtrlEnterForNewLine: c_int = 1 << 11;
	pub const NoHorizontalScroll: c_int  = 1 << 12;
	pub const AlwaysInsertMode: c_int    = 1 << 13;
	pub const ReadOnly: c_int            = 1 << 14;
	pub const Password: c_int            = 1 << 15;
	// [Internal]
	pub const Multiline: c_int           = 1 << 20;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiTreeNodeFlags_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Selected: c_int             = 1 << 0;
	pub const Framed: c_int               = 1 << 1;
	pub const AllowOverlapMode: c_int     = 1 << 2;
	pub const NoTreePushOnOpen: c_int     = 1 << 3;
	pub const NoAutoOpenOnLog: c_int      = 1 << 4;
	pub const DefaultOpen: c_int          = 1 << 5;
	pub const OpenOnDoubleClick: c_int    = 1 << 6;
	pub const OpenOnArrow: c_int          = 1 << 7;
	pub const Leaf: c_int                 = 1 << 8;
	pub const Bullet: c_int               = 1 << 9;
	pub const SpanAllAvailWidth: c_int    = 1 << 10;
	pub const NoScrollOnOpen: c_int       = 1 << 11;
	// pub const SpanAllAvailWidth: c_int  = 1 << 10;
	// pub const NoScrollOnOpen: c_int     = 1 << 11;
	pub const CollapsingHeader: c_int     = Framed | NoAutoOpenOnLog;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiSelectableFlags_ {
	use libc::c_int;
	pub type Type = c_int;

	// Default: 0
	pub const DontClosePopups: c_int    = 1 << 0;
	pub const SpanAllColumns: c_int     = 1 << 1;
	pub const AllowDoubleClick: c_int   = 1 << 2;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiKey_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Tab: c_int = 0;
	pub const LeftArrow: c_int = Tab + 1;
	pub const RightArrow: c_int = LeftArrow + 1;
	pub const UpArrow: c_int = RightArrow + 1;
	pub const DownArrow: c_int = UpArrow + 1;
	pub const PageUp: c_int = DownArrow + 1;
	pub const PageDown: c_int = PageUp + 1;
	pub const Home: c_int = PageDown + 1;
	pub const End: c_int = Home + 1;
	pub const Delete: c_int = End + 1;
	pub const Backspace: c_int = Delete + 1;
	pub const Enter: c_int = Backspace + 1;
	pub const Escape: c_int = Enter + 1;
	pub const A: c_int = Escape + 1;
	pub const C: c_int = A + 1;
	pub const V: c_int = C + 1;
	pub const X: c_int = V + 1;
	pub const Y: c_int = X + 1;
	pub const Z: c_int = Y + 1;
	pub const COUNT: c_int = Z + 1;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiCol_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Text: c_int = 0;
	pub const TextDisabled: c_int = Text + 1;
	pub const WindowBg: c_int = TextDisabled + 1;
	pub const ChildWindowBg: c_int = WindowBg + 1;
	pub const PopupBg: c_int = ChildWindowBg + 1;
	pub const Border: c_int = PopupBg + 1;
	pub const BorderShadow: c_int = Border + 1;
	pub const FrameBg: c_int = BorderShadow + 1;
	pub const FrameBgHovered: c_int = FrameBg + 1;
	pub const FrameBgActive: c_int = FrameBgHovered + 1;
	pub const TitleBg: c_int = FrameBgActive + 1;
	pub const TitleBgCollapsed: c_int = TitleBg + 1;
	pub const TitleBgActive: c_int = TitleBgCollapsed + 1;
	pub const MenuBarBg: c_int = TitleBgActive + 1;
	pub const ScrollbarBg: c_int = MenuBarBg + 1;
	pub const ScrollbarGrab: c_int = ScrollbarBg + 1;
	pub const ScrollbarGrabHovered: c_int = ScrollbarGrab + 1;
	pub const ScrollbarGrabActive: c_int = ScrollbarGrabHovered + 1;
	pub const ComboBg: c_int = ScrollbarGrabActive + 1;
	pub const CheckMark: c_int = ComboBg + 1;
	pub const SliderGrab: c_int = CheckMark + 1;
	pub const SliderGrabActive: c_int = SliderGrab + 1;
	pub const Button: c_int = SliderGrabActive + 1;
	pub const ButtonHovered: c_int = Button + 1;
	pub const ButtonActive: c_int = ButtonHovered + 1;
	pub const Header: c_int = ButtonActive + 1;
	pub const HeaderHovered: c_int = Header + 1;
	pub const HeaderActive: c_int = HeaderHovered + 1;
	pub const Column: c_int = HeaderActive + 1;
	pub const ColumnHovered: c_int = Column + 1;
	pub const ColumnActive: c_int = ColumnHovered + 1;
	pub const ResizeGrip: c_int = ColumnActive + 1;
	pub const ResizeGripHovered: c_int = ResizeGrip + 1;
	pub const ResizeGripActive: c_int = ResizeGripHovered + 1;
	pub const CloseButton: c_int = ResizeGripActive + 1;
	pub const CloseButtonHovered: c_int = CloseButton + 1;
	pub const CloseButtonActive: c_int = CloseButtonHovered + 1;
	pub const PlotLines: c_int = CloseButtonActive + 1;
	pub const PlotLinesHovered: c_int = PlotLines + 1;
	pub const PlotHistogram: c_int = PlotLinesHovered + 1;
	pub const PlotHistogramHovered: c_int = PlotHistogram + 1;
	pub const TextSelectedBg: c_int = PlotHistogramHovered + 1;
	pub const ModalWindowDarkening: c_int = TextSelectedBg + 1;
	pub const COUNT: c_int = ModalWindowDarkening + 1;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiStyleVar_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Alpha: c_int = 0;
	pub const WindowPadding: c_int = Alpha + 1;
	pub const WindowRounding: c_int = WindowPadding + 1;
	pub const WindowMinSize: c_int = WindowRounding + 1;
	pub const ChildWindowRounding: c_int = WindowMinSize + 1;
	pub const FramePadding: c_int = ChildWindowRounding + 1;
	pub const FrameRounding: c_int = FramePadding + 1;
	pub const ItemSpacing: c_int = FrameRounding + 1;
	pub const ItemInnerSpacing: c_int = ItemSpacing + 1;
	pub const IndentSpacing: c_int = ItemInnerSpacing + 1;
	pub const GrabMinSize: c_int = IndentSpacing + 1;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiAlign_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Left: c_int     = 1 << 0;
	pub const Center: c_int   = 1 << 1;
	pub const Right: c_int    = 1 << 2;
	pub const Top: c_int      = 1 << 3;
	pub const VCenter: c_int  = 1 << 4;
	pub const Default: c_int  = Left | Top;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiColorEditMode_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const UserSelect: c_int = -2;
	pub const UserSelectShowButton: c_int = -1;
	pub const RGB: c_int = 0;
	pub const HSV: c_int = 1;
	pub const HEX: c_int = 2;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiMouseCursor_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Arrow: c_int = 0;
	pub const TextInput: c_int = Arrow + 1;
	pub const Move: c_int = TextInput + 1;
	pub const ResizeNS: c_int = Move + 1;
	pub const ResizeEW: c_int = ResizeNS + 1;
	pub const ResizeNESW: c_int = ResizeEW + 1;
	pub const ResizeNWSE: c_int = ResizeNESW + 1;
	pub const COUNT: c_int = ResizeNWSE + 1;
}

#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
pub mod ImGuiSetCond_ {
	use libc::c_int;
	pub type Type = c_int;

	pub const Always: c_int        = 1 << 0;
	pub const Once: c_int          = 1 << 1;
	pub const FirstUseEver: c_int  = 1 << 2;
	pub const Appearing: c_int     = 1 << 3;
}

// #NOTE ImGuiOnceUponAFrame
// #NOTE ImGuiTextFilter
// #NOTE ImGuiTextBuffer
// #NOTE ImGuiStorage

#[repr(C)]
pub struct ImGuiOnceUponAFrame {
	pub ref_frame: c_int
}

#[repr(C)]
pub struct ImGuiTextFilter {
	pub input_buf: [c_char; 256],
	pub filters: ImVector<ImGuiTextFilterTextRange>,
	pub count_grep: c_int
}

#[repr(C)]
pub struct ImGuiTextFilterTextRange {
	pub b: c_string,
	pub e: c_string
}

#[repr(C)]
pub struct ImGuiTextBuffer {
	pub buf: ImVector<c_char>
}

#[repr(C)]
pub struct ImGuiStorage {
	pub data: ImVector<ImGuiStoragePair>
}

#[repr(C)]
pub struct ImGuiStoragePair {
	pub key: ImGuiID,

	// union { int val_i; float val_f; void* val_p; };
	pub val: *const c_void,
}


#[repr(C)]
pub struct ImGuiTextEditCallbackData {
	pub event_flag: ImGuiInputTextFlags_::Type,
	pub flags: ImGuiInputTextFlags_::Type,
	pub user_data: *mut c_void,
	pub read_only: c_bool,

	// CharFilter event:
	pub event_char: ImWchar,

	// Completion,History,Always events:
	pub event_key: ImGuiKey_::Type,
	pub buf: *mut c_char,
	pub buf_text_len: c_int,
	pub buf_size: c_int,
	pub buf_dirty: c_bool,
	pub cursor_pos: c_int,
	pub selection_start: c_int,
	pub selection_end: c_int,
}

#[repr(C)]
pub struct ImGuiSizeConstraintCallbackData {
	pub user_data: *mut c_void,
	pub pos: ImVec2,
	pub current_size: ImVec2,
	pub desired_size: ImVec2,
}

#[repr(C)]
pub struct ImColor {
	value: ImVec4,
}

impl ImColor {
	pub fn new() -> ImColor { Self::from_rgba(0.0, 0.0, 0.0, 0.0) }

	pub fn from_rgba8(r: u8, g: u8, b: u8, a: u8) -> ImColor {
		let sc = 1.0 / 255.0;
		Self::from_rgba(
			(r as f32) / sc,
			(g as f32) / sc,
			(b as f32) / sc,
			(a as f32) / sc
		)
	}

	pub fn from_rgba32(rgba: u32) -> ImColor {
		let sc = 1.0 / 255.0;
		Self::from_rgba(
			((rgba & 0xFF) as f32) / sc,
			(((rgba >> 8) & 0xFF) as f32) / sc,
			(((rgba >> 16) & 0xFF) as f32) / sc,
			(((rgba >> 24) & 0xFF) as f32) / sc,
		)
	}

	pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> ImColor {
		ImColor {
			value: ImVec4::new(r, g, b, a)
		}
	}

	pub fn from_rgb8(r: u8, g: u8, b: u8) -> ImColor { Self::from_rgba8(r, g, b, 255) }
	pub fn from_rgb(r: f32, g: f32, b: f32) -> ImColor { Self::from_rgba(r, g, b, 1.0) }

	pub fn from_color(col: &ImColor) -> ImColor {
		Self::from_rgba(
			col.value.x,
			col.value.y,
			col.value.z,
			col.value.w
		)
	}
}

#[repr(C)]
pub struct ImGuiListClipper {
	pub start_pos_y: c_float,
	pub item_height: c_float,
	pub item_count: c_int,
	pub step_no: c_int,
	pub display_start: c_int,
	pub display_end: c_int,
}

#[inline]
pub fn im_col32<C: Into<u32>>(r: C, g: C, b: C, a: C) -> u32 {
	(a.into() << 24) | (b.into() << 16) | (g.into() << 8) | r.into()
}

pub const IM_COL32_WHITE: u32 = 0xFFFFFFFF;
pub const IM_COL32_BLACK: u32 = 0xFF000000;
pub const IM_COL32_BLACK_TRANS: u32 = 0x00000000;

#[repr(C)]
pub struct ImDrawCmd {
	pub elem_count: c_uint,
	pub clip_rect: ImVec4,
	pub texture_id: ImTextureID,
	pub user_callback: ImDrawCallback,
	pub user_callback_data: *mut c_void,
} // #TODO implement this constructor?

#[repr(C)]
pub struct ImDrawVert {
	pub pos: ImVec2,
	pub uv: ImVec2,
	pub col: ImU32
}

#[repr(C)]
pub struct ImDrawChannel {
	pub cmd_buffer: ImVector<ImDrawCmd>,
	pub idx_buffer: ImVector<ImDrawIdx>
}

#[repr(C)]
pub struct ImDrawList {
	pub cmd_buffer: ImVector<ImDrawCmd>,
	pub idx_buffer: ImVector<ImDrawIdx>,
	pub vtx_buffer: ImVector<ImDrawVert>,

	pub _owner_name: c_string,
	pub _vtx_current_idx: c_uint,
	pub _vtx_write_ptr: *mut ImDrawVert,
	pub _idx_write_ptr: *mut ImDrawIdx,
	pub _clip_rect_stack: ImVector<ImVec4>,
	pub _texture_id_stack: ImVector<ImTextureID>,
	pub _path: ImVector<ImVec2>,
	pub _channels_current: c_int,
	pub _channels_count: c_int,
	pub _channels: ImVector<ImDrawChannel>,
}

#[repr(C)]
pub struct ImDrawData {
	pub valid: c_bool,
	pub cmd_lists: *mut *mut ImDrawList,
	pub cmd_lists_count: c_int,
	pub total_vtx_count: c_int,
	pub total_idx_count: c_int,
}

#[repr(C)]
pub struct ImFontConfig {
	pub font_data: *mut c_void,
	pub font_data_size: c_int,
	pub font_data_owned_by_atlas: c_bool,
	pub font_no: c_int,
	pub size_pixels: c_float,
	pub oversample_h: c_int,
	pub oversample_v: c_int,
	pub pixel_snap_h: c_bool,
	pub glyph_extra_spacing: ImVec2,
	pub glyph_ranges: *const ImWchar,
	pub merge_mode: c_bool,
	pub merge_glyph_center_v: c_bool,

	// [Internal]
	pub name: [c_char; 32],
	pub dst_font: *mut ImFont,
}

#[repr(C)]
pub struct ImFontAtlas {
	pub tex_id: *mut c_void,
	pub tex_pixels_alpha8: &'static mut c_uchar,
	pub tex_pixels_rgba32: &'static mut c_uint,
	pub tex_width: c_int,
	pub tex_height: c_int,
	pub tex_desired_width: c_int,
	pub tex_uv_white_pixel: ImVec2,
	pub fonts: ImVector<*mut ImFont>,

	pub config_data: ImVector<ImFontConfig>,
}

#[repr(C)]
pub struct ImFont {
	pub font_size: c_float,
	pub scale: c_float,
	pub display_offset: ImVec2,
	pub glyphs: ImVector<ImFontGlyph>,
	pub index_x_advance: ImVector<c_float>,
	pub index_lookup: ImVector<c_short>,
	pub fallback_glyph: *const ImFontGlyph,
	pub fallback_x_advance: c_float,
	pub fallback_char: ImWchar,

	pub config_data_count: c_short,
	pub config_data: &'static mut ImFontConfig,
	pub container_atlas: &'static mut ImFontAtlas,
	pub ascent: c_float,
	pub descent: c_float
}

#[repr(C)]
pub struct ImFontGlyph {
	pub code_point: ImWchar,
	pub x_advance: c_float,
	pub x0: c_float,
	pub y0: c_float,
	pub x1: c_float,
	pub y1: c_float,
}

#[repr(C)]
pub struct ImVector<T> {
	pub size: c_int,
	pub capacity: c_int,
	pub data: *mut T,
}

impl<T> Index<usize> for ImVector<T> {
	type Output = T;

	fn index<'a>(&'a self, _index: usize) -> &'a T {
		let _ref = unsafe { self.data.offset(_index as isize).as_ref() };
		if let Some(r) = _ref {
			return r
		} else {
			panic!("Bad index in ImVector: {} [size: {}] [capacity: {}]", _index, self.size, self.capacity);
		}
	}
}

impl<T> IndexMut<usize> for ImVector<T> {
	fn index_mut<'a>(&'a mut self, _index: usize) -> &'a mut T {
		let _ref = unsafe { self.data.offset(_index as isize).as_mut() };
		if let Some(r) = _ref {
			return r
		} else {
			panic!("Bad mut index in ImVector: {} [size: {}] [capacity: {}]", _index, self.size, self.capacity);
		}
	}
}

use super::imstr::ImStr;
use super::imstrarray::ImStrArray;
use super::raw::types::*;
use super::raw::api::*;
use super::enums::*;
use std::mem::transmute;
use std::ffi::CStr;
use libc::{
	c_float,
	c_int,
	c_char,
	c_void
};
use std::ptr;

pub type ProducerFunction<T> = Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int) -> T>;
pub type ListBoxProducerFunction<T> = Option<unsafe extern "C" fn(data: *mut c_void, idx: c_int, out_text: *mut *const c_char) -> T>;

unsafe fn as_ref<T>(ptr: *mut T) -> Option<&'static mut T> {
	Some(transmute(ptr))
}

pub fn cbool(b: bool) -> i8 {
	unsafe { transmute(b) }
}

pub fn get_io_safe() -> Option<&'static mut ImGuiIO> {
	unsafe { as_ref(igGetIO()) }
}

pub fn get_io() -> &'static mut ImGuiIO {
	unsafe { as_ref(igGetIO()).expect("Failed to get an instance of ImGuiIO.") }
}

pub fn get_style() -> Option<&'static mut ImGuiStyle> {
	unsafe { as_ref(igGetStyle()) }
}

pub fn get_draw_data() -> Option<&'static mut ImDrawData> {
	unsafe { as_ref(igGetDrawData()) }
}

pub fn new_frame() {
	unsafe { igNewFrame() }
}

pub fn render() {
	unsafe { igRender() }
}

pub fn shutdown() {
	unsafe { igShutdown() }
}

pub fn show_user_guide() {
	unsafe { igShowUserGuide() }
}

pub fn show_style_editor(_ref: &mut ImGuiStyle) {
	unsafe { igShowStyleEditor(_ref) }
}

pub fn show_test_window(opened: &mut bool) {
	unsafe { igShowTestWindow(transmute(opened)) }
}

pub fn show_metrics_window(opened: &mut bool) {
	unsafe { igShowMetricsWindow(transmute(opened)) }
}

pub fn begin<'a>(name: ImStr<'a>, p_opened: &mut bool, flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBegin(name.as_ptr(), transmute(p_opened), flags.as_c()) != 0
	}
}

pub fn begin2<'a>(name: ImStr<'a>, p_opened: &mut bool, size_on_first_use: ImVec2, bg_alpha: f32, flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBegin2(name.as_ptr(), transmute(p_opened), size_on_first_use, bg_alpha , flags.as_c()) != 0
	}
}

pub fn end() {
	unsafe { igEnd() }
}

pub fn begin_child<'a>(str_id: ImStr<'a>, size: ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBeginChild(str_id.as_ptr(), size, transmute(border), extra_flags.as_c()) != 0
	}
}

pub fn begin_child_ex(id: ImGuiID, size: ImVec2, border: bool, extra_flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBeginChildEx(id, size, transmute(border), extra_flags.as_c()) != 0
	}
}

pub fn end_child() {
	unsafe {
		igEndChild();
	}
}

pub fn get_content_region_max(out: &mut ImVec2) {
	unsafe {
		igGetContentRegionMax(out)
	}
}

pub fn get_content_region_avail(out: &mut ImVec2) {
	unsafe {
		igGetContentRegionAvail(out)
	}
}

pub fn get_content_region_avail_width() -> f32 {
	unsafe {
		igGetContentRegionAvailWidth() as f32
	}
}

pub fn get_window_content_region_min(out: &mut ImVec2) {
	unsafe {
		igGetWindowContentRegionMin(out)
	}
}

pub fn get_window_content_region_max(out: &mut ImVec2) {
	unsafe {
		igGetWindowContentRegionMax(out)
	}
}

pub fn get_window_content_region_width() -> f32 {
	unsafe {
		igGetWindowContentRegionWidth()
	}
}

pub fn get_window_draw_list() -> Option<&'static mut ImDrawList> {
	unsafe {
		as_ref(igGetWindowDrawList())
	}
}

pub fn get_window_pos(out: &mut ImVec2) {
	unsafe {
		igGetWindowPos(out)
	}
}

pub fn get_window_size(out: &mut ImVec2) {
	unsafe {
		igGetWindowSize(out)
	}
}

pub fn get_window_width() -> f32 {
	unsafe {
		igGetWindowWidth() as f32
	}
}

pub fn get_window_height() -> f32 {
	unsafe {
		igGetWindowHeight() as f32
	}
}

pub fn is_window_collapsed() -> bool {
	unsafe {
		igIsWindowCollapsed() != 0
	}
}

pub fn set_window_font_scale(scale: f32) {
	unsafe {
		igSetWindowFontScale(scale )
	}
}

pub fn set_next_window_pos(pos: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetNextWindowPos(pos, cond.as_c());
	}
}

pub fn set_next_window_pos_center(cond: ImGuiSetCond) {
	unsafe {
		igSetNextWindowPosCenter(cond.as_c());
	}
}

pub fn set_next_window_size(size: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetNextWindowSize(size, cond.as_c());
	}
}

pub fn set_next_window_content_size(size: ImVec2) {
	unsafe {
		igSetNextWindowContentSize(size)
	}
}

pub fn set_next_window_content_width(width: f32) {
	unsafe {
		igSetNextWindowContentWidth(width )
	}
}

pub fn set_next_window_collapsed(collapsed: bool, cond: ImGuiSetCond) {
	unsafe {
		igSetNextWindowCollapsed(transmute(collapsed), cond.as_c())
	}
}

pub fn set_next_window_focus() {
	unsafe {
		igSetNextWindowFocus()
	}
}

pub fn set_window_pos(pos: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowPos(pos, cond.as_c())
	}
}

pub fn set_window_size(size: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowSize(size, cond.as_c())
	}
}

pub fn set_window_collapsed(collapsed: bool, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowCollapsed(transmute(collapsed), cond.as_c())
	}
}

pub fn set_window_focus() {
	unsafe {
		igSetWindowFocus()
	}
}

pub fn set_window_pos_by_name<'a>(name: ImStr<'a>, pos: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowPosByName(name.as_ptr(), pos, cond.as_c())
	}
}

// #TODO alias set_window_size_by_name
pub fn set_window_size2<'a>(name: ImStr<'a>, size: ImVec2, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowSize2(name.as_ptr(), size, cond.as_c())
	}
}

// #TODO alias set_window_collapsed_by_name
pub fn set_window_collapsed2<'a>(name: ImStr<'a>, collapsed: bool, cond: ImGuiSetCond) {
	unsafe {
		igSetWindowCollapsed2(name.as_ptr(), transmute(collapsed), cond.as_c())
	}
}

// #TODO alias set_window_focus_by_name
pub fn set_window_focus2<'a>(name: ImStr<'a>) {
	unsafe {
		igSetWindowFocus2(name.as_ptr())
	}
}

pub fn get_scroll_x() -> f32 {
	unsafe {
		igGetScrollX() as f32
	}
}

pub fn get_scroll_y() -> f32 {
	unsafe {
		igGetScrollY() as f32
	}
}

pub fn get_scroll_max_x() -> f32 {
	unsafe {
		igGetScrollMaxX() as f32
	}
}

pub fn get_scroll_max_y() -> f32 {
	unsafe {
		igGetScrollMaxY() as f32
	}
}

pub fn set_scroll_x(scroll_x: f32) {
	unsafe {
		igSetScrollX(scroll_x )
	}
} 

pub fn set_scroll_y(scroll_y: f32) {
	unsafe {
		igSetScrollY(scroll_y )
	}
}

pub fn set_scroll_here(center_y_ratio: f32) {
	unsafe {
		igSetScrollHere(center_y_ratio )
	}
}

pub fn set_scroll_from_pos_y(pos_y: f32, center_y_ratio: f32) {
	unsafe {
		igSetScrollFromPosY(pos_y , center_y_ratio )
	}
}

pub fn set_keyboard_focus_here(offset: i32) {
	unsafe {
		igSetKeyboardFocusHere(offset)
	}
}

pub fn set_state_storage(tree: &mut ImGuiStorage) {
	unsafe {
		igSetStateStorage(tree)
	}
}

pub fn get_state_storage() -> Option<&'static mut ImGuiStorage> {
	unsafe {
		as_ref(igGetStateStorage())
	}
}

pub fn push_font(font: &mut ImFont) {
	unsafe {
		igPushFont(font)
	}
}

pub fn pop_font() {
	unsafe {
		igPopFont()
	}
}

pub fn push_style_color(idx: ImGuiCol, col: ImVec4) {
	unsafe {
		igPushStyleColor(idx.as_c(), col)
	}
}

pub fn push_style_var(idx: ImGuiStyleVar, val: f32) {
	unsafe {
		igPushStyleVar(idx.as_c(), val)
	}
}

pub fn push_style_var_vec(idx: ImGuiStyleVar, val: ImVec2) {
	unsafe {
		igPushStyleVarVec(idx.as_c(), val)
	}
}

pub fn pop_style_var(count: i32) {
	unsafe {
		igPopStyleVar(count )
	}
}

pub fn get_font() -> Option<&'static mut ImFont> {
	unsafe {
		as_ref(igGetFont())
	}
}

pub fn get_font_size() -> f32 {
	unsafe {
		igGetFontSize() as f32
	}
}

pub fn get_font_tex_uv_white_pixel(p_out: &mut ImVec2) {
	unsafe {
		igGetFontTexUvWhitePixel(p_out)
	}
}

pub fn get_color_u32(idx: ImGuiCol, alpha_mul: f32) -> u32 {
	unsafe {
		igGetColorU32(idx.as_c(), alpha_mul ) as u32
	}
}

pub fn get_color_u32_vec(col: &ImVec4) -> u32 {
	unsafe {
		igGetColorU32Vec(col) as u32
	}
}

pub fn push_item_width(item_width: f32) {
	unsafe {
		igPushItemWidth(item_width )
	}
}

pub fn pop_item_width() {
	unsafe {
		igPopItemWidth()
	}
}

pub fn calc_item_width() -> f32 {
	unsafe {
		igCalcItemWidth() as f32
	}
}

pub fn push_text_wrap_pos(wrap_pos_x: f32) {
	unsafe {
		igPushTextWrapPos(wrap_pos_x )
	}
}

pub fn push_allow_keyboard_focus(v: bool) {
	unsafe {
		igPushAllowKeyboardFocus(transmute(v))
	}
}

pub fn pop_allow_keyboard_focus() {
	unsafe {
		igPopAllowKeyboardFocus()
	}
}

pub fn push_button_repeat(repeat: bool) {
	unsafe {
		igPushButtonRepeat(transmute(repeat))
	}
}

pub fn pop_button_repeat() {
	unsafe {
		igPopButtonRepeat()
	}
}

pub fn begin_group() {
	unsafe {
		igBeginGroup()
	}
}

pub fn end_group() {
	unsafe {
		igEndGroup()
	}
}

pub fn separator() {
	unsafe {
		igSeparator()
	}
}

pub fn same_line() {
	unsafe {
		igSameLine(0.0, -1.0);
	}
}

pub fn same_line_ex(pos_x: f32, spacing_w: f32) {
	unsafe {
		igSameLine(pos_x, spacing_w)
	}
}

pub fn spacing() {
	unsafe {
		igSpacing()
	}
}

pub fn dummy(size: &ImVec2) {
	unsafe {
		igDummy(size)
	}
}

pub fn indent() {
	unsafe {
		igIndent()
	}
}

pub fn unindent() {
	unsafe {
		igUnindent()
	}
}

pub fn get_cursor_pos(p_out: &mut ImVec2) {
	unsafe {
		igGetCursorPos(p_out)
	}
}

pub fn get_cursor_pos_x() -> f32 {
	unsafe {
		igGetCursorPosX() as f32
	}
}

pub fn get_cursor_pos_y() -> f32 {
	unsafe {
		igGetCursorPosY() as f32
	}
}

pub fn set_cursor_pos(local_pos: ImVec2) {
	unsafe {
		igSetCursorPos(local_pos)
	}
}

pub fn set_cursor_pos_x(x: f32) {
	unsafe {
		igSetCursorPosX(x )
	}
}

pub fn set_cursor_pos_y(y: f32) {
	unsafe {
		igSetCursorPosY(y )
	}
}

pub fn get_cursor_start_pos(p_out: &mut ImVec2) {
	unsafe {
		igGetCursorStartPos(p_out)
	}
}

pub fn get_cursor_screen_pos(p_out: &mut ImVec2) {
	unsafe {
		igGetCursorScreenPos(p_out)
	}
}

pub fn set_cursor_screen_pos(pos: ImVec2) {
	unsafe {
		igSetCursorPos(pos)
	}
}

pub fn is_align_first_text_height_to_widgets() {
	unsafe {
		igAlignFirstTextHeightToWidgets()
	}
}

pub fn get_text_line_height() -> f32 {
	unsafe {
		igGetTextLineHeight() as f32
	}
}

pub fn get_text_line_height_with_spacing() -> f32 {
	unsafe {
		igGetTextLineHeightWithSpacing() as f32
	}
}

pub fn get_items_line_height_width_spacing() -> f32 {
	unsafe {
		igGetItemsLineHeightWithSpacing() as f32
	}
}

pub fn columns<'a>(count: i32, id: ImStr<'a>, border: bool) {
	unsafe {
		igColumns(count, id.as_ptr(), transmute(border))
	}
}

pub fn next_column() {
	unsafe {
		igNextColumn()
	}
}

pub fn get_column_index() -> i32 {
	unsafe {
		igGetColumnIndex() as i32
	}
}

pub fn get_column_offset(column_index: i32) -> f32 {
	unsafe {
		igGetColumnOffset(column_index ) as f32
	}
}

pub fn set_column_offset(column_index: i32, offset_x: f32) {
	unsafe {
		igSetColumnOffset(column_index , offset_x )
	}
}

pub fn get_column_width(column_index: i32) -> f32 {
	unsafe {
		igGetColumnWidth(column_index ) as f32
	}
}

pub fn get_columns_count() -> i32 {
	unsafe {
		igGetColumnsCount() as i32
	}
}

pub fn push_id_str<'a>(str_id: ImStr<'a>) {
	unsafe {
		igPushIdStr(str_id.as_ptr())
	}
}

pub fn push_id_str_range<'a>(str_: ImStr<'a>) {
	unsafe {
		igPushIdStrRange(str_.begin(), str_.end())
	}
}

pub fn push_id_ptr(ptr_id: &c_void) {
	unsafe {
		igPushIdPtr(ptr_id)
	}
}

pub fn push_id_int(int_id: i32) {
	unsafe {
		igPushIdInt(int_id )
	}
}

pub fn pop_id() {
	unsafe {
		igPopId()
	}
}

pub fn get_id_str<'a>(str_id: ImStr<'a>) -> ImGuiID {
	unsafe {
		igGetIdStr(str_id.as_ptr())
	}
}

pub fn get_id_str_range<'a>(str_: ImStr<'a>) -> ImGuiID {
	unsafe {
		igGetIdStrRange(str_.begin(), str_.end())
	}
}


pub fn get_id_ptr(ptr_id: &c_void) -> ImGuiID {
	unsafe {
		igGetIdPtr(ptr_id)
	}
}

pub fn text<'a>(fmt: ImStr<'a>) {
	unsafe { igText(fmt.as_ptr()) }
}

pub fn get_text_colored<'a>(col: ImVec4, fmt: ImStr<'a>) {
	unsafe { igTextColored(col, fmt.as_ptr()) }
}

pub fn text_disabled<'a>(fmt: ImStr<'a>) {
	unsafe { igTextDisabled(fmt.as_ptr()) }
}

pub fn text_wrapped<'a>(fmt: ImStr<'a>) {
	unsafe { igTextWrapped(fmt.as_ptr()) }
}

pub fn text_unformatted<'a>(text: ImStr<'a>) {
	unsafe {
		igTextUnformatted(text.begin(), text.end())
	}
}

pub fn label_text<'a>(label: ImStr<'a>, fmt: ImStr<'a>) {
	unsafe {
		igLabelText(label.as_ptr(), fmt.as_ptr())
	}
}

pub fn bullet() {
	unsafe {
		igBullet();
	}
}

pub fn bullet_text<'a>(fmt: ImStr<'a>) {
	unsafe {
		igBulletText(fmt.as_ptr())
	}
}

pub fn button<'a>(label: ImStr<'a>, size: ImVec2) -> bool {
	unsafe {
		igButton(label.as_ptr(), size) != 0
	}
}

pub fn small_button<'a>(label: ImStr<'a>) -> bool {
	unsafe {
		igSmallButton(label.as_ptr()) != 0
	}
}

pub fn invisible_button<'a>(str_id: ImStr<'a>, size: ImVec2) -> bool {
	unsafe {
		igInvisibleButton(str_id.as_ptr(), size) != 0
	}
}

pub fn image(user_texture_id: ImTextureID, size: ImVec2, uv0: ImVec2, uv1: ImVec2,
	tint_col: ImVec4, border_col: ImVec4) {
	unsafe {
		igImage(user_texture_id, size, uv0, uv1, tint_col, border_col)
	}
}

pub fn image_button(user_texture_id: ImTextureID, size: ImVec2, uv0: ImVec2, uv1: ImVec2,
	frame_padding: i32, tint_col: ImVec4, border_col: ImVec4) -> bool {
	unsafe {
		igImageButton(user_texture_id, size, uv0, uv1, frame_padding , tint_col, border_col) != 0
	}
}

pub fn collapsing_header<'a>(label: ImStr<'a>, str_id: ImStr<'a>, display_frame: bool, default_open: bool) -> bool {
	unsafe {
		igCollapsingHeader(label.as_ptr(), str_id.as_ptr(), transmute(display_frame), transmute(default_open)) != 0
	}
}

pub fn checkbox<'a>(label: ImStr<'a>, b: &mut bool) -> bool {
	unsafe {
		igCheckbox(label.as_ptr(), transmute(b)) != 0
	}
}

pub fn checkbox_flags<'a>(label: ImStr<'a>, flags: &mut u32, flags_value: u32) -> bool {
	unsafe {
		igCheckboxFlags(label.as_ptr(), flags, flags_value) != 0
	}
}

pub fn radio_button_bool<'a>(label: ImStr<'a>, active: bool) -> bool {
	unsafe { igRadioButtonBool(label.as_ptr(), transmute(active)) != 0 }
}

pub fn radio_button<'a>(label: ImStr<'a>, v: &mut i32, v_button: i32) -> bool {
	unsafe {
		igRadioButton(label.as_ptr(), v, v_button) != 0
	}
}

pub fn combo<'a>(label: ImStr<'a>, current_item: &mut i32, mut items: ImStrArray<'a>, height_in_items: i32) -> bool {
	unsafe {
		igCombo(label.as_ptr(), current_item, items.as_mut_ptr(), items.len_c(), height_in_items) != 0
	}
}

pub fn color_button(col: ImVec4, small_height: bool, outline_border: bool) -> bool {
	unsafe {
		igColorButton(col, transmute(small_height), transmute(outline_border)) != 0
	}
}

pub fn color_edit_3<'a>(label: ImStr<'a>, col: &mut f32) -> bool {
	unsafe {
		igColorEdit3(label.as_ptr(), col as *mut c_float) != 0
	}
}

pub fn color_edit_4<'a>(label: ImStr<'a>, col: &mut f32, show_alpha: bool) -> bool {
	unsafe {
		igColorEdit4(label.as_ptr(), col as *mut c_float, transmute(show_alpha)) != 0
	}
}

pub fn color_edit_mode(mode: ImGuiColorEditMode) {
	unsafe {
		igColorEditMode(mode.as_c())
	}
}

pub fn plot_lines<'a>(label: ImStr<'a>, values: &[f32], values_count: i32, values_offset: i32, overlay_text: ImStr<'a>, scale_min: f32, scale_max: f32, graph_size: ImVec2, stride: i32) {
	unsafe {
		igPlotLines(label.as_ptr(), values.as_ptr(), values_count , values_offset ,
			overlay_text.as_ptr(), scale_min , scale_max as f32, graph_size, stride )
	}
}

pub fn plot_lines_2<'a>(label: ImStr<'a>, values_getter: ProducerFunction<c_float>, data: &mut c_void,
	values_count: i32, values_offset: i32, overlay_text: ImStr<'a>, scale_min: f32, scale_max: f32, graph_size: ImVec2) {
	unsafe {
		igPlotLines2(label.as_ptr(), values_getter, data, values_count , values_offset , overlay_text.as_ptr(),
			scale_min , scale_max , graph_size)
	}
}

pub fn plot_histogram<'a>(label: ImStr<'a>, values: &[f32], values_count: i32, values_offset: i32, overlay_text: ImStr<'a>, scale_min: f32, scale_max: f32, graph_size: ImVec2, stride: i32) {
	unsafe {
		igPlotHistogram(label.as_ptr(), values.as_ptr(), values_count , values_offset , overlay_text.as_ptr(), scale_min , scale_max , graph_size, stride )
	}
}

pub fn plot_histogram2<'a>(label: ImStr<'a>, values_getter: ProducerFunction<c_float>, data: &mut c_void, values_count: i32, values_offset: i32, overlay_text: ImStr<'a>, scale_min: f32, scale_max: f32, graph_size: ImVec2) {
	unsafe {
		igPlotHistogram2(label.as_ptr(), values_getter, data, values_count , values_offset , overlay_text.as_ptr(), scale_min , scale_max , graph_size)
	}
}

pub fn progress_bar<'a>(fraction: f32, size_arg: &ImVec2, overlay_text: ImStr<'a>) {
	unsafe {
		igProgressBar(fraction , size_arg, overlay_text.as_ptr())
	}
}

pub fn slider_float<'a>(label: ImStr<'a>, v: &mut f32, v_min: f32, v_max: f32, display_format: ImStr<'a>, power: f32) -> bool {
	unsafe {
		igSliderFloat(label.as_ptr(), v, v_min, v_max, display_format.as_ptr(), power) != 0
	}
}

pub fn slider_angle<'a>(label: ImStr<'a>, v_rad: &mut f32, v_degrees_min: f32, v_degrees_max: f32) -> bool {
	unsafe {
		igSliderAngle(label.as_ptr(), v_rad, v_degrees_min, v_degrees_max) != 0
	}
}

pub fn slider_int<'a>(label: ImStr<'a>, v: &mut i32, v_min: i32, v_max: i32, display_format: ImStr<'a>) -> bool {
	unsafe {
		igSliderInt(label.as_ptr(), v, v_min, v_max, display_format.as_ptr()) != 0
	}
}

pub fn v_slider_float<'a>(label: ImStr<'a>, size: ImVec2, v: &mut f32, v_min: f32, v_max: f32, display_format: ImStr<'a>, power: f32) -> bool {
	unsafe {
		igVSliderFloat(label.as_ptr(), size, v, v_min, v_max, display_format.as_ptr(), power) != 0
	}
}

pub fn v_slider_int<'a>(label: ImStr<'a>, size: ImVec2, v: &mut i32, v_min: i32, v_max: i32, display_format: ImStr<'a>) -> bool {
	unsafe {
		igVSliderInt(label.as_ptr(), size, v, v_min, v_max, display_format.as_ptr()) != 0
	}
}

pub fn drag_float<'a>(label: ImStr<'a>, v: &mut f32, v_speed: f32, v_min: f32, v_max: f32, display_format: ImStr<'a>, power: f32) -> bool {
	unsafe {
		igDragFloat(label.as_ptr(), v, v_speed, v_min, v_max, display_format.as_ptr(), power) != 0
	}
}

pub fn drag_float_range_2<'a>(label: ImStr<'a>, v_current_min: &mut f32, v_current_max: &mut f32, v_speed: f32, v_min: f32, v_max: f32, display_format: ImStr<'a>, display_format_max: ImStr<'a>, power: f32) -> bool {
	unsafe {
		igDragFloatRange2(label.as_ptr(), v_current_min, v_current_max, v_speed, v_min, v_max, display_format.as_ptr(), display_format_max.as_ptr(), power) != 0
	}
}

pub fn drag_int<'a>(label: ImStr<'a>, v: &mut i32, v_speed: f32, v_min: i32, v_max: i32, display_format: ImStr<'a>) -> bool {
	unsafe {
		igDragInt(label.as_ptr(), v, v_speed, v_min, v_max, display_format.as_ptr()) != 0
	}
}

pub fn drag_int_range_2<'a>(label: ImStr<'a>, v_current_min: &mut i32, v_current_max: &mut i32, v_speed: f32, v_min: i32, v_max: i32, display_format: ImStr<'a>, display_format_max: ImStr<'a>) -> bool {
	unsafe {
		igDragIntRange2(label.as_ptr(), v_current_min, v_current_max, v_speed, v_min, v_max, display_format.as_ptr(), display_format_max.as_ptr()) != 0
	}
}

pub fn input_text<'a>(label: ImStr<'a>, buf: &mut [i8], flags: ImGuiInputTextFlags, callback: ImGuiTextEditCallback, user_data: Option<&mut c_void>) -> bool {
	unsafe {
		igInputText(label.as_ptr(), buf.as_mut_ptr(), buf.len(), flags.as_c(), callback, transmute(user_data)) != 0
	}
}

pub fn input_text_multiline<'a>(label: ImStr<'a>, buf: &mut [i8], size: ImVec2, flags: ImGuiInputTextFlags, callback: ImGuiTextEditCallback, user_data: Option<&mut c_void>) -> bool {
	unsafe {
		igInputTextMultiline(label.as_ptr(), buf.as_mut_ptr(), buf.len(), size, flags.as_c(), callback, transmute(user_data)) != 0
	}
}

pub fn input_float<'a>(label: ImStr<'a>, v: &mut f32, step: f32, step_fast: f32, decimal_precision: i32, extra_flags: ImGuiInputTextFlags) -> bool {
	unsafe {
		igInputFloat(label.as_ptr(), v, step, step_fast, decimal_precision, extra_flags.as_c()) != 0
	}
}

pub fn input_int<'a>(label: ImStr<'a>, v: &mut i32, step: i32, step_fast: i32, extra_flags: ImGuiInputTextFlags) -> bool {
	unsafe {
		igInputInt(label.as_ptr(), v, step, step_fast, extra_flags.as_c()) != 0
	}
}

pub fn tree_node<'a>(str_label_id: ImStr<'a>) -> bool {
	unsafe { igTreeNode(str_label_id.as_ptr()) != 0 }
}

pub fn tree_node_str<'a>(str_id: ImStr<'a>, fmt: ImStr<'a>) -> bool {
	unsafe { igTreeNodeStr(str_id.as_ptr(), fmt.as_ptr()) != 0 }
}

pub fn tree_node_ptr<'a>(ptr_id: Option<&c_void>, fmt: ImStr<'a>) -> bool {
	unsafe { igTreeNodePtr(transmute(ptr_id), fmt.as_ptr()) != 0 }
}

pub fn tree_push_str<'a>(str_id: ImStr<'a>) {
	unsafe { igTreePushStr(str_id.as_ptr()) }
}

pub fn tree_push_ptr(ptr_id: Option<&c_void>) {
	unsafe { igTreePushPtr(transmute(ptr_id)) }
}

pub fn tree_pop() {
	unsafe { igTreePop() }
}

pub fn set_next_tree_node_opened(opened: bool, cond: ImGuiSetCond) {
	unsafe { igSetNextTreeNodeOpened(transmute(opened), cond.as_c()) }
}

pub fn selectable<'a>(label: ImStr<'a>) -> bool {
	unsafe {
		igSelectable(label.as_ptr(), 0, 0, ImVec2::zero()) != 0
	}
}


pub fn selectable_fl<'a>(label: ImStr<'a>, flags: ImGuiSelectableFlags) -> bool {
	unsafe {
		igSelectable(label.as_ptr(), 0, flags.as_c(), ImVec2::zero()) != 0
	}
}

pub fn selectable_ex<'a>(label: ImStr<'a>, selected: bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
	unsafe {
		igSelectable(label.as_ptr(), transmute(selected), flags.as_c(), size) != 0
	}
}

pub fn selectable_ex2<'a>(label: ImStr<'a>, p_selected: &mut bool, flags: ImGuiSelectableFlags, size: ImVec2) -> bool {
	unsafe {
		igSelectableEx(label.as_ptr(), transmute(p_selected), flags.as_c(), size) != 0
	}
}

pub fn list_box<'a>(label: ImStr<'a>, current_item: &mut i32, mut items: ImStrArray<'a>, height_in_items: i32) -> bool {
	unsafe {
		igListBox(label.as_ptr(), current_item, items.as_mut_ptr(), items.len_c() as i32, height_in_items) != 0
	}
}

pub fn list_box_2<'a>(label: ImStr<'a>, current_item: &mut i32, items_getter: ListBoxProducerFunction<u8>, data: Option<&mut c_void>, items_count: i32, height_in_items: i32) -> bool {
	unsafe {
		igListBox2(label.as_ptr(), current_item, items_getter, transmute(data), items_count, height_in_items) != 0
	}
}

pub fn list_box_header<'a>(label: ImStr<'a>, size: ImVec2) -> bool {
	unsafe {
		igListBoxHeader(label.as_ptr(), size) != 0
	}
}

pub fn list_box_header_2<'a>(label: ImStr<'a>, items_count: i32, height_in_items: i32) -> bool {
	unsafe {
		igListBoxHeader2(label.as_ptr(), items_count, height_in_items) != 0
	}
}

pub fn list_box_footer() {
	unsafe {
		igListBoxFooter()
	}
}

pub fn value_bool<'a>(prefix: ImStr<'a>, b: bool) {
	unsafe { igValueBool(prefix.as_ptr(), transmute(b)) }
}

pub fn value_int<'a>(prefix: ImStr<'a>, v: i32) {
	unsafe { igValueInt(prefix.as_ptr(), v) }
}

pub fn value_uint<'a>(prefix: ImStr<'a>, v: u32) {
	unsafe { igValueUInt(prefix.as_ptr(), v) }
}

pub fn value_float<'a>(prefix: ImStr<'a>, v: f32, float_format: ImStr<'a>) {
	unsafe { igValueFloat(prefix.as_ptr(), v, float_format.as_ptr()) }
}

pub fn value_color<'a>(prefix: ImStr<'a>, v: ImVec4) {
	unsafe { igValueColor(prefix.as_ptr(), v) }
}

pub fn value_color_2<'a>(prefix: ImStr<'a>, v: u32) {
	unsafe { igValueColor2(prefix.as_ptr(), v) }
}

pub fn set_tooltip<'a>(fmt: ImStr<'a>) {
	unsafe { igSetTooltip(fmt.as_ptr()) }
}

pub fn begin_tooltip() {
	unsafe { igBeginTooltip() }
}

pub fn end_tooltip() {
	unsafe { igEndTooltip() }
}

pub fn begin_main_menu_bar() -> bool {
	unsafe { igBeginMainMenuBar() != 0 }
}

pub fn end_main_menu_bar() {
	unsafe { igEndMainMenuBar() }
}

pub fn begin_menu_bar() -> bool {
	unsafe { igBeginMenuBar() != 0 }
}

pub fn end_menu_bar() {
	unsafe { igEndMenuBar() }
}

pub fn begin_menu<'a>(label: ImStr<'a>, enabled: bool) -> bool {
	unsafe { igBeginMenu(label.as_ptr(), transmute(enabled)) != 0 }
}

pub fn end_menu() {
	unsafe { igEndMenu() }
}

pub fn menu_item<'a>(label: ImStr<'a>) -> bool {
	unsafe {
		igMenuItem(label.as_ptr(), ptr::null(), transmute(false), transmute(true)) != 0
	}
}

pub fn menu_item_ex<'a>(label: ImStr<'a>, selected: bool, enabled: bool) -> bool {
	unsafe {
		igMenuItem(label.as_ptr(), ptr::null(), transmute(selected), transmute(enabled)) != 0
	}
}

pub fn menu_item_shortcut<'a>(label: ImStr<'a>, shortcut: ImStr<'a>, selected: bool, enabled: bool) -> bool {
	unsafe {
		igMenuItem(label.as_ptr(), shortcut.as_ptr(), transmute(selected), transmute(enabled)) != 0
	}
}

pub fn menu_item_ptr<'a>(label: ImStr<'a>, shortcut: ImStr<'a>, p_selected: &mut bool, enabled: bool) -> bool {
	unsafe {
		igMenuItemPtr(label.as_ptr(), shortcut.as_ptr(), transmute(p_selected), transmute(enabled)) != 0
	}
}

pub fn open_popup<'a>(str_id: ImStr<'a>) {
	unsafe { igOpenPopup(str_id.as_ptr()) }
}

pub fn begin_popup<'a>(str_id: ImStr<'a>) -> bool {
	unsafe { igBeginPopup(str_id.as_ptr()) != 0 }
}

pub fn begin_popup_modal<'a>(name: ImStr<'a>, p_opened: &mut bool, extra_flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBeginPopupModal(name.as_ptr(), transmute(p_opened), extra_flags.as_c()) != 0
	}
}

pub fn begin_popup_context_item<'a>(str_id: ImStr<'a>, mouse_button: i32) -> bool {
	unsafe { igBeginPopupContextItem(str_id.as_ptr(), mouse_button) != 0 }
}

pub fn begin_popup_context_window<'a>(also_over_items: bool, str_id: ImStr<'a>, mouse_button: i32) -> bool {
	unsafe { igBeginPopupContextWindow(transmute(also_over_items), str_id.as_ptr(), mouse_button) != 0 }
}

pub fn begin_popup_context_void<'a>(str_id: ImStr<'a>, mouse_button: i32) -> bool {
	unsafe {
		igBeginPopupContextVoid(str_id.as_ptr(), mouse_button) != 0
	}
}

pub fn end_popup() {
	unsafe { igEndPopup() }
}

pub fn close_current_popup() {
	unsafe { igCloseCurrentPopup() }
}

pub fn log_totty(max_depth: i32) {
	unsafe { igLogToTTY(max_depth) }
}

pub fn log_to_file<'a>(max_depth: i32, filename: ImStr<'a>) {
	unsafe { igLogToFile(max_depth, filename.as_ptr()) }
}

pub fn log_to_clipboard(max_depth: i32) {
	unsafe { igLogToClipboard(max_depth) }
}

pub fn log_finish() {
	unsafe { igLogFinish() }
}

pub fn log_buttons() {
	unsafe { igLogButtons() }
}

pub fn log_text<'a>(fmt: ImStr<'a>) {
	unsafe { igLogText(fmt.as_ptr()) }
}

pub fn is_item_hovered() -> bool {
	unsafe { igIsItemHovered() != 0 }
}

pub fn is_item_hovered_rect() -> bool {
	unsafe { igIsItemHoveredRect() != 0 }
}

pub fn is_item_active() -> bool {
	unsafe { igIsItemActive() != 0 }
}

pub fn is_item_visible() -> bool {
	unsafe { igIsItemVisible() != 0 }
}

pub fn is_any_item_hovered() -> bool {
	unsafe { igIsAnyItemHovered() != 0 }
}

pub fn is_any_item_active() -> bool {
	unsafe { igIsAnyItemActive() != 0 }
}

pub fn get_item_rect_min(out: &mut ImVec2) {
	unsafe { igGetItemRectMin(out) }
}

pub fn get_item_rect_max(out: &mut ImVec2) {
	unsafe { igGetItemRectMax(out) }
}

pub fn get_item_rect_size(out: &mut ImVec2) {
	unsafe { igGetItemRectSize(out) }
}

pub fn set_item_allow_overlap() {
	unsafe { igSetItemAllowOverlap() }
}

pub fn is_window_hovered() -> bool {
	unsafe { igIsWindowHovered() != 0 }
}

pub fn is_window_focused() -> bool {
	unsafe { igIsWindowFocused() != 0 }
}

pub fn is_root_window_focused() -> bool {
	unsafe { igIsRootWindowFocused() != 0 }
}

pub fn is_root_window_or_any_child_focused() -> bool {
	unsafe { igIsRootWindowOrAnyChildFocused() != 0 }
}

pub fn is_rect_visible(item_size: ImVec2) -> bool {
	unsafe { igIsRectVisible(item_size) != 0 }
}

pub fn is_pos_hovering_any_window(pos: ImVec2) -> bool {
	unsafe { igIsPosHoveringAnyWindow(pos) != 0 }
}

pub fn get_time() -> f32 {
	unsafe { igGetTime() }
}

pub fn get_frame_count() -> i32 {
	unsafe { igGetFrameCount() }
}

pub fn get_style_col_name<'a>(idx: ImGuiCol) -> &'a CStr {
	unsafe { CStr::from_ptr(igGetStyleColName(idx.as_c())) }
}

pub fn calc_item_rect_closest_point(out: &mut ImVec2, pos: ImVec2, on_edge: bool, outward: f32) {
	unsafe { igCalcItemRectClosestPoint(out, pos, transmute(on_edge), outward) }
}

pub fn calc_text_size<'a>(out: &mut ImVec2, text: ImStr<'a>, hide_text_after_double_hash: bool, wrap_width: f32) {
	unsafe {
		igCalcTextSize(out, text.begin(), text.end(), transmute(hide_text_after_double_hash), wrap_width)
	}
}

pub fn calc_list_clipping(items_count: i32, items_height: f32, out_items_display_start: &mut i32, out_items_display_end: &mut i32) {
	unsafe { igCalcListClipping(items_count, items_height, out_items_display_start, out_items_display_end) }
}

pub fn begin_child_frame(id: ImGuiID, size: ImVec2, extra_flags: ImGuiWindowFlags) -> bool {
	unsafe {
		igBeginChildFrame(id, size, extra_flags.as_c()) != 0
	}
}

pub fn end_child_frame() {
	unsafe { igEndChildFrame() } 
}

pub fn color_convert_u32_to_float_4(out: &mut ImVec4, _in: u32) {
	unsafe { igColorConvertU32ToFloat4(out, _in) }
}

pub fn color_convert_float_4_to_u32(_in: ImVec4) -> u32 {
	unsafe { igColorConvertFloat4ToU32(_in) }
}

pub fn color_convert_rgb_to_hsv(r: f32, g: f32, b: f32, out_h: &mut f32, out_s: &mut f32, out_v: &mut f32) {
	unsafe { igColorConvertRGBtoHSV(r, g, b, out_h, out_s, out_v) }
}

pub fn color_convert_hsv_to_rgb(h: f32, s: f32, v: f32, out_r: &mut f32, out_g: &mut f32, out_b: &mut f32) {
	unsafe { igColorConvertHSVtoRGB(h, s, v, out_r, out_g, out_b) }
}

pub fn get_key_index(key: ImGuiKey) -> i32 {
	unsafe { igGetKeyIndex(key.as_c()) }
}

pub fn is_key_down(key_index: i32) -> bool {
	unsafe { igIsKeyDown(key_index) != 0 }
}

pub fn is_key_pressed(key_index: i32, repeat: bool) -> bool {
	unsafe { igIsKeyPressed(key_index, transmute(repeat)) != 0 }
}

pub fn is_key_released(key_index: i32) -> bool {
	unsafe { igIsKeyReleased(key_index) != 0 }
}

pub fn is_mouse_down(button: i32) -> bool {
	unsafe { igIsMouseDown(button) != 0 }
}

pub fn is_mouse_clicked(button: i32, repeat: bool) -> bool {
	unsafe { igIsMouseClicked(button, transmute(repeat)) != 0 }
}

pub fn is_mouse_double_clicked(button: i32) -> bool {
	unsafe { igIsMouseDoubleClicked(button) != 0 }
}

pub fn is_mouse_released(button: i32) -> bool {
	unsafe { igIsMouseReleased(button) != 0 }
}

pub fn is_mouse_hovering_window() -> bool {
	unsafe { igIsMouseHoveringWindow() != 0 }
}

pub fn is_mouse_hovering_any_window() -> bool {
	unsafe { igIsMouseHoveringAnyWindow() != 0 }
}

pub fn is_mouse_hovering_rect(r_min: ImVec2, r_max: ImVec2, clip: bool) -> bool {
	unsafe { igIsMouseHoveringRect(r_min, r_max, transmute(clip)) != 0 }
}

pub fn is_mouse_dragging(button: i32, lock_threshold: f32) -> bool {
	unsafe { igIsMouseDragging(button, lock_threshold) != 0 }
}

pub fn get_mouse_pos(out: &mut ImVec2) {
	unsafe { igGetMousePos(out) }
}

pub fn get_mouse_pos_on_opening_current_popup(out: &mut ImVec2) {
	unsafe { igGetMousePosOnOpeningCurrentPopup(out) }
}

pub fn get_mouse_drag_delta(out: &mut ImVec2, button: i32, lock_threshold: f32) {
	unsafe { igGetMouseDragDelta(out, button, lock_threshold) }
}

pub fn reset_mouse_drag_delta(button: i32) {
	unsafe { igResetMouseDragDelta(button) }
}

pub fn get_mouse_cursor() -> ImGuiMouseCursor {
	ImGuiMouseCursor::from_c( unsafe { igGetMouseCursor() } )
}

pub fn set_mouse_cursor(_type: ImGuiMouseCursor) {
	unsafe { igSetMouseCursor(_type.as_c()) }
}

pub fn capture_keyboard_from_app(capture: bool) {
	unsafe { igCaptureKeyboardFromApp(transmute(capture)) }
}

pub fn capture_mouse_from_app(capture: bool) {
	unsafe { igCaptureMouseFromApp(transmute(capture)) }
}

pub fn mem_alloc(sz: usize) -> &'static mut c_void {
	unsafe { transmute(igMemAlloc(sz)) }
}

pub fn mem_free(ptr: &mut c_void) {
	unsafe { igMemFree(transmute(ptr)) }
}

pub fn get_clipboard_text<'a>() -> &'a CStr {
	unsafe { CStr::from_ptr(igGetClipboardText()) }
}

pub fn set_clipboard_text<'a>(text: ImStr<'a>) {
	unsafe { igSetClipboardText(text.as_ptr()) }
}

pub fn get_version<'a>() -> &'a CStr {
	unsafe { CStr::from_ptr(igGetVersion()) }
}

pub fn get_internal_state() -> &'static mut c_void {
	unsafe { transmute(igGetInternalState()) }
}

pub fn get_internal_state_size() -> usize {
	unsafe { igGetInternalStateSize() }
}

pub fn set_internal_state(state: Option<&mut c_void>, construct: bool) {
	unsafe {
		igSetInternalState(transmute(state), transmute(construct))
	}
}

impl ImFontConfig {
	pub fn new() -> ImFontConfig {
		use std::mem;
		let mut config = unsafe { mem::uninitialized() };
		Self::default_constructor(&mut config);
		return config
	}

	fn default_constructor(config: &mut ImFontConfig) {
		unsafe {
			ImFontConfig_DefaultConstructor(config)
		}
	}
}

impl ImFontAtlas {
	pub fn get_tex_data_as_rgba32(&mut self, out_pixels: *mut *mut u8,
		out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
		unsafe {
			ImFontAtlas_GetTexDataAsRGBA32(self, out_pixels, out_width, out_height, out_bytes_per_pixel)
		}
	}

	pub fn get_text_data_as_alpha8(&mut self, out_pixels: *mut *mut u8,
		out_width: &mut i32, out_height: &mut i32, out_bytes_per_pixel: &mut i32) {
		unsafe {
			ImFontAtlas_GetTexDataAsAlpha8(self, out_pixels, out_width, out_height, out_bytes_per_pixel)
		}
	}

	pub fn set_tex_id(&mut self, tex: Option<&mut c_void>) {
		unsafe {
			ImFontAtlas_SetTexID(self, transmute(tex))
		}
	}

	pub fn add_font(&mut self, font_cfg: &ImFontConfig) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFont(self, font_cfg))
		}
	}

	pub fn add_font_default(&mut self, font_cfg: &ImFontConfig) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFontDefault(self, font_cfg))
		}
	}

	pub fn add_front_from_file_ttf<'a>(&mut self, filename: ImStr<'a>, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &ImWchar) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFontFromFileTTF(self, filename.as_ptr(), size_pixels, font_cfg, glyph_ranges))
		}
	}

	pub fn add_font_from_memory_ttf(&mut self, ttf_data: &mut c_void, ttf_size: i32, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &ImWchar) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFontFromMemoryTTF(self, ttf_data, ttf_size, size_pixels, font_cfg, glyph_ranges))
		}
	}

	pub fn add_font_from_memory_compressed_ttf(&mut self, compressed_ttf_data: &c_void, compressed_ttf_size: i32, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &ImWchar) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFontFromMemoryCompressedTTF(self, compressed_ttf_data, compressed_ttf_size, size_pixels, font_cfg, glyph_ranges))
		}
	}

	pub fn add_font_from_memory_compressed_base85_ttf(&mut self, compressed_ttf_data_base85: &i8, size_pixels: f32, font_cfg: &ImFontConfig, glyph_ranges: &ImWchar) -> Option<&'static mut ImFont> {
		unsafe {
			as_ref(ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(self ,compressed_ttf_data_base85, size_pixels, font_cfg, glyph_ranges))
		}
	}

	pub fn clear_tex_data(&mut self) {
		unsafe {
			ImFontAtlas_ClearTexData(self)
		}
	}

	pub fn clear(&mut self) {
		unsafe {
			ImFontAtlas_Clear(self)
		}
	}
}

impl ImGuiIO {
	pub fn add_input_character(&self, c: u16) {
		unsafe {
			ImGuiIO_AddInputCharacter(c)
		}
	}

	pub fn add_input_characters_utf8<'a>(&self, utf8_chars: ImStr<'a>) {
		unsafe {
			ImGuiIO_AddInputCharactersUTF8(utf8_chars.as_ptr())
		}
	}

	pub fn clear_input_characters(&self, ) {
		unsafe {
			ImGuiIO_ClearInputCharacters();
		}
	}
}

impl ImDrawData {
	pub fn deindex_all_buffers(&mut self) {
		unsafe {
			ImDrawData_DeIndexAllBuffers(self)
		}
	}
}

impl ImDrawList {
	pub fn get_vertex_buffer_size(&mut self) -> i32 {
		unsafe {
			ImDrawList_GetVertexBufferSize(self)
		}
	}

	pub fn get_vertex_ptr(&mut self, n: i32) -> Option<&'static mut ImDrawVert> {
		unsafe {
			as_ref(ImDrawList_GetVertexPtr(self, n))
		}
	}

	pub fn get_index_buffer_size(&mut self) -> i32 {
		unsafe {
			ImDrawList_GetIndexBufferSize(self)
		}
	}

	pub fn get_index_ptr(&mut self, n: i32) -> Option<&'static mut ImDrawIdx> {
		unsafe {
			as_ref(ImDrawList_GetIndexPtr(self, n))
		}
	}

	pub fn get_cmd_size(&mut self) -> i32 {
		unsafe {
			ImDrawList_GetCmdSize(self)
		}
	}

	pub fn get_cmd_ptr(&mut self, n: i32) -> Option<&'static mut ImDrawCmd> {
		unsafe {
			as_ref(ImDrawList_GetCmdPtr(self, n))
		}
	}

	pub fn clear(&mut self) {
		unsafe {
			ImDrawList_Clear(self)
		}
	}

	pub fn clear_free_memory(&mut self) {
		unsafe {
			ImDrawList_ClearFreeMemory(self)
		}
	}

	pub fn push_clip_rect(&mut self, clip_rect: ImVec4) {
		unsafe {
			ImDrawList_PushClipRect(self, clip_rect)
		}
	}

	pub fn pop_clip_rect(&mut self) {
		unsafe {
			ImDrawList_PopClipRect(self)
		}
	}

	pub fn push_texture_id(&mut self, texture_id: ImTextureID) {
		unsafe {
			ImDrawList_PushTextureID(self, texture_id)
		}
	}

	pub fn pop_texture_id(&mut self) {
		unsafe {
			ImDrawList_PopTextureID(self)
		}
	}

	pub fn add_line(&mut self, a: ImVec2, b: ImVec2, col: u32, thickness: f32) {
		unsafe {
			ImDrawList_AddLine(self, a, b, col, thickness)
		}
	}

	pub fn add_rect(&mut self, a: ImVec2, b: ImVec2, col: u32, rounding: f32, rounding_corners: i32, thickness: f32) {
		unsafe {
			ImDrawList_AddRect(self, a, b, col, rounding, rounding_corners, thickness)
		}
	}

	pub fn add_rect_filled(&mut self, a: ImVec2, b: ImVec2, col: u32, rounding: f32, rounding_corners: i32) {
		unsafe {
			ImDrawList_AddRectFilled(self, a, b, col, rounding, rounding_corners)
		}
	}

	pub fn add_rect_filled_multicolor(&mut self, a: ImVec2, b: ImVec2, col_upr_left: u32, col_upr_right: u32, col_bot_right: u32, col_bot_left: u32) {
		unsafe {
			ImDrawList_AddRectFilledMultiColor(self, a, b, col_upr_left, col_upr_right, col_bot_right, col_bot_left)
		}
	}

	pub fn add_triangle(&mut self, a: ImVec2, b: ImVec2, c: ImVec2, col: u32, thickness: f32) {
		unsafe {
			ImDrawList_AddTriangle(self, a, b, c, col, thickness)
		}
	}

	pub fn add_triangle_filled(&mut self, a: ImVec2, b: ImVec2, c: ImVec2, col: u32) {
		unsafe {
			ImDrawList_AddTriangleFilled(self, a, b, c, col)
		}
	}

	pub fn add_circle(&mut self, centre: ImVec2, radius: f32, col: u32, num_segments: i32, thickness: f32) {
		unsafe {
			ImDrawList_AddCircle(self, centre, radius, col, num_segments, thickness)
		}
	}

	pub fn add_circle_filled(&mut self, centre: ImVec2, radius: f32, col: u32, num_segments: i32) {
		unsafe {
			ImDrawList_AddCircleFilled(self, centre, radius, col, num_segments)
		}
	}

	pub fn add_text<'a>(&mut self, pos: ImVec2, col: u32, text: ImStr<'a>) {
		unsafe {
			ImDrawList_AddText(self, pos, col, text.begin(), text.end())
		}
	}

	pub fn add_text_ext<'a>(&mut self, font: &ImFont, font_size: f32, pos: ImVec2, col: u32, text: ImStr<'a>, wrap_width: f32, cpu_fine_clip_rect: &ImVec4) {
		unsafe {
			ImDrawList_AddTextExt(self, font, font_size, pos, col, text.begin(), text.end(), wrap_width, cpu_fine_clip_rect)
		}
	}

	pub fn add_image(&mut self, user_texture_id: ImTextureID, a: ImVec2, b: ImVec2, uv0: ImVec2, uv1: ImVec2, col: u32) {
		unsafe {
			ImDrawList_AddImage(self, user_texture_id, a, b, uv0, uv1, col)
		}
	}

	pub fn add_polyline(&mut self, points: &[ImVec2], col: u32, closed: bool, thickness: f32, anti_aliased: bool) {
		unsafe {
			ImDrawList_AddPolyline(self, points.as_ptr(), points.len() as i32, col, transmute(closed), thickness, transmute(anti_aliased))
		}
	}

	pub fn add_convex_poly_filled(&mut self, points: &[ImVec2], col: u32, anti_aliased: bool) {
		unsafe {
			ImDrawList_AddConvexPolyFilled(self, points.as_ptr(), points.len() as i32, col, transmute(anti_aliased))
		}
	}

	pub fn add_bezier_curve(&mut self, pos0: ImVec2, cp0: ImVec2, cp1: ImVec2, pos1: ImVec2, col: u32, thickness: f32, num_segments: i32) {
		unsafe {
			ImDrawList_AddBezierCurve(self, pos0, cp0, cp1, pos1, col, thickness, num_segments)
		}
	}

	pub fn path_clear(&mut self) {
		unsafe {
			ImDrawList_PathClear(self)
		}
	}

	pub fn path_line_to(&mut self, pos: ImVec2) {
		unsafe {
			ImDrawList_PathLineTo(self, pos)
		}
	}

	pub fn path_line_to_merge_duplicate(&mut self, pos: ImVec2) {
		unsafe {
			ImDrawList_PathLineToMergeDuplicate(self, pos)
		}
	}

	pub fn path_fill(&mut self, col: u32) {
		unsafe {
			ImDrawList_PathFill(self, col)
		}
	}

	pub fn path_stroke(&mut self, col: u32, closed: bool, thickness: f32) {
		unsafe {
			ImDrawList_PathStroke(self, col, transmute(closed), thickness)
		}
	}

	pub fn path_arc_to(&mut self, centre: ImVec2, radius: f32, a_min: f32, a_max: f32, num_segments: i32) {
		unsafe {
			ImDrawList_PathArcTo(self, centre, radius, a_min, a_max, num_segments)
		}
	}

	pub fn path_arc_to_fast(&mut self, centre: ImVec2, radius: f32, a_min_of_12: i32, a_max_of_12: i32) {
		unsafe {
			ImDrawList_PathArcToFast(self, centre, radius, a_min_of_12, a_max_of_12)
		}
	}

	pub fn path_bezier_curve_to(&mut self, p1: ImVec2, p2: ImVec2, p3: ImVec2, num_segments: i32) {
		unsafe {
			ImDrawList_PathBezierCurveTo(self, p1, p2, p3, num_segments)
		}
	}

	pub fn path_rect(&mut self, rect_min: ImVec2, rect_max: ImVec2, rounding: f32, rounding_corners: i32) {
		unsafe {
			ImDrawList_PathRect(self, rect_min, rect_max, rounding, rounding_corners)
		}
	}

	pub fn channel_split(&mut self, channels_count: i32) {
		unsafe {
			ImDrawList_ChannelsSplit(self, channels_count)
		}
	}

	pub fn channels_merge(&mut self) {
		unsafe {
			ImDrawList_ChannelsMerge(self)
		}
	}

	pub fn channels_set_current(&mut self, channel_index: i32) {
		unsafe {
			ImDrawList_ChannelsSetCurrent(self, channel_index)
		}
	}

	pub fn add_callback(&mut self, callback: ImDrawCallback, callback_data: Option<&mut c_void>) {
		unsafe {
			ImDrawList_AddCallback(self, callback, transmute(callback_data))
		}
	}

	pub fn add_draw_cmd(&mut self) {
		unsafe {
			ImDrawList_AddDrawCmd(self)
		}
	}

	pub fn prim_reserve(&mut self, idx_count: i32, vtx_count: i32) {
		unsafe {
			ImDrawList_PrimReserve(self, idx_count, vtx_count)
		}
	}

	pub fn prim_rect(&mut self, a: ImVec2, b: ImVec2, col: u32) {
		unsafe {
			ImDrawList_PrimRect(self, a, b, col)
		}
	}

	pub fn prim_rect_uv(&mut self, a: ImVec2, b: ImVec2, uv_a: ImVec2, uv_b: ImVec2, col: u32) {
		unsafe {
			ImDrawList_PrimRectUV(self, a, b, uv_a, uv_b, col)
		}
	}

	pub fn primp_quad_uv(&mut self, a: ImVec2, b: ImVec2, c: ImVec2, d: ImVec2, uv_a: ImVec2, uv_b: ImVec2, uv_c:ImVec2, uv_d: ImVec2, col: u32) {
		unsafe {
			ImDrawList_PrimQuadUV(self, a, b, c, d, uv_a, uv_b, uv_c, uv_d, col)
		}
	}

	pub fn prim_vtx(&mut self, pos: ImVec2, uv: ImVec2, col: u32) {
		unsafe {
			ImDrawList_PrimVtx(self, pos, uv, col)
		}
	}

	pub fn prim_write_vtx(&mut self, pos: ImVec2, uv: ImVec2, col: u32) {
		unsafe {
			ImDrawList_PrimWriteVtx(self, pos, uv, col)
		}
	}

	pub fn prim_write_idx(&mut self, idx: ImDrawIdx) {
		unsafe {
			ImDrawList_PrimWriteIdx(self, idx)
		}
	}

	pub fn update_clip_rect(&mut self) {
		unsafe {
			ImDrawList_UpdateClipRect(self)
		}
	}

	pub fn update_texture_id(&mut self) {
		unsafe {
			ImDrawList_UpdateTextureID(self)
		}
	}
}


/*
#TODO
Missing:
igCombo2
igCombo3

igSliderFloat2
igSliderFloat3
igSliderFloat4

igSliderInt2
igSliderInt3
igSliderInt4

igDragFloat2
igDragFloat3
igDragFloat4

igDragInt2
igDragInt3
igDragInt4

igInputFloat2
igInputFloat3
igInputFloat4

igInputInt2
igInputInt3
igInputInt4
*/
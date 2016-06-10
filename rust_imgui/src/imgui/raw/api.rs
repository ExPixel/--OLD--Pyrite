use super::types::*;
use libc;
// use va_list::va_list;

#[allow(improper_ctypes)] // va_list is zero sized here :\
extern {
	pub fn igGetIO() -> *mut ImGuiIO;
	pub fn igGetStyle() -> *mut ImGuiStyle;
	pub fn igGetDrawData() -> *mut ImDrawData;
	pub fn igNewFrame();
	pub fn igRender();
	pub fn igShutdown();
	pub fn igShowUserGuide();
	pub fn igShowStyleEditor(_ref: *mut ImGuiStyle);
	pub fn igShowTestWindow(opened: *mut u8);
	pub fn igShowMetricsWindow(opened: *mut u8);
	pub fn igBegin(name: *const libc::c_char, p_opened: *mut u8,
				   flags: ImGuiWindowFlags_::Type) -> u8;
	pub fn igBegin2(name: *const libc::c_char, p_opened: *mut u8,
					size_on_first_use: ImVec2,
					bg_alpha: libc::c_float,
					flags: ImGuiWindowFlags_::Type) -> u8;
	pub fn igEnd();
	pub fn igBeginChild(str_id: *const libc::c_char,
						size: ImVec2, border: u8,
						extra_flags: ImGuiWindowFlags_::Type) -> u8;
	pub fn igBeginChildEx(id: ImGuiID, size: ImVec2, border: u8,
						  extra_flags: ImGuiWindowFlags_::Type) -> u8;
	pub fn igEndChild();
	pub fn igGetContentRegionMax(out: *mut ImVec2);
	pub fn igGetContentRegionAvail(out: *mut ImVec2);
	pub fn igGetContentRegionAvailWidth() -> libc::c_float;
	pub fn igGetWindowContentRegionMin(out: *mut ImVec2);
	pub fn igGetWindowContentRegionMax(out: *mut ImVec2);
	pub fn igGetWindowContentRegionWidth() -> libc::c_float;
	pub fn igGetWindowDrawList() -> *mut ImDrawList;
	pub fn igGetWindowPos(out: *mut ImVec2);
	pub fn igGetWindowSize(out: *mut ImVec2);
	pub fn igGetWindowWidth() -> libc::c_float;
	pub fn igGetWindowHeight() -> libc::c_float;
	pub fn igIsWindowCollapsed() -> u8;
	pub fn igSetWindowFontScale(scale: libc::c_float);
	pub fn igSetNextWindowPos(pos: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetNextWindowPosCenter(cond: ImGuiSetCond_::Type);
	pub fn igSetNextWindowSize(size: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetNextWindowContentSize(size: ImVec2);
	pub fn igSetNextWindowContentWidth(width: libc::c_float);
	pub fn igSetNextWindowCollapsed(collapsed: u8, cond: ImGuiSetCond_::Type);
	pub fn igSetNextWindowFocus();
	pub fn igSetWindowPos(pos: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowSize(size: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowCollapsed(collapsed: u8, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowFocus();
	pub fn igSetWindowPosByName(name: *const libc::c_char,
								pos: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowSize2(name: *const libc::c_char,
							size: ImVec2, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowCollapsed2(name: *const libc::c_char,
								 collapsed: u8, cond: ImGuiSetCond_::Type);
	pub fn igSetWindowFocus2(name: *const libc::c_char);
	pub fn igGetScrollX() -> libc::c_float;
	pub fn igGetScrollY() -> libc::c_float;
	pub fn igGetScrollMaxX() -> libc::c_float;
	pub fn igGetScrollMaxY() -> libc::c_float;
	pub fn igSetScrollX(scroll_x: libc::c_float);
	pub fn igSetScrollY(scroll_y: libc::c_float);
	pub fn igSetScrollHere(center_y_ratio: libc::c_float);
	pub fn igSetScrollFromPosY(pos_y: libc::c_float,
							   center_y_ratio: libc::c_float);
	pub fn igSetKeyboardFocusHere(offset: libc::c_int);
	pub fn igSetStateStorage(tree: *mut ImGuiStorage);
	pub fn igGetStateStorage() -> *mut ImGuiStorage;
	pub fn igPushFont(font: *mut ImFont);
	pub fn igPopFont();
	pub fn igPushStyleColor(idx: ImGuiCol_::Type, col: ImVec4);
	pub fn igPopStyleColor(count: libc::c_int);
	pub fn igPushStyleVar(idx: ImGuiStyleVar_::Type, val: libc::c_float);
	pub fn igPushStyleVarVec(idx: ImGuiStyleVar_::Type, val: ImVec2);
	pub fn igPopStyleVar(count: libc::c_int);
	pub fn igGetFont() -> *mut ImFont;
	pub fn igGetFontSize() -> libc::c_float;
	pub fn igGetFontTexUvWhitePixel(pOut: *mut ImVec2);
	pub fn igGetColorU32(idx: ImGuiCol_::Type, alpha_mul: libc::c_float)
	 -> ImU32;
	pub fn igGetColorU32Vec(col: *const ImVec4) -> ImU32;
	pub fn igPushItemWidth(item_width: libc::c_float);
	pub fn igPopItemWidth();
	pub fn igCalcItemWidth() -> libc::c_float;
	pub fn igPushTextWrapPos(wrap_pos_x: libc::c_float);
	pub fn igPopTextWrapPos();
	pub fn igPushAllowKeyboardFocus(v: u8);
	pub fn igPopAllowKeyboardFocus();
	pub fn igPushButtonRepeat(repeat: u8);
	pub fn igPopButtonRepeat();
	pub fn igBeginGroup();
	pub fn igEndGroup();
	pub fn igSeparator();
	pub fn igSameLine(pos_x: libc::c_float,
					  spacing_w: libc::c_float);
	pub fn igSpacing();
	pub fn igDummy(size: *const ImVec2);
	pub fn igIndent();
	pub fn igUnindent();
	pub fn igGetCursorPos(pOut: *mut ImVec2);
	pub fn igGetCursorPosX() -> libc::c_float;
	pub fn igGetCursorPosY() -> libc::c_float;
	pub fn igSetCursorPos(local_pos: ImVec2);
	pub fn igSetCursorPosX(x: libc::c_float);
	pub fn igSetCursorPosY(y: libc::c_float);
	pub fn igGetCursorStartPos(pOut: *mut ImVec2);
	pub fn igGetCursorScreenPos(pOut: *mut ImVec2);
	pub fn igSetCursorScreenPos(pos: ImVec2);
	pub fn igAlignFirstTextHeightToWidgets();
	pub fn igGetTextLineHeight() -> libc::c_float;
	pub fn igGetTextLineHeightWithSpacing() -> libc::c_float;
	pub fn igGetItemsLineHeightWithSpacing() -> libc::c_float;
	pub fn igColumns(count: libc::c_int,
					 id: *const libc::c_char, border: u8);
	pub fn igNextColumn();
	pub fn igGetColumnIndex() -> libc::c_int;
	pub fn igGetColumnOffset(column_index: libc::c_int)
	 -> libc::c_float;
	pub fn igSetColumnOffset(column_index: libc::c_int,
							 offset_x: libc::c_float);
	pub fn igGetColumnWidth(column_index: libc::c_int)
	 -> libc::c_float;
	pub fn igGetColumnsCount() -> libc::c_int;
	pub fn igPushIdStr(str_id: *const libc::c_char);
	pub fn igPushIdStrRange(str_begin: *const libc::c_char,
							str_end: *const libc::c_char);
	pub fn igPushIdPtr(ptr_id: *const libc::c_void);
	pub fn igPushIdInt(int_id: libc::c_int);
	pub fn igPopId();
	pub fn igGetIdStr(str_id: *const libc::c_char) -> ImGuiID;
	pub fn igGetIdStrRange(str_begin: *const libc::c_char,
						   str_end: *const libc::c_char) -> ImGuiID;
	pub fn igGetIdPtr(ptr_id: *const libc::c_void) -> ImGuiID;
	pub fn igText(fmt: *const libc::c_char, ...);
	// pub fn igTextV(fmt: *const libc::c_char, args: va_list);
	pub fn igTextColored(col: ImVec4,
						 fmt: *const libc::c_char, ...);
	// pub fn igTextColoredV(col: ImVec4,
	// 					  fmt: *const libc::c_char, args: va_list);
	pub fn igTextDisabled(fmt: *const libc::c_char, ...);
	// pub fn igTextDisabledV(fmt: *const libc::c_char, args: va_list);
	pub fn igTextWrapped(fmt: *const libc::c_char, ...);
	// pub fn igTextWrappedV(fmt: *const libc::c_char, args: va_list);
	pub fn igTextUnformatted(text: *const libc::c_char,
							 text_end: *const libc::c_char);
	pub fn igLabelText(label: *const libc::c_char,
					   fmt: *const libc::c_char, ...);
	// pub fn igLabelTextV(label: *const libc::c_char,
	// 					fmt: *const libc::c_char, args: va_list);
	pub fn igBullet();
	pub fn igBulletText(fmt: *const libc::c_char, ...);
	// pub fn igBulletTextV(fmt: *const libc::c_char, args: va_list);
	pub fn igButton(label: *const libc::c_char, size: ImVec2)
	 -> u8;
	pub fn igSmallButton(label: *const libc::c_char) -> u8;
	pub fn igInvisibleButton(str_id: *const libc::c_char,
							 size: ImVec2) -> u8;
	pub fn igImage(user_texture_id: ImTextureID, size: ImVec2,
				   uv0: ImVec2, uv1: ImVec2,
				   tint_col: ImVec4, border_col: ImVec4);
	pub fn igImageButton(user_texture_id: ImTextureID, size: ImVec2,
						 uv0: ImVec2, uv1: ImVec2,
						 frame_padding: libc::c_int,
						 bg_col: ImVec4, tint_col: ImVec4)
	 -> u8;
	pub fn igCollapsingHeader(label: *const libc::c_char,
							  str_id: *const libc::c_char,
							  display_frame: u8, default_open: u8) -> u8;
	pub fn igCheckbox(label: *const libc::c_char, v: *mut u8) -> u8;
	pub fn igCheckboxFlags(label: *const libc::c_char,
						   flags: *mut libc::c_uint,
						   flags_value: libc::c_uint) -> u8;
	pub fn igRadioButtonBool(label: *const libc::c_char, active: u8)
	 -> u8;
	pub fn igRadioButton(label: *const libc::c_char,
						 v: *mut libc::c_int,
						 v_button: libc::c_int) -> u8;
	pub fn igCombo(label: *const libc::c_char,
				   current_item: *mut libc::c_int,
				   items: *mut *const libc::c_char,
				   items_count: libc::c_int,
				   height_in_items: libc::c_int) -> u8;
	pub fn igCombo2(label: *const libc::c_char,
					current_item: *mut libc::c_int,
					items_separated_by_zeros: *const libc::c_char,
					height_in_items: libc::c_int) -> u8;
	pub fn igCombo3(label: *const libc::c_char,
					current_item: *mut libc::c_int,
					items_getter:
						::std::option::Option<unsafe extern "C" fn(data:
																	   *mut libc::c_void,
																   idx:
																	   libc::c_int,
																   out_text:
																	   *mut *const libc::c_char)
												  -> u8>,
					data: *mut libc::c_void,
					items_count: libc::c_int,
					height_in_items: libc::c_int) -> u8;
	pub fn igColorButton(col: ImVec4, small_height: u8,
						 outline_border: u8) -> u8;
	pub fn igColorEdit3(label: *const libc::c_char,
						col: *mut libc::c_float) -> u8;
	pub fn igColorEdit4(label: *const libc::c_char,
						col: *mut libc::c_float, show_alpha: u8)
	 -> u8;
	pub fn igColorEditMode(mode: ImGuiColorEditMode_::Type);
	pub fn igPlotLines(label: *const libc::c_char,
					   values: *const libc::c_float,
					   values_count: libc::c_int,
					   values_offset: libc::c_int,
					   overlay_text: *const libc::c_char,
					   scale_min: libc::c_float,
					   scale_max: libc::c_float,
					   graph_size: ImVec2,
					   stride: libc::c_int);
	pub fn igPlotLines2(label: *const libc::c_char,
						values_getter:
							::std::option::Option<unsafe extern "C" fn(data:
																		   *mut libc::c_void,
																	   idx:
																		   libc::c_int)
													  ->
														  libc::c_float>,
						data: *mut libc::c_void,
						values_count: libc::c_int,
						values_offset: libc::c_int,
						overlay_text: *const libc::c_char,
						scale_min: libc::c_float,
						scale_max: libc::c_float,
						graph_size: ImVec2);
	pub fn igPlotHistogram(label: *const libc::c_char,
						   values: *const libc::c_float,
						   values_count: libc::c_int,
						   values_offset: libc::c_int,
						   overlay_text: *const libc::c_char,
						   scale_min: libc::c_float,
						   scale_max: libc::c_float,
						   graph_size: ImVec2,
						   stride: libc::c_int);
	pub fn igPlotHistogram2(label: *const libc::c_char,
							values_getter:
								::std::option::Option<unsafe extern "C" fn(data:
																			   *mut libc::c_void,
																		   idx:
																			   libc::c_int)
														  ->
															  libc::c_float>,
							data: *mut libc::c_void,
							values_count: libc::c_int,
							values_offset: libc::c_int,
							overlay_text: *const libc::c_char,
							scale_min: libc::c_float,
							scale_max: libc::c_float,
							graph_size: ImVec2);
	pub fn igProgressBar(fraction: libc::c_float,
						 size_arg: *const ImVec2,
						 overlay: *const libc::c_char);
	pub fn igSliderFloat(label: *const libc::c_char,
						 v: *mut libc::c_float,
						 v_min: libc::c_float,
						 v_max: libc::c_float,
						 display_format: *const libc::c_char,
						 power: libc::c_float) -> u8;
	pub fn igSliderFloat2(label: *const libc::c_char,
						  v: *mut libc::c_float,
						  v_min: libc::c_float,
						  v_max: libc::c_float,
						  display_format: *const libc::c_char,
						  power: libc::c_float) -> u8;
	pub fn igSliderFloat3(label: *const libc::c_char,
						  v: *mut libc::c_float,
						  v_min: libc::c_float,
						  v_max: libc::c_float,
						  display_format: *const libc::c_char,
						  power: libc::c_float) -> u8;
	pub fn igSliderFloat4(label: *const libc::c_char,
						  v: *mut libc::c_float,
						  v_min: libc::c_float,
						  v_max: libc::c_float,
						  display_format: *const libc::c_char,
						  power: libc::c_float) -> u8;
	pub fn igSliderAngle(label: *const libc::c_char,
						 v_rad: *mut libc::c_float,
						 v_degrees_min: libc::c_float,
						 v_degrees_max: libc::c_float) -> u8;
	pub fn igSliderInt(label: *const libc::c_char,
					   v: *mut libc::c_int,
					   v_min: libc::c_int,
					   v_max: libc::c_int,
					   display_format: *const libc::c_char) -> u8;
	pub fn igSliderInt2(label: *const libc::c_char,
						v: *mut libc::c_int,
						v_min: libc::c_int,
						v_max: libc::c_int,
						display_format: *const libc::c_char) -> u8;
	pub fn igSliderInt3(label: *const libc::c_char,
						v: *mut libc::c_int,
						v_min: libc::c_int,
						v_max: libc::c_int,
						display_format: *const libc::c_char) -> u8;
	pub fn igSliderInt4(label: *const libc::c_char,
						v: *mut libc::c_int,
						v_min: libc::c_int,
						v_max: libc::c_int,
						display_format: *const libc::c_char) -> u8;
	pub fn igVSliderFloat(label: *const libc::c_char,
						  size: ImVec2,
						  v: *mut libc::c_float,
						  v_min: libc::c_float,
						  v_max: libc::c_float,
						  display_format: *const libc::c_char,
						  power: libc::c_float) -> u8;
	pub fn igVSliderInt(label: *const libc::c_char,
						size: ImVec2, v: *mut libc::c_int,
						v_min: libc::c_int,
						v_max: libc::c_int,
						display_format: *const libc::c_char) -> u8;
	pub fn igDragFloat(label: *const libc::c_char,
					   v: *mut libc::c_float,
					   v_speed: libc::c_float,
					   v_min: libc::c_float,
					   v_max: libc::c_float,
					   display_format: *const libc::c_char,
					   power: libc::c_float) -> u8;
	pub fn igDragFloat2(label: *const libc::c_char,
						v: *mut libc::c_float,
						v_speed: libc::c_float,
						v_min: libc::c_float,
						v_max: libc::c_float,
						display_format: *const libc::c_char,
						power: libc::c_float) -> u8;
	pub fn igDragFloat3(label: *const libc::c_char,
						v: *mut libc::c_float,
						v_speed: libc::c_float,
						v_min: libc::c_float,
						v_max: libc::c_float,
						display_format: *const libc::c_char,
						power: libc::c_float) -> u8;
	pub fn igDragFloat4(label: *const libc::c_char,
						v: *mut libc::c_float,
						v_speed: libc::c_float,
						v_min: libc::c_float,
						v_max: libc::c_float,
						display_format: *const libc::c_char,
						power: libc::c_float) -> u8;
	pub fn igDragFloatRange2(label: *const libc::c_char,
							 v_current_min: *mut libc::c_float,
							 v_current_max: *mut libc::c_float,
							 v_speed: libc::c_float,
							 v_min: libc::c_float,
							 v_max: libc::c_float,
							 display_format: *const libc::c_char,
							 display_format_max:
								 *const libc::c_char,
							 power: libc::c_float) -> u8;
	pub fn igDragInt(label: *const libc::c_char,
					 v: *mut libc::c_int,
					 v_speed: libc::c_float,
					 v_min: libc::c_int,
					 v_max: libc::c_int,
					 display_format: *const libc::c_char) -> u8;
	pub fn igDragInt2(label: *const libc::c_char,
					  v: *mut libc::c_int,
					  v_speed: libc::c_float,
					  v_min: libc::c_int,
					  v_max: libc::c_int,
					  display_format: *const libc::c_char) -> u8;
	pub fn igDragInt3(label: *const libc::c_char,
					  v: *mut libc::c_int,
					  v_speed: libc::c_float,
					  v_min: libc::c_int,
					  v_max: libc::c_int,
					  display_format: *const libc::c_char) -> u8;
	pub fn igDragInt4(label: *const libc::c_char,
					  v: *mut libc::c_int,
					  v_speed: libc::c_float,
					  v_min: libc::c_int,
					  v_max: libc::c_int,
					  display_format: *const libc::c_char) -> u8;
	pub fn igDragIntRange2(label: *const libc::c_char,
						   v_current_min: *mut libc::c_int,
						   v_current_max: *mut libc::c_int,
						   v_speed: libc::c_float,
						   v_min: libc::c_int,
						   v_max: libc::c_int,
						   display_format: *const libc::c_char,
						   display_format_max: *const libc::c_char)
	 -> u8;
	pub fn igInputText(label: *const libc::c_char,
					   buf: *mut libc::c_char, buf_size: libc::size_t,
					   flags: ImGuiInputTextFlags_::Type,
					   callback: ImGuiTextEditCallback,
					   user_data: *mut libc::c_void) -> u8;
	pub fn igInputTextMultiline(label: *const libc::c_char,
								buf: *mut libc::c_char,
								buf_size: libc::size_t, size: ImVec2,
								flags: ImGuiInputTextFlags_::Type,
								callback: ImGuiTextEditCallback,
								user_data: *mut libc::c_void) -> u8;
	pub fn igInputFloat(label: *const libc::c_char,
						v: *mut libc::c_float,
						step: libc::c_float,
						step_fast: libc::c_float,
						decimal_precision: libc::c_int,
						extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputFloat2(label: *const libc::c_char,
						 v: *mut libc::c_float,
						 decimal_precision: libc::c_int,
						 extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputFloat3(label: *const libc::c_char,
						 v: *mut libc::c_float,
						 decimal_precision: libc::c_int,
						 extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputFloat4(label: *const libc::c_char,
						 v: *mut libc::c_float,
						 decimal_precision: libc::c_int,
						 extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputInt(label: *const libc::c_char,
					  v: *mut libc::c_int,
					  step: libc::c_int,
					  step_fast: libc::c_int,
					  extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputInt2(label: *const libc::c_char,
					   v: *mut libc::c_int,
					   extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputInt3(label: *const libc::c_char,
					   v: *mut libc::c_int,
					   extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igInputInt4(label: *const libc::c_char,
					   v: *mut libc::c_int,
					   extra_flags: ImGuiInputTextFlags_::Type) -> u8;
	pub fn igTreeNode(str_label_id: *const libc::c_char) -> u8;
	pub fn igTreeNodeStr(str_id: *const libc::c_char,
						 fmt: *const libc::c_char, ...) -> u8;
	pub fn igTreeNodePtr(ptr_id: *const libc::c_void,
						 fmt: *const libc::c_char, ...) -> u8;
	// pub fn igTreeNodeStrV(str_id: *const libc::c_char,
	// 					  fmt: *const libc::c_char, args: va_list)
	//  -> u8;
	// pub fn igTreeNodePtrV(ptr_id: *const libc::c_void,
	// 					  fmt: *const libc::c_char, args: va_list)
	//  -> u8;
	pub fn igTreePushStr(str_id: *const libc::c_char);
	pub fn igTreePushPtr(ptr_id: *const libc::c_void);
	pub fn igTreePop();
	pub fn igSetNextTreeNodeOpened(opened: u8, cond: ImGuiSetCond_::Type);
	pub fn igSelectable(label: *const libc::c_char, selected: u8,
						flags: ImGuiSelectableFlags_::Type, size: ImVec2) -> u8;
	pub fn igSelectableEx(label: *const libc::c_char,
						  p_selected: *mut u8, flags: ImGuiSelectableFlags_::Type,
						  size: ImVec2) -> u8;
	pub fn igListBox(label: *const libc::c_char,
					 current_item: *mut libc::c_int,
					 items: *mut *const libc::c_char,
					 items_count: libc::c_int,
					 height_in_items: libc::c_int) -> u8;
	pub fn igListBox2(label: *const libc::c_char,
					  current_item: *mut libc::c_int,
					  items_getter:
						  ::std::option::Option<unsafe extern "C" fn(data:
																		 *mut libc::c_void,
																	 idx:
																		 libc::c_int,
																	 out_text:
																		 *mut *const libc::c_char)
													-> u8>,
					  data: *mut libc::c_void,
					  items_count: libc::c_int,
					  height_in_items: libc::c_int) -> u8;
	pub fn igListBoxHeader(label: *const libc::c_char,
						   size: ImVec2) -> u8;
	pub fn igListBoxHeader2(label: *const libc::c_char,
							items_count: libc::c_int,
							height_in_items: libc::c_int) -> u8;
	pub fn igListBoxFooter();
	pub fn igValueBool(prefix: *const libc::c_char, b: u8);
	pub fn igValueInt(prefix: *const libc::c_char,
					  v: libc::c_int);
	pub fn igValueUInt(prefix: *const libc::c_char,
					   v: libc::c_uint);
	pub fn igValueFloat(prefix: *const libc::c_char,
						v: libc::c_float,
						float_format: *const libc::c_char);
	pub fn igValueColor(prefix: *const libc::c_char,
						v: ImVec4);
	pub fn igValueColor2(prefix: *const libc::c_char,
						 v: libc::c_uint);
	pub fn igSetTooltip(fmt: *const libc::c_char, ...);
	// pub fn igSetTooltipV(fmt: *const libc::c_char, args: va_list);
	pub fn igBeginTooltip();
	pub fn igEndTooltip();
	pub fn igBeginMainMenuBar() -> u8;
	pub fn igEndMainMenuBar();
	pub fn igBeginMenuBar() -> u8;
	pub fn igEndMenuBar();
	pub fn igBeginMenu(label: *const libc::c_char, enabled: u8)
	 -> u8;
	pub fn igEndMenu();
	pub fn igMenuItem(label: *const libc::c_char,
					  shortcut: *const libc::c_char, selected: u8,
					  enabled: u8) -> u8;
	pub fn igMenuItemPtr(label: *const libc::c_char,
						 shortcut: *const libc::c_char,
						 p_selected: *mut u8, enabled: u8) -> u8;
	pub fn igOpenPopup(str_id: *const libc::c_char);
	pub fn igBeginPopup(str_id: *const libc::c_char) -> u8;
	pub fn igBeginPopupModal(name: *const libc::c_char,
							 p_opened: *mut u8, extra_flags: ImGuiWindowFlags_::Type)
	 -> u8;
	pub fn igBeginPopupContextItem(str_id: *const libc::c_char,
								   mouse_button: libc::c_int) -> u8;
	pub fn igBeginPopupContextWindow(also_over_items: u8,
									 str_id: *const libc::c_char,
									 mouse_button: libc::c_int)
	 -> u8;
	pub fn igBeginPopupContextVoid(str_id: *const libc::c_char,
								   mouse_button: libc::c_int) -> u8;
	pub fn igEndPopup();
	pub fn igCloseCurrentPopup();
	pub fn igLogToTTY(max_depth: libc::c_int);
	pub fn igLogToFile(max_depth: libc::c_int,
					   filename: *const libc::c_char);
	pub fn igLogToClipboard(max_depth: libc::c_int);
	pub fn igLogFinish();
	pub fn igLogButtons();
	pub fn igLogText(fmt: *const libc::c_char, ...);
	pub fn igIsItemHovered() -> u8;
	pub fn igIsItemHoveredRect() -> u8;
	pub fn igIsItemActive() -> u8;
	pub fn igIsItemVisible() -> u8;
	pub fn igIsAnyItemHovered() -> u8;
	pub fn igIsAnyItemActive() -> u8;
	pub fn igGetItemRectMin(pOut: *mut ImVec2);
	pub fn igGetItemRectMax(pOut: *mut ImVec2);
	pub fn igGetItemRectSize(pOut: *mut ImVec2);
	pub fn igSetItemAllowOverlap();
	pub fn igIsWindowHovered() -> u8;
	pub fn igIsWindowFocused() -> u8;
	pub fn igIsRootWindowFocused() -> u8;
	pub fn igIsRootWindowOrAnyChildFocused() -> u8;
	pub fn igIsRectVisible(item_size: ImVec2) -> u8;
	pub fn igIsPosHoveringAnyWindow(pos: ImVec2) -> u8;
	pub fn igGetTime() -> libc::c_float;
	pub fn igGetFrameCount() -> libc::c_int;
	pub fn igGetStyleColName(idx: ImGuiCol_::Type) -> *const libc::c_char;
	pub fn igCalcItemRectClosestPoint(pOut: *mut ImVec2,
									  pos: ImVec2, on_edge: u8,
									  outward: libc::c_float);
	pub fn igCalcTextSize(pOut: *mut ImVec2,
						  text: *const libc::c_char,
						  text_end: *const libc::c_char,
						  hide_text_after_double_hash: u8,
						  wrap_width: libc::c_float);
	pub fn igCalcListClipping(items_count: libc::c_int,
							  items_height: libc::c_float,
							  out_items_display_start:
								  *mut libc::c_int,
							  out_items_display_end:
								  *mut libc::c_int);
	pub fn igBeginChildFrame(id: ImGuiID, size: ImVec2,
							 extra_flags: ImGuiWindowFlags_::Type) -> u8;
	pub fn igEndChildFrame();
	pub fn igColorConvertU32ToFloat4(pOut: *mut ImVec4, _in: ImU32);
	pub fn igColorConvertFloat4ToU32(_in: ImVec4) -> ImU32;
	pub fn igColorConvertRGBtoHSV(r: libc::c_float,
								  g: libc::c_float,
								  b: libc::c_float,
								  out_h: *mut libc::c_float,
								  out_s: *mut libc::c_float,
								  out_v: *mut libc::c_float);
	pub fn igColorConvertHSVtoRGB(h: libc::c_float,
								  s: libc::c_float,
								  v: libc::c_float,
								  out_r: *mut libc::c_float,
								  out_g: *mut libc::c_float,
								  out_b: *mut libc::c_float);
	pub fn igGetKeyIndex(key: ImGuiKey_::Type) -> libc::c_int;
	pub fn igIsKeyDown(key_index: libc::c_int) -> u8;
	pub fn igIsKeyPressed(key_index: libc::c_int, repeat: u8) -> u8;
	pub fn igIsKeyReleased(key_index: libc::c_int) -> u8;
	pub fn igIsMouseDown(button: libc::c_int) -> u8;
	pub fn igIsMouseClicked(button: libc::c_int, repeat: u8) -> u8;
	pub fn igIsMouseDoubleClicked(button: libc::c_int) -> u8;
	pub fn igIsMouseReleased(button: libc::c_int) -> u8;
	pub fn igIsMouseHoveringWindow() -> u8;
	pub fn igIsMouseHoveringAnyWindow() -> u8;
	pub fn igIsMouseHoveringRect(r_min: ImVec2, r_max: ImVec2,
								 clip: u8) -> u8;
	pub fn igIsMouseDragging(button: libc::c_int,
							 lock_threshold: libc::c_float) -> u8;
	pub fn igGetMousePos(pOut: *mut ImVec2);
	pub fn igGetMousePosOnOpeningCurrentPopup(pOut: *mut ImVec2);
	pub fn igGetMouseDragDelta(pOut: *mut ImVec2,
							   button: libc::c_int,
							   lock_threshold: libc::c_float);
	pub fn igResetMouseDragDelta(button: libc::c_int);
	pub fn igGetMouseCursor() -> ImGuiMouseCursor_::Type;
	pub fn igSetMouseCursor(_type: ImGuiMouseCursor_::Type);
	pub fn igCaptureKeyboardFromApp(capture: u8);
	pub fn igCaptureMouseFromApp(capture: u8);
	pub fn igMemAlloc(sz: libc::size_t) -> *mut libc::c_void;
	pub fn igMemFree(ptr: *mut libc::c_void);
	pub fn igGetClipboardText() -> *const libc::c_char;
	pub fn igSetClipboardText(text: *const libc::c_char);
	pub fn igGetVersion() -> *const libc::c_char;
	pub fn igGetInternalState() -> *mut libc::c_void;
	pub fn igGetInternalStateSize() -> libc::size_t;
	pub fn igSetInternalState(state: *mut libc::c_void,
							  construct: u8);
	pub fn ImFontConfig_DefaultConstructor(config: *mut ImFontConfig);
	pub fn ImFontAtlas_GetTexDataAsRGBA32(atlas: *mut ImFontAtlas,
										  out_pixels:
											  *mut *mut libc::c_uchar,
										  out_width:
											  *mut libc::c_int,
										  out_height:
											  *mut libc::c_int,
										  out_bytes_per_pixel:
											  *mut libc::c_int);
	pub fn ImFontAtlas_GetTexDataAsAlpha8(atlas: *mut ImFontAtlas,
										  out_pixels:
											  *mut *mut libc::c_uchar,
										  out_width:
											  *mut libc::c_int,
										  out_height:
											  *mut libc::c_int,
										  out_bytes_per_pixel:
											  *mut libc::c_int);
	pub fn ImFontAtlas_SetTexID(atlas: *mut ImFontAtlas,
								tex: *mut libc::c_void);
	pub fn ImFontAtlas_AddFont(atlas: *mut ImFontAtlas,
							   font_cfg: *const ImFontConfig) -> *mut ImFont;
	pub fn ImFontAtlas_AddFontDefault(atlas: *mut ImFontAtlas,
									  font_cfg: *const ImFontConfig)
	 -> *mut ImFont;
	pub fn ImFontAtlas_AddFontFromFileTTF(atlas: *mut ImFontAtlas,
										  filename:
											  *const libc::c_char,
										  size_pixels:
											  libc::c_float,
										  font_cfg: *const ImFontConfig,
										  glyph_ranges: *const ImWchar)
	 -> *mut ImFont;
	pub fn ImFontAtlas_AddFontFromMemoryTTF(atlas: *mut ImFontAtlas,
											ttf_data:
												*mut libc::c_void,
											ttf_size: libc::c_int,
											size_pixels:
												libc::c_float,
											font_cfg: *const ImFontConfig,
											glyph_ranges: *const ImWchar)
	 -> *mut ImFont;
	pub fn ImFontAtlas_AddFontFromMemoryCompressedTTF(atlas: *mut ImFontAtlas,
													  compressed_ttf_data:
														  *const libc::c_void,
													  compressed_ttf_size:
														  libc::c_int,
													  size_pixels:
														  libc::c_float,
													  font_cfg:
														  *const ImFontConfig,
													  glyph_ranges:
														  *const ImWchar)
	 -> *mut ImFont;
	pub fn ImFontAtlas_AddFontFromMemoryCompressedBase85TTF(atlas:
																*mut ImFontAtlas,
															compressed_ttf_data_base85:
																*const libc::c_char,
															size_pixels:
																libc::c_float,
															font_cfg:
																*const ImFontConfig,
															glyph_ranges:
																*const ImWchar)
	 -> *mut ImFont;
	pub fn ImFontAtlas_ClearTexData(atlas: *mut ImFontAtlas);
	pub fn ImFontAtlas_Clear(atlas: *mut ImFontAtlas);
	pub fn ImGuiIO_AddInputCharacter(c: libc::c_ushort);
	pub fn ImGuiIO_AddInputCharactersUTF8(utf8_chars:
											  *const libc::c_char);
	pub fn ImGuiIO_ClearInputCharacters();
	pub fn ImDrawData_DeIndexAllBuffers(drawData: *mut ImDrawData);
	pub fn ImDrawList_GetVertexBufferSize(list: *mut ImDrawList)
	 -> libc::c_int;
	pub fn ImDrawList_GetVertexPtr(list: *mut ImDrawList,
								   n: libc::c_int)
	 -> *mut ImDrawVert;
	pub fn ImDrawList_GetIndexBufferSize(list: *mut ImDrawList)
	 -> libc::c_int;
	pub fn ImDrawList_GetIndexPtr(list: *mut ImDrawList,
								  n: libc::c_int) -> *mut ImDrawIdx;
	pub fn ImDrawList_GetCmdSize(list: *mut ImDrawList)
	 -> libc::c_int;
	pub fn ImDrawList_GetCmdPtr(list: *mut ImDrawList,
								n: libc::c_int) -> *mut ImDrawCmd;
	pub fn ImDrawList_Clear(list: *mut ImDrawList);
	pub fn ImDrawList_ClearFreeMemory(list: *mut ImDrawList);
	pub fn ImDrawList_PushClipRect(list: *mut ImDrawList,
								   clip_rect: ImVec4);
	pub fn ImDrawList_PushClipRectFullScreen(list: *mut ImDrawList);
	pub fn ImDrawList_PopClipRect(list: *mut ImDrawList);
	pub fn ImDrawList_PushTextureID(list: *mut ImDrawList,
									texture_id: ImTextureID);
	pub fn ImDrawList_PopTextureID(list: *mut ImDrawList);
	pub fn ImDrawList_AddLine(list: *mut ImDrawList, a: ImVec2,
							  b: ImVec2, col: ImU32,
							  thickness: libc::c_float);
	pub fn ImDrawList_AddRect(list: *mut ImDrawList, a: ImVec2,
							  b: ImVec2, col: ImU32,
							  rounding: libc::c_float,
							  rounding_corners: libc::c_int,
							  thickness: libc::c_float);
	pub fn ImDrawList_AddRectFilled(list: *mut ImDrawList, a: ImVec2,
									b: ImVec2, col: ImU32,
									rounding: libc::c_float,
									rounding_corners: libc::c_int);
	pub fn ImDrawList_AddRectFilledMultiColor(list: *mut ImDrawList,
											  a: ImVec2,
											  b: ImVec2,
											  col_upr_left: ImU32,
											  col_upr_right: ImU32,
											  col_bot_right: ImU32,
											  col_bot_left: ImU32);
	pub fn ImDrawList_AddTriangle(list: *mut ImDrawList, a: ImVec2,
								  b: ImVec2, c: ImVec2,
								  col: ImU32,
								  thickness: libc::c_float);
	pub fn ImDrawList_AddTriangleFilled(list: *mut ImDrawList,
										a: ImVec2, b: ImVec2,
										c: ImVec2, col: ImU32);
	pub fn ImDrawList_AddCircle(list: *mut ImDrawList, centre: ImVec2,
								radius: libc::c_float, col: ImU32,
								num_segments: libc::c_int,
								thickness: libc::c_float);
	pub fn ImDrawList_AddCircleFilled(list: *mut ImDrawList,
									  centre: ImVec2,
									  radius: libc::c_float,
									  col: ImU32,
									  num_segments: libc::c_int);
	pub fn ImDrawList_AddText(list: *mut ImDrawList, pos: ImVec2,
							  col: ImU32,
							  text_begin: *const libc::c_char,
							  text_end: *const libc::c_char);
	pub fn ImDrawList_AddTextExt(list: *mut ImDrawList, font: *const ImFont,
								 font_size: libc::c_float,
								 pos: ImVec2, col: ImU32,
								 text_begin: *const libc::c_char,
								 text_end: *const libc::c_char,
								 wrap_width: libc::c_float,
								 cpu_fine_clip_rect: *const ImVec4);
	pub fn ImDrawList_AddImage(list: *mut ImDrawList,
							   user_texture_id: ImTextureID, a: ImVec2,
							   b: ImVec2, uv0: ImVec2,
							   uv1: ImVec2, col: ImU32);
	pub fn ImDrawList_AddPolyline(list: *mut ImDrawList,
								  points: *const ImVec2,
								  num_points: libc::c_int,
								  col: ImU32, closed: u8,
								  thickness: libc::c_float,
								  anti_aliased: u8);
	pub fn ImDrawList_AddConvexPolyFilled(list: *mut ImDrawList,
										  points: *const ImVec2,
										  num_points: libc::c_int,
										  col: ImU32, anti_aliased: u8);
	pub fn ImDrawList_AddBezierCurve(list: *mut ImDrawList,
									 pos0: ImVec2, cp0: ImVec2,
									 cp1: ImVec2, pos1: ImVec2,
									 col: ImU32,
									 thickness: libc::c_float,
									 num_segments: libc::c_int);
	pub fn ImDrawList_PathClear(list: *mut ImDrawList);
	pub fn ImDrawList_PathLineTo(list: *mut ImDrawList, pos: ImVec2);
	pub fn ImDrawList_PathLineToMergeDuplicate(list: *mut ImDrawList,
											   pos: ImVec2);
	pub fn ImDrawList_PathFill(list: *mut ImDrawList, col: ImU32);
	pub fn ImDrawList_PathStroke(list: *mut ImDrawList, col: ImU32,
								 closed: u8,
								 thickness: libc::c_float);
	pub fn ImDrawList_PathArcTo(list: *mut ImDrawList, centre: ImVec2,
								radius: libc::c_float,
								a_min: libc::c_float,
								a_max: libc::c_float,
								num_segments: libc::c_int);
	pub fn ImDrawList_PathArcToFast(list: *mut ImDrawList,
									centre: ImVec2,
									radius: libc::c_float,
									a_min_of_12: libc::c_int,
									a_max_of_12: libc::c_int);
	pub fn ImDrawList_PathBezierCurveTo(list: *mut ImDrawList,
										p1: ImVec2, p2: ImVec2,
										p3: ImVec2,
										num_segments: libc::c_int);
	pub fn ImDrawList_PathRect(list: *mut ImDrawList, rect_min: ImVec2,
							   rect_max: ImVec2,
							   rounding: libc::c_float,
							   rounding_corners: libc::c_int);
	pub fn ImDrawList_ChannelsSplit(list: *mut ImDrawList,
									channels_count: libc::c_int);
	pub fn ImDrawList_ChannelsMerge(list: *mut ImDrawList);
	pub fn ImDrawList_ChannelsSetCurrent(list: *mut ImDrawList,
										 channel_index:
											 libc::c_int);
	pub fn ImDrawList_AddCallback(list: *mut ImDrawList,
								  callback: ImDrawCallback,
								  callback_data: *mut libc::c_void);
	pub fn ImDrawList_AddDrawCmd(list: *mut ImDrawList);
	pub fn ImDrawList_PrimReserve(list: *mut ImDrawList,
								  idx_count: libc::c_int,
								  vtx_count: libc::c_int);
	pub fn ImDrawList_PrimRect(list: *mut ImDrawList, a: ImVec2,
							   b: ImVec2, col: ImU32);
	pub fn ImDrawList_PrimRectUV(list: *mut ImDrawList, a: ImVec2,
								 b: ImVec2, uv_a: ImVec2,
								 uv_b: ImVec2, col: ImU32);
	pub fn ImDrawList_PrimQuadUV(list: *mut ImDrawList, a: ImVec2,
								 b: ImVec2, c: ImVec2,
								 d: ImVec2, uv_a: ImVec2,
								 uv_b: ImVec2, uv_c: ImVec2,
								 uv_d: ImVec2, col: ImU32);
	pub fn ImDrawList_PrimVtx(list: *mut ImDrawList, pos: ImVec2,
							  uv: ImVec2, col: ImU32);
	pub fn ImDrawList_PrimWriteVtx(list: *mut ImDrawList, pos: ImVec2,
								   uv: ImVec2, col: ImU32);
	pub fn ImDrawList_PrimWriteIdx(list: *mut ImDrawList, idx: ImDrawIdx);
	pub fn ImDrawList_UpdateClipRect(list: *mut ImDrawList);
	pub fn ImDrawList_UpdateTextureID(list: *mut ImDrawList);
}

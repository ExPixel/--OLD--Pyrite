use super::raw::types::*;

bitflags! {
	pub flags ImGuiWindowFlags: ImGuiWindowFlags_::Type {
		const ImGuiWindowFlags_None = 0,
		const ImGuiWindowFlags_NoTitleBar = ImGuiWindowFlags_::NoTitleBar,
		const ImGuiWindowFlags_NoResize = ImGuiWindowFlags_::NoResize,
		const ImGuiWindowFlags_NoMove = ImGuiWindowFlags_::NoMove,
		const ImGuiWindowFlags_NoScrollbar = ImGuiWindowFlags_::NoScrollbar,
		const ImGuiWindowFlags_NoScrollWithMouse = ImGuiWindowFlags_::NoScrollWithMouse,
		const ImGuiWindowFlags_NoCollapse = ImGuiWindowFlags_::NoCollapse,
		const ImGuiWindowFlags_AlwaysAutoResize = ImGuiWindowFlags_::AlwaysAutoResize,
		const ImGuiWindowFlags_ShowBorders = ImGuiWindowFlags_::ShowBorders,
		const ImGuiWindowFlags_NoSavedSettings = ImGuiWindowFlags_::NoSavedSettings,
		const ImGuiWindowFlags_NoInputs = ImGuiWindowFlags_::NoInputs,
		const ImGuiWindowFlags_MenuBar = ImGuiWindowFlags_::MenuBar,
		const ImGuiWindowFlags_HorizontalScrollbar = ImGuiWindowFlags_::HorizontalScrollbar,
		const ImGuiWindowFlags_NoFocusOnAppearing = ImGuiWindowFlags_::NoFocusOnAppearing,
		const ImGuiWindowFlags_NoBringToFrontOnFocus = ImGuiWindowFlags_::NoBringToFrontOnFocus,
		const ImGuiWindowFlags_AlwaysVerticalScrollbar = ImGuiWindowFlags_::AlwaysVerticalScrollbar,
		const ImGuiWindowFlags_AlwaysHorizontalScrollbar = ImGuiWindowFlags_::AlwaysHorizontalScrollbar,
		const ImGuiWindowFlags_AlwaysUseWindowPadding = ImGuiWindowFlags_::AlwaysUseWindowPadding,
		const ImGuiWindowFlags_ChildWindow = ImGuiWindowFlags_::ChildWindow,
		const ImGuiWindowFlags_ChildWindowAutoFitX = ImGuiWindowFlags_::ChildWindowAutoFitX,
		const ImGuiWindowFlags_ChildWindowAutoFitY = ImGuiWindowFlags_::ChildWindowAutoFitY,
		const ImGuiWindowFlags_ComboBox = ImGuiWindowFlags_::ComboBox,
		const ImGuiWindowFlags_Tooltip = ImGuiWindowFlags_::Tooltip,
		const ImGuiWindowFlags_Popup = ImGuiWindowFlags_::Popup,
		const ImGuiWindowFlags_Modal = ImGuiWindowFlags_::Modal,
		const ImGuiWindowFlags_ChildMenu = ImGuiWindowFlags_::ChildMenu,
	}
}

impl ImGuiWindowFlags {
	pub fn as_c(self) -> ImGuiWindowFlags_::Type { self.bits }
}

pub enum ImGuiSetCond {
	Always = ImGuiSetCond_::Always as isize,
	Once = ImGuiSetCond_::Once as isize,
	FirstUseEver = ImGuiSetCond_::FirstUseEver as isize,
	Appearing = ImGuiSetCond_::Appearing as isize,
}

impl ImGuiSetCond {
	pub fn as_c(self) -> ImGuiSetCond_::Type { self as ImGuiSetCond_::Type }
}

pub enum ImGuiCol {
	Text = ImGuiCol_::Text as isize,
	TextDisabled = ImGuiCol_::TextDisabled as isize,
	WindowBg = ImGuiCol_::WindowBg as isize,
	ChildWindowBg = ImGuiCol_::ChildWindowBg as isize,
	PopupBg = ImGuiCol_::PopupBg as isize,
	Border = ImGuiCol_::Border as isize,
	BorderShadow = ImGuiCol_::BorderShadow as isize,
	FrameBg = ImGuiCol_::FrameBg as isize,
	FrameBgHovered = ImGuiCol_::FrameBgHovered as isize,
	FrameBgActive = ImGuiCol_::FrameBgActive as isize,
	TitleBg = ImGuiCol_::TitleBg as isize,
	TitleBgCollapsed = ImGuiCol_::TitleBgCollapsed as isize,
	TitleBgActive = ImGuiCol_::TitleBgActive as isize,
	MenuBarBg = ImGuiCol_::MenuBarBg as isize,
	ScrollbarBg = ImGuiCol_::ScrollbarBg as isize,
	ScrollbarGrab = ImGuiCol_::ScrollbarGrab as isize,
	ScrollbarGrabHovered = ImGuiCol_::ScrollbarGrabHovered as isize,
	ScrollbarGrabActive = ImGuiCol_::ScrollbarGrabActive as isize,
	ComboBg = ImGuiCol_::ComboBg as isize,
	CheckMark = ImGuiCol_::CheckMark as isize,
	SliderGrab = ImGuiCol_::SliderGrab as isize,
	SliderGrabActive = ImGuiCol_::SliderGrabActive as isize,
	Button = ImGuiCol_::Button as isize,
	ButtonHovered = ImGuiCol_::ButtonHovered as isize,
	ButtonActive = ImGuiCol_::ButtonActive as isize,
	Header = ImGuiCol_::Header as isize,
	HeaderHovered = ImGuiCol_::HeaderHovered as isize,
	HeaderActive = ImGuiCol_::HeaderActive as isize,
	Column = ImGuiCol_::Column as isize,
	ColumnHovered = ImGuiCol_::ColumnHovered as isize,
	ColumnActive = ImGuiCol_::ColumnActive as isize,
	ResizeGrip = ImGuiCol_::ResizeGrip as isize,
	ResizeGripHovered = ImGuiCol_::ResizeGripHovered as isize,
	ResizeGripActive = ImGuiCol_::ResizeGripActive as isize,
	CloseButton = ImGuiCol_::CloseButton as isize,
	CloseButtonHovered = ImGuiCol_::CloseButtonHovered as isize,
	CloseButtonActive = ImGuiCol_::CloseButtonActive as isize,
	PlotLines = ImGuiCol_::PlotLines as isize,
	PlotLinesHovered = ImGuiCol_::PlotLinesHovered as isize,
	PlotHistogram = ImGuiCol_::PlotHistogram as isize,
	PlotHistogramHovered = ImGuiCol_::PlotHistogramHovered as isize,
	TextSelectedBg = ImGuiCol_::TextSelectedBg as isize,
	ModalWindowDarkening = ImGuiCol_::ModalWindowDarkening as isize,
}

impl ImGuiCol {
	pub fn as_c(self) -> ImGuiCol_::Type { self as i32 }
}

pub enum ImGuiStyleVar {
	Alpha = ImGuiStyleVar_::Alpha as isize,
	WindowPadding = ImGuiStyleVar_::WindowPadding as isize,
	WindowRounding = ImGuiStyleVar_::WindowRounding as isize,
	WindowMinSize = ImGuiStyleVar_::WindowMinSize as isize,
	ChildWindowRounding = ImGuiStyleVar_::ChildWindowRounding as isize,
	FramePadding = ImGuiStyleVar_::FramePadding as isize,
	FrameRounding = ImGuiStyleVar_::FrameRounding as isize,
	ItemSpacing = ImGuiStyleVar_::ItemSpacing as isize,
	ItemInnerSpacing = ImGuiStyleVar_::ItemInnerSpacing as isize,
	IndentSpacing = ImGuiStyleVar_::IndentSpacing as isize,
	GrabMinSize = ImGuiStyleVar_::GrabMinSize as isize,
}

impl ImGuiStyleVar {
	pub fn as_c(self) -> ImGuiStyleVar_::Type { self as ImGuiStyleVar_::Type }
}

pub enum ImGuiColorEditMode {
	UserSelect = ImGuiColorEditMode_::UserSelect as isize,
	UserSelectShowButton = ImGuiColorEditMode_::UserSelectShowButton as isize,
	RGB = ImGuiColorEditMode_::RGB as isize,
	HSV = ImGuiColorEditMode_::HSV as isize,
	HEX = ImGuiColorEditMode_::HEX as isize,
}

impl ImGuiColorEditMode {
	pub fn as_c(self) -> ImGuiColorEditMode_::Type { self as ImGuiColorEditMode_::Type }
}

// bitflags! {
//     flags Flags: u32 {
//         const FLAG_A       = 0b00000001,
//         const FLAG_B       = 0b00000010,
//         const FLAG_C       = 0b00000100,
//         const FLAG_ABC     = FLAG_A.bits
//                            | FLAG_B.bits
//                            | FLAG_C.bits,
//     }
// }

bitflags! {
	pub flags ImGuiInputTextFlags: ImGuiInputTextFlags_::Type {
		const ImGuiInputTextFlags_CharsDecimal = ImGuiInputTextFlags_::CharsDecimal,
		const ImGuiInputTextFlags_CharsHexadecimal = ImGuiInputTextFlags_::CharsHexadecimal,
		const ImGuiInputTextFlags_CharsUppercase = ImGuiInputTextFlags_::CharsUppercase,
		const ImGuiInputTextFlags_CharsNoBlank = ImGuiInputTextFlags_::CharsNoBlank,
		const ImGuiInputTextFlags_AutoSelectAll = ImGuiInputTextFlags_::AutoSelectAll,
		const ImGuiInputTextFlags_EnterReturnsTrue = ImGuiInputTextFlags_::EnterReturnsTrue,
		const ImGuiInputTextFlags_CallbackCompletion = ImGuiInputTextFlags_::CallbackCompletion,
		const ImGuiInputTextFlags_CallbackHistory = ImGuiInputTextFlags_::CallbackHistory,
		const ImGuiInputTextFlags_CallbackAlways = ImGuiInputTextFlags_::CallbackAlways,
		const ImGuiInputTextFlags_CallbackCharFilter = ImGuiInputTextFlags_::CallbackCharFilter,
		const ImGuiInputTextFlags_AllowTabInput = ImGuiInputTextFlags_::AllowTabInput,
		const ImGuiInputTextFlags_CtrlEnterForNewLine = ImGuiInputTextFlags_::CtrlEnterForNewLine,
		const ImGuiInputTextFlags_NoHorizontalScroll = ImGuiInputTextFlags_::NoHorizontalScroll,
		const ImGuiInputTextFlags_AlwaysInsertMode = ImGuiInputTextFlags_::AlwaysInsertMode,
		const ImGuiInputTextFlags_ReadOnly = ImGuiInputTextFlags_::ReadOnly,
		const ImGuiInputTextFlags_Password = ImGuiInputTextFlags_::Password,
		const ImGuiInputTextFlags_Multiline = ImGuiInputTextFlags_::Multiline,	
	}
}

impl ImGuiInputTextFlags {
	pub fn as_c(self) -> ImGuiInputTextFlags_::Type { self.bits }
}

bitflags! {
	pub flags ImGuiSelectableFlags: ImGuiSelectableFlags_::Type {
		const DontClosePopups = ImGuiSelectableFlags_::DontClosePopups,
		const SpanAllColumns = ImGuiSelectableFlags_::SpanAllColumns,
		const AllowDoubleClick = ImGuiSelectableFlags_::AllowDoubleClick,
	}
}

impl ImGuiSelectableFlags {
	pub fn as_c(self) -> ImGuiSelectableFlags_::Type { self.bits }
}

pub enum ImGuiKey {
	Tab = ImGuiKey_::Tab as isize,
	LeftArrow = ImGuiKey_::LeftArrow as isize,
	RightArrow = ImGuiKey_::RightArrow as isize,
	UpArrow = ImGuiKey_::UpArrow as isize,
	DownArrow = ImGuiKey_::DownArrow as isize,
	PageUp = ImGuiKey_::PageUp as isize,
	PageDown = ImGuiKey_::PageDown as isize,
	Home = ImGuiKey_::Home as isize,
	End = ImGuiKey_::End as isize,
	Delete = ImGuiKey_::Delete as isize,
	Backspace = ImGuiKey_::Backspace as isize,
	Enter = ImGuiKey_::Enter as isize,
	Escape = ImGuiKey_::Escape as isize,
	A = ImGuiKey_::A as isize,
	C = ImGuiKey_::C as isize,
	V = ImGuiKey_::V as isize,
	X = ImGuiKey_::X as isize,
	Y = ImGuiKey_::Y as isize,
	Z = ImGuiKey_::Z as isize,
}

impl ImGuiKey {
	pub fn as_c(self) -> ImGuiKey_::Type { self as ImGuiKey_::Type }
}

pub enum ImGuiMouseCursor {
	Arrow = ImGuiMouseCursor_::Arrow as isize,
	TextInput = ImGuiMouseCursor_::TextInput as isize,
	Move = ImGuiMouseCursor_::Move as isize,
	ResizeNS = ImGuiMouseCursor_::ResizeNS as isize,
	ResizeEW = ImGuiMouseCursor_::ResizeEW as isize,
	ResizeNESW = ImGuiMouseCursor_::ResizeNESW as isize,
	ResizeNWSE = ImGuiMouseCursor_::ResizeNWSE as isize,
}

impl ImGuiMouseCursor {
	pub fn as_c(self) -> ImGuiMouseCursor_::Type { self as ImGuiMouseCursor_::Type }

	pub fn from_c(a: ImGuiMouseCursor_::Type) -> ImGuiMouseCursor {
		match a {
			ImGuiMouseCursor_::Arrow => ImGuiMouseCursor::Arrow,
			ImGuiMouseCursor_::TextInput => ImGuiMouseCursor::TextInput,
			ImGuiMouseCursor_::Move => ImGuiMouseCursor::Move,
			ImGuiMouseCursor_::ResizeNS => ImGuiMouseCursor::ResizeNS,
			ImGuiMouseCursor_::ResizeEW => ImGuiMouseCursor::ResizeEW,
			ImGuiMouseCursor_::ResizeNESW => ImGuiMouseCursor::ResizeNESW,
			ImGuiMouseCursor_::ResizeNWSE => ImGuiMouseCursor::ResizeNWSE,
			_ => unreachable!()
		}
	}
}
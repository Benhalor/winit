use crate::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Position, Size, LogicalPosition};
use crate::error::{ExternalError, NotSupportedError, OsError as RootOE};
use crate::event;
use crate::icon::Icon;
use crate::monitor::MonitorHandle as RootMH;
use crate::window::{
    CursorIcon, Fullscreen, UserAttentionType, WindowAttributes, WindowId as RootWI,
};

use raw_window_handle::web::WebHandle;

use super::{monitor, EventLoopWindowTarget};

use std::cell::{Ref, RefCell};
use std::collections::vec_deque::IntoIter as VecDequeIter;
use std::collections::VecDeque;
use std::rc::Rc;
use std::{mem, ptr, str};
use std::sync::Mutex;
use crate::event::{Event, WindowEvent, DeviceEvent, MouseButton, ElementState, TouchPhase, ModifiersState, DeviceId, KeyboardInput, VirtualKeyCode, Touch};
use std::sync::Arc;
use std::os::raw::{c_char, c_void, c_double, c_ulong, c_int};

use crate::platform_impl::WindowId;


const DOCUMENT_NAME: &'static str = "#document\0";


use crate::platform_impl::OsError;

use crate::platform_impl::platform::ffi;

pub struct Window2 {
    cursor_grabbed: Mutex<bool>,
    cursor_hidden: Mutex<bool>,
    is_fullscreen: bool,
    //events: Box<Mutex<VecDeque<Event<'_, T>>>>,todo
}

pub struct Window {
    window: Arc<Window2>,
}

impl Window {
    pub fn new<T>(
        target: &EventLoopWindowTarget<T>,
        attr: WindowAttributes,
        platform_attr: PlatformSpecificBuilderAttributes,
    ) -> Result<Self, RootOE> {
        

        let w = Window2 {
            cursor_grabbed: Default::default(),
            cursor_hidden: Default::default(),
            //events: Default::default(),todo
            is_fullscreen: attr.fullscreen.is_some(),
        };

        let window = Window {
            window: Arc::new(w),
        };


        // TODO: set up more event callbacks
        /*unsafe {
            em_try(ffi::emscripten_set_mousemove_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(mouse_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_mousedown_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(mouse_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_mouseup_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(mouse_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_keydown_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(keyboard_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_keyup_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(keyboard_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_touchstart_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(touch_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_touchend_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(touch_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_touchmove_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(touch_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
            em_try(ffi::emscripten_set_touchcancel_callback(DOCUMENT_NAME.as_ptr() as *const c_char, mem::transmute(&*window.window.events), ffi::EM_FALSE, Some(touch_callback)))
                .map_err(|e| OsError(format!("emscripten error: {}", e)))?;
        }*/

        if attr.fullscreen.is_some() {
            unsafe {
                em_try(ffi::emscripten_request_fullscreen(ptr::null(), ffi::EM_TRUE))
                    .map_err(|e| OsError(e))?;
                em_try(ffi::emscripten_set_fullscreenchange_callback(ptr::null(), 0 as *mut c_void, ffi::EM_FALSE, Some(fullscreen_callback)))
                    .map_err(|e| OsError(e))?;
            }
        } else if let Some(size) = attr.dimensions {
            window.set_inner_size(size);
        }

        //todo *events_loop.window.lock().unwrap() = Some(window.window.clone());
        Ok(window)


    }

    pub fn set_title(&self, title: &str) {
        //self.canvas.borrow().set_attribute("alt", title);
    }

    pub fn set_visible(&self, _visible: bool) {
        // Intentionally a no-op
    }

    pub fn request_redraw(&self) {
        //(self.register_redraw_request)();
    }

    pub fn outer_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        Ok(Some((0, 0).into()))
    }

    pub fn inner_position(&self) -> Result<PhysicalPosition<i32>, NotSupportedError> {
        // Note: the canvas element has no window decorations, so this is equal to `outer_position`.
        self.outer_position()
    }

    pub fn set_outer_position(&self, position: Position) {
        /*let position = position.to_logical::<f64>(self.scale_factor());

        let canvas = self.canvas.borrow();
        canvas.set_attribute("position", "fixed");
        canvas.set_attribute("left", &position.x.to_string());
        canvas.set_attribute("top", &position.y.to_string());*/
    }

    #[inline]
    pub fn inner_size(&self) -> PhysicalSize<u32> {
       unsafe {
            let mut width = 0;
            let mut height = 0;
            let mut fullscreen = 0;

            if ffi::emscripten_get_canvas_size(&mut width, &mut height, &mut fullscreen)
                != ffi::EMSCRIPTEN_RESULT_SUCCESS
            {
                None
            } else {
                let dpi_factor = self.get_hidpi_factor();
                let logical = PhysicalSize::from_physical((width as i32, height as i32), dpi_factor);
                Some(logical)
            }
        }
    }

    #[inline]
    pub fn outer_size(&self) -> PhysicalSize<u32> {
        // Note: the canvas element has no window decorations, so this is equal to `inner_size`.
        self.inner_size()
    }

    #[inline]
    pub fn set_inner_size(&self, size: Size) {
        unsafe {
            let dpi_factor = self.get_hidpi_factor();
            let physical = PhysicalSize::from_logical(size, dpi_factor);
            let (width, height): (u32, u32) = physical.into();
            ffi::emscripten_set_element_css_size(
                ptr::null(),
                width as c_double,
                height as c_double,
            );
        }
    }

    #[inline]
    pub fn set_min_inner_size(&self, _dimensions: Option<Size>) {
        // Intentionally a no-op: users can't resize canvas elements
    }

    #[inline]
    pub fn set_max_inner_size(&self, _dimensions: Option<Size>) {
        // Intentionally a no-op: users can't resize canvas elements
    }

    #[inline]
    pub fn set_resizable(&self, _resizable: bool) {
        // Intentionally a no-op: users can't resize canvas elements
    }

    #[inline]
    pub fn scale_factor(&self) -> f64 {
        //super::backend::scale_factor()
    }

    #[inline]
    pub fn set_cursor_icon(&self, cursor: CursorIcon) {
        // N/A
    }

    #[inline]
    pub fn set_cursor_position(&self, _position: Position) -> Result<(), ExternalError> {
        Err(ExternalError::NotSupported(NotSupportedError::new()))
    }

    #[inline]
    pub fn set_cursor_grab(&self, _grab: bool) -> Result<(), ExternalError> {
        Err(ExternalError::NotSupported(NotSupportedError::new()))
    }

    #[inline]
    pub fn set_cursor_visible(&self, visible: bool) {
        let mut hidden_lock = self.window.cursor_hidden.lock().unwrap();
        if visible == *hidden_lock { return; }
        if !visible {
            unsafe { ffi::emscripten_hide_mouse() };
        } else {
            show_mouse();
        }
        *hidden_lock = visible;
    }

    #[inline]
    pub fn drag_window(&self) -> Result<(), ExternalError> {
        //Err(ExternalError::NotSupported(NotSupportedError::new()))
    }

    #[inline]
    pub fn set_minimized(&self, _minimized: bool) {
        // Intentionally a no-op, as canvases cannot be 'minimized'
    }

    #[inline]
    pub fn set_maximized(&self, _maximized: bool) {
        // Intentionally a no-op, as canvases cannot be 'maximized'
    }

    #[inline]
    pub fn is_maximized(&self) -> bool {
        // Canvas cannot be 'maximized'
        false
    }

    #[inline]
    pub fn fullscreen(&self) -> Option<Fullscreen> {
        /*if self.canvas.borrow().is_fullscreen() {
            Some(Fullscreen::Borderless(Some(self.current_monitor_inner())))
        } else {
            None
        }*/
        None
    }

    #[inline]
    pub fn set_fullscreen(&self, monitor: Option<Fullscreen>) {
        /*if monitor.is_some() {
            self.canvas.borrow().request_fullscreen();
        } else if self.canvas.borrow().is_fullscreen() {
            backend::exit_fullscreen();
        }*/
    }

    #[inline]
    pub fn set_decorations(&self, _decorations: bool) {
        // Intentionally a no-op, no canvas decorations
    }

    #[inline]
    pub fn set_always_on_top(&self, _always_on_top: bool) {
        // Intentionally a no-op, no window ordering
    }

    #[inline]
    pub fn set_window_icon(&self, _window_icon: Option<Icon>) {
        // Currently an intentional no-op
    }

    #[inline]
    pub fn set_ime_position(&self, _position: Position) {
        // Currently a no-op as it does not seem there is good support for this on web
    }

    #[inline]
    pub fn focus_window(&self) {
        // Currently a no-op as it does not seem there is good support for this on web
    }

    #[inline]
    pub fn request_user_attention(&self, _request_type: Option<UserAttentionType>) {
        // Currently an intentional no-op
    }

    #[inline]
    // Allow directly accessing the current monitor internally without unwrapping.
    fn current_monitor_inner(&self) -> RootMH {
        RootMH {
            inner: monitor::Handle,
        }
    }

    #[inline]
    pub fn current_monitor(&self) -> Option<RootMH> {
        Some(self.current_monitor_inner())
    }

    #[inline]
    pub fn available_monitors(&self) -> VecDequeIter<monitor::Handle> {
        VecDeque::new().into_iter()
    }

    #[inline]
    pub fn primary_monitor(&self) -> Option<RootMH> {
        Some(RootMH {
            inner: monitor::Handle,
        })
    }

    #[inline]
    pub fn id(&self) -> Id {
        return self.id;
    }

    #[inline]
    pub fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        let handle = WebHandle {
            id: self.id.0,
            ..WebHandle::empty()
        };

        raw_window_handle::RawWindowHandle::Web(handle)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        // Delete window from events_loop
        // TODO: ?
        /*if let Some(ev) = self.events_loop.upgrade() {
            let _ = ev.window.lock().unwrap().take().unwrap();
        }*/

        unsafe {
            // Return back to normal cursor state
            self.hide_cursor(false);
            self.grab_cursor(false);

            // Exit fullscreen if on
            if self.window.is_fullscreen {
                ffi::emscripten_set_fullscreenchange_callback(ptr::null(), 0 as *mut c_void, ffi::EM_FALSE, None);
                ffi::emscripten_exit_fullscreen();
            }

            // Delete callbacks
            ffi::emscripten_set_keydown_callback(DOCUMENT_NAME.as_ptr() as *const c_char, 0 as *mut c_void, ffi::EM_FALSE,None);
            ffi::emscripten_set_keyup_callback(DOCUMENT_NAME.as_ptr() as *const c_char, 0 as *mut c_void, ffi::EM_FALSE,None);
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub(crate) u32);

impl Id {
    pub const unsafe fn dummy() -> Id {
        Id(0)
    }
}

#[derive(Default, Clone)]
pub struct PlatformSpecificBuilderAttributes {
    //pub(crate) canvas: Option<backend::RawCanvasType>,
}



fn em_try(res: ffi::EMSCRIPTEN_RESULT) -> Result<(), String> {
    match res {
        ffi::EMSCRIPTEN_RESULT_SUCCESS | ffi::EMSCRIPTEN_RESULT_DEFERRED => Ok(()),
        r @ _ => Err(error_to_str(r).to_string()),
    }
}

extern "C" fn mouse_callback(
    event_type: c_int,
    event: *const ffi::EmscriptenMouseEvent,
    event_queue: *mut c_void) -> ffi::EM_BOOL
{
    unsafe {
        let queue: &Mutex<VecDeque<Event<'_>>> = mem::transmute(event_queue);

        let modifiers = ModifiersState {
            shift: (*event).shiftKey == ffi::EM_TRUE,
            ctrl: (*event).ctrlKey == ffi::EM_TRUE,
            alt: (*event).altKey == ffi::EM_TRUE,
            logo: (*event).metaKey == ffi::EM_TRUE,
        };

        match event_type {
            ffi::EMSCRIPTEN_EVENT_MOUSEMOVE => {
                let dpi_factor = get_hidpi_factor();
                let position = LogicalPosition::from_physical(
                    ((*event).canvasX as f64, (*event).canvasY as f64),
                    dpi_factor,
                );
                queue.lock().unwrap().push_back(Event::WindowEvent {
                    window_id: WindowId(WindowId(0)),
                    event: WindowEvent::CursorMoved {
                        device_id: DeviceId(DeviceId),
                        position,
                        modifiers: modifiers,
                    }
                });
                queue.lock().unwrap().push_back(Event::DeviceEvent {
                    device_id: DeviceId(DeviceId),
                    event: DeviceEvent::MouseMotion {
                        delta: ((*event).movementX as f64, (*event).movementY as f64),
                    }
                });
            },
            mouse_input @ ffi::EMSCRIPTEN_EVENT_MOUSEDOWN |
            mouse_input @ ffi::EMSCRIPTEN_EVENT_MOUSEUP => {
                let button = match (*event).button {
                    0 => MouseButton::Left,
                    1 => MouseButton::Middle,
                    2 => MouseButton::Right,
                    other => MouseButton::Other(other as u8),
                };
                let state = match mouse_input {
                    ffi::EMSCRIPTEN_EVENT_MOUSEDOWN => ElementState::Pressed,
                    ffi::EMSCRIPTEN_EVENT_MOUSEUP => ElementState::Released,
                    _ => unreachable!(),
                };
                queue.lock().unwrap().push_back(Event::WindowEvent {
                    window_id: WindowId(WindowId(0)),
                    event: WindowEvent::MouseInput {
                        device_id: DeviceId(DeviceId),
                        state: state,
                        button: button,
                        modifiers: modifiers,
                    }
                })
            },
            _ => {
            }
        }
    }
    ffi::EM_FALSE
}

extern "C" fn keyboard_callback(
    event_type: c_int,
    event: *const ffi::EmscriptenKeyboardEvent,
    event_queue: *mut c_void) -> ffi::EM_BOOL
{
    unsafe {
        let queue: &Mutex<VecDeque<Event<'_>>> = mem::transmute(event_queue);

        let modifiers = ModifiersState {
            shift: (*event).shiftKey == ffi::EM_TRUE,
            ctrl: (*event).ctrlKey == ffi::EM_TRUE,
            alt: (*event).altKey == ffi::EM_TRUE,
            logo: (*event).metaKey == ffi::EM_TRUE,
        };

        match event_type {
            ffi::EMSCRIPTEN_EVENT_KEYDOWN => {
                queue.lock().unwrap().push_back(Event::WindowEvent {
                    window_id: WindowId(WindowId(0)),
                    event: WindowEvent::KeyboardInput {
                        device_id: DeviceId(DeviceId),
                        input: KeyboardInput {
                            scancode: key_translate((*event).key) as u32,
                            state: ElementState::Pressed,
                            virtual_keycode: key_translate_virt((*event).key, (*event).location),
                            modifiers,
                        },
                    },
                });
            },
            ffi::EMSCRIPTEN_EVENT_KEYUP => {
                queue.lock().unwrap().push_back(Event::WindowEvent {
                    window_id: WindowId(WindowId(0)),
                    event: WindowEvent::KeyboardInput {
                        device_id: DeviceId(DeviceId),
                        input: KeyboardInput {
                            scancode: key_translate((*event).key) as u32,
                            state: ElementState::Released,
                            virtual_keycode: key_translate_virt((*event).key, (*event).location),
                            modifiers,
                        },
                    },
                });
            },
            _ => {
            }
        }
    }
    ffi::EM_FALSE
}

extern fn touch_callback(
    event_type: c_int,
    event: *const ffi::EmscriptenTouchEvent,
    event_queue: *mut c_void) -> ffi::EM_BOOL
{
    unsafe {
        let queue: &Mutex<VecDeque<Event<'_>>> = mem::transmute(event_queue);

        let phase = match event_type {
            ffi::EMSCRIPTEN_EVENT_TOUCHSTART => TouchPhase::Started,
            ffi::EMSCRIPTEN_EVENT_TOUCHEND => TouchPhase::Ended,
            ffi::EMSCRIPTEN_EVENT_TOUCHMOVE => TouchPhase::Moved,
            ffi::EMSCRIPTEN_EVENT_TOUCHCANCEL => TouchPhase::Cancelled,
            _ => return ffi::EM_FALSE,
        };

        for touch in 0..(*event).numTouches as usize {
            let touch = (*event).touches[touch];
            if touch.isChanged == ffi::EM_TRUE {
                let dpi_factor = get_hidpi_factor();
                let location = LogicalPosition::from_physical(
                    (touch.canvasX as f64, touch.canvasY as f64),
                    dpi_factor,
                );
                queue.lock().unwrap().push_back(Event::WindowEvent {
                    window_id: WindowId(WindowId(0)),
                    event: WindowEvent::Touch(Touch {
                        device_id: DeviceId(DeviceId),
                        phase,
                        id: touch.identifier as u64,
                        location,
                    }),
                });
            }
        }
    }
    ffi::EM_FALSE
}

// In case of fullscreen window this method will request fullscreen on change
#[allow(non_snake_case)]
unsafe extern "C" fn fullscreen_callback(
    _eventType: c_int,
    _fullscreenChangeEvent: *const ffi::EmscriptenFullscreenChangeEvent,
    _userData: *mut c_void) -> ffi::EM_BOOL
{
    ffi::emscripten_request_fullscreen(ptr::null(), ffi::EM_TRUE);
    ffi::EM_FALSE
}

fn show_mouse() {
    // Hide mouse hasn't show mouse equivalent.
    // There is a pull request on emscripten that hasn't been merged #4616
    // that contains:
    //
    // var styleSheet = document.styleSheets[0];
    // var rules = styleSheet.cssRules;
    // for (var i = 0; i < rules.length; i++) {
    //   if (rules[i].cssText.substr(0, 6) == 'canvas') {
    //     styleSheet.deleteRule(i);
    //     i--;
    //   }
    // }
    // styleSheet.insertRule('canvas.emscripten { border: none; cursor: auto; }', 0);
    unsafe {
            ffi::emscripten_asm_const(b"var styleSheet = document.styleSheets[0]; var rules = styleSheet.cssRules; for (var i = 0; i < rules.length; i++) { if (rules[i].cssText.substr(0, 6) == 'canvas') { styleSheet.deleteRule(i); i--; } } styleSheet.insertRule('canvas.emscripten { border: none; cursor: auto; }', 0);\0".as_ptr() as *const c_char);
    }
}

fn error_to_str(code: ffi::EMSCRIPTEN_RESULT) -> &'static str {
    match code {
        ffi::EMSCRIPTEN_RESULT_SUCCESS | ffi::EMSCRIPTEN_RESULT_DEFERRED
            => "Internal error in the library (success detected as failure)",

        ffi::EMSCRIPTEN_RESULT_NOT_SUPPORTED => "Not supported",
        ffi::EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => "Failed not deferred",
        ffi::EMSCRIPTEN_RESULT_INVALID_TARGET => "Invalid target",
        ffi::EMSCRIPTEN_RESULT_UNKNOWN_TARGET => "Unknown target",
        ffi::EMSCRIPTEN_RESULT_INVALID_PARAM => "Invalid parameter",
        ffi::EMSCRIPTEN_RESULT_FAILED => "Failed",
        ffi::EMSCRIPTEN_RESULT_NO_DATA => "No data",

        _ => "Undocumented error"
    }
}

fn get_hidpi_factor() -> f64 {
    unsafe { ffi::emscripten_get_device_pixel_ratio() as f64 }
}

fn key_translate(input: [ffi::EM_UTF8; ffi::EM_HTML5_SHORT_STRING_LEN_BYTES]) -> u8 {
    let slice = &input[0..input.iter().take_while(|x| **x != 0).count()];
    let maybe_key = unsafe { str::from_utf8(mem::transmute::<_, &[u8]>(slice)) };
    let key = match maybe_key {
        Ok(key) => key,
        Err(_) => { return 0; },
    };
    if key.chars().count() == 1 {
        key.as_bytes()[0]
    } else {
        0
    }
}

fn key_translate_virt(input: [ffi::EM_UTF8; ffi::EM_HTML5_SHORT_STRING_LEN_BYTES],
                      location: c_ulong) -> Option<VirtualKeyCode>
{
    let slice = &input[0..input.iter().take_while(|x| **x != 0).count()];
    let maybe_key = unsafe { str::from_utf8(mem::transmute::<_, &[u8]>(slice)) };
    let key = match maybe_key {
        Ok(key) => key,
        Err(_) => { return None; },
    };
    
    match key {
        "Alt" => match location {
            ffi::DOM_KEY_LOCATION_LEFT => Some(VirtualKeyCode::LAlt),
            ffi::DOM_KEY_LOCATION_RIGHT => Some(VirtualKeyCode::RAlt),
            _ => None,
        },
        "AltGraph" => None,
        "CapsLock" => None,
        "Control" => match location {
            ffi::DOM_KEY_LOCATION_LEFT => Some(VirtualKeyCode::LControl),
            ffi::DOM_KEY_LOCATION_RIGHT => Some(VirtualKeyCode::RControl),
            _ => None,
        },
        "Fn" => None,
        "FnLock" => None,
        "Hyper" => None,
        "Meta" => None,
        "NumLock" => Some(VirtualKeyCode::Numlock),
        "ScrollLock" => Some(VirtualKeyCode::Scroll),
        "Shift" => match location {
            ffi::DOM_KEY_LOCATION_LEFT => Some(VirtualKeyCode::LShift),
            ffi::DOM_KEY_LOCATION_RIGHT => Some(VirtualKeyCode::RShift),
            _ => None,
        },
        "Super" => None,
        "Symbol" => None,
        "SymbolLock" => None,

        "Enter" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::NumpadEnter),
            _ => Some(VirtualKeyCode::Return),
        },
        "Tab" => Some(VirtualKeyCode::Tab),
        " " => Some(VirtualKeyCode::Space),

        "ArrowDown" => Some(VirtualKeyCode::Down),
        "ArrowLeft" => Some(VirtualKeyCode::Left),
        "ArrowRight" => Some(VirtualKeyCode::Right),
        "ArrowUp" => Some(VirtualKeyCode::Up),
        "End" => None,
        "Home" => None,
        "PageDown" => None,
        "PageUp" => None,

        "Backspace" => Some(VirtualKeyCode::Back),
        "Clear" => None,
        "Copy" => None,
        "CrSel" => None,
        "Cut" => None,
        "Delete" => None,
        "EraseEof" => None,
        "ExSel" => None,
        "Insert" => Some(VirtualKeyCode::Insert),
        "Paste" => None,
        "Redo" => None,
        "Undo" => None,

        "Accept" => None,
        "Again" => None,
        "Attn" => None,
        "Cancel" => None,
        "ContextMenu" => None,
        "Escape" => Some(VirtualKeyCode::Escape),
        "Execute" => None,
        "Find" => None,
        "Finish" => None,
        "Help" => None,
        "Pause" => Some(VirtualKeyCode::Pause),
        "Play" => None,
        "Props" => None,
        "Select" => None,
        "ZoomIn" => None,
        "ZoomOut" => None,

        "BrightnessDown" => None,
        "BrightnessUp" => None,
        "Eject" => None,
        "LogOff" => None,
        "Power" => Some(VirtualKeyCode::Power),
        "PowerOff" => None,
        "PrintScreen" => Some(VirtualKeyCode::Snapshot),
        "Hibernate" => None,
        "Standby" => Some(VirtualKeyCode::Sleep),
        "WakeUp" => Some(VirtualKeyCode::Wake),

        "AllCandidates" => None,
        "Alphanumeric" => None,
        "CodeInput" => None,
        "Compose" => Some(VirtualKeyCode::Compose),
        "Convert" => Some(VirtualKeyCode::Convert),
        "Dead" => None,
        "FinalMode" => None,
        "GroupFirst" => None,
        "GroupLast" => None,
        "GroupNext" => None,
        "GroupPrevious" => None,
        "ModeChange" => None,
        "NextCandidate" => None,
        "NonConvert" => None,
        "PreviousCandidate" => None,
        "Process" => None,
        "SingleCandidate" => None,

        "HangulMode" => None,
        "HanjaMode" => None,
        "JunjaMode" => None,

        "Eisu" => None,
        "Hankaku" => None,
        "Hiragana" => None,
        "HiraganaKatakana" => None,
        "KanaMode" => Some(VirtualKeyCode::Kana),
        "KanjiMode" => Some(VirtualKeyCode::Kanji),
        "Romaji" => None,
        "Zenkaku" => None,
        "ZenkakuHanaku" => None,

        "F1" => Some(VirtualKeyCode::F1),
        "F2" => Some(VirtualKeyCode::F2),
        "F3" => Some(VirtualKeyCode::F3),
        "F4" => Some(VirtualKeyCode::F4),
        "F5" => Some(VirtualKeyCode::F5),
        "F6" => Some(VirtualKeyCode::F6),
        "F7" => Some(VirtualKeyCode::F7),
        "F8" => Some(VirtualKeyCode::F8),
        "F9" => Some(VirtualKeyCode::F9),
        "F10" => Some(VirtualKeyCode::F10),
        "F11" => Some(VirtualKeyCode::F11),
        "F12" => Some(VirtualKeyCode::F12),
        "F13" => Some(VirtualKeyCode::F13),
        "F14" => Some(VirtualKeyCode::F14),
        "F15" => Some(VirtualKeyCode::F15),
        "F16" => Some(VirtualKeyCode::F16),
        "F17" => Some(VirtualKeyCode::F17),
        "F18" => Some(VirtualKeyCode::F18),
        "F19" => Some(VirtualKeyCode::F19),
        "F20" => Some(VirtualKeyCode::F20),
        "F21" => Some(VirtualKeyCode::F21),
        "F22" => Some(VirtualKeyCode::F22),
        "F23" => Some(VirtualKeyCode::F23),
        "F24" => Some(VirtualKeyCode::F24),
        "Soft1" => None,
        "Soft2" => None,
        "Soft3" => None,
        "Soft4" => None,

        "AppSwitch" => None,
        "Call" => None,
        "Camera" => None,
        "CameraFocus" => None,
        "EndCall" => None,
        "GoBack" => None,
        "GoHome" => None,
        "HeadsetHook" => None,
        "LastNumberRedial" => None,
        "Notification" => None,
        "MannerMode" => None,
        "VoiceDial" => None,

        "ChannelDown" => None,
        "ChannelUp" => None,
        "MediaFastForward" => None,
        "MediaPause" => None,
        "MediaPlay" => None,
        "MediaPlayPause" => Some(VirtualKeyCode::PlayPause),
        "MediaRecord" => None,
        "MediaRewind" => None,
        "MediaStop" => Some(VirtualKeyCode::MediaStop),
        "MediaTrackNext" => Some(VirtualKeyCode::NextTrack),
        "MediaTrackPrevious" => Some(VirtualKeyCode::PrevTrack),

        "AudioBalanceLeft" => None,
        "AudioBalanceRight" => None,
        "AudioBassDown" => None,
        "AudioBassBoostDown" => None,
        "AudioBassBoostToggle" => None,
        "AudioBassBoostUp" => None,
        "AudioBassUp" => None,
        "AudioFaderFront" => None,
        "AudioFaderRear" => None,
        "AudioSurroundModeNext" => None,
        "AudioTrebleDown" => None,
        "AudioTrebleUp" => None,
        "AudioVolumeDown" => Some(VirtualKeyCode::VolumeDown),
        "AudioVolumeMute" => Some(VirtualKeyCode::Mute),
        "AudioVolumeUp" => Some(VirtualKeyCode::VolumeUp),
        "MicrophoneToggle" => None,
        "MicrophoneVolumeDown" => None,
        "MicrophoneVolumeMute" => None,
        "MicrophoneVolumeUp" => None,

        "TV" => None,
        "TV3DMode" => None,
        "TVAntennaCable" => None,
        "TVAudioDescription" => None,
        "TVAudioDescriptionMixDown" => None,
        "TVAudioDescriptionMixUp" => None,
        "TVContentsMenu" => None,
        "TVDataService" => None,
        "TVInput" => None,
        "TVInputComponent1" => None,
        "TVInputComponent2" => None,
        "TVInputComposite1" => None,
        "TVInputComposite2" => None,
        "TVInputHDM1" => None,
        "TVInputHDM2" => None,
        "TVInputHDM3" => None,
        "TVInputHDM4" => None,
        "TVInputVGA1" => None,
        "TVMediaContext" => None,
        "TVNetwork" => None,
        "TVNumberEntry" => None,
        "TVPower" => None,
        "TVRadioService" => None,
        "TVSatellite" => None,
        "TVSatelliteBS" => None,
        "TVSatelliteCS" => None,
        "TVSatelliteToggle" => None,
        "TVTerrestrialAnalog" => None,
        "TVTerrestrialDigital" => None,
        "TVTimer" => None,

        "AVRInput" => None,
        "AVRPower" => None,
        "ColorF0Red" => None,
        "ColorF1Green" => None,
        "ColorF2Yellow" => None,
        "ColorF3Blue" => None,
        "ColorF4Grey" => None,
        "ColorF5Brown" => None,
        "ClosedCaptionToggle" => None,
        "Dimmer" => None,
        "DisplaySwap" => None,
        "DVR" => None,
        "Exit" => None,
        "FavoriteClear0" => None,
        "FavoriteClear1" => None,
        "FavoriteClear2" => None,
        "FavoriteClear3" => None,
        "FavoriteRecall0" => None,
        "FavoriteRecall1" => None,
        "FavoriteRecall2" => None,
        "FavoriteRecall3" => None,
        "FavoriteStore0" => None,
        "FavoriteStore1" => None,
        "FavoriteStore2" => None,
        "FavoriteStore3" => None,
        "FavoriteStore4" => None,
        "Guide" => None,
        "GuideNextDay" => None,
        "GuidePreviousDay" => None,
        "Info" => None,
        "InstantReplay" => None,
        "Link" => None,
        "ListProgram" => None,
        "LiveContent" => None,
        "Lock" => None,
        "MediaApps" => None,
        "MediaAudioTrack" => None,
        "MediaLast" => None,
        "MediaSkipBackward" => None,
        "MediaSkipForward" => None,
        "MediaStepBackward" => None,
        "MediaStepForward" => None,
        "MediaTopMenu" => None,
        "NavigateIn" => None,
        "NavigateNext" => None,
        "NavigateOut" => None,
        "NavigatePrevious" => None,
        "NextFavoriteChannel" => None,
        "NextUserProfile" => None,
        "OnDemand" => None,
        "Pairing" => None,
        "PinPDown" => None,
        "PinPMove" => None,
        "PinPToggle" => None,
        "PinPUp" => None,
        "PlaySpeedDown" => None,
        "PlaySpeedReset" => None,
        "PlaySpeedUp" => None,
        "RandomToggle" => None,
        "RcLowBattery" => None,
        "RecordSpeedNext" => None,
        "RfBypass" => None,
        "ScanChannelsToggle" => None,
        "ScreenModeNext" => None,
        "Settings" => None,
        "SplitScreenToggle" => None,
        "STBInput" => None,
        "STBPower" => None,
        "Subtitle" => None,
        "Teletext" => None,
        "VideoModeNext" => None,
        "Wink" => None,
        "ZoomToggle" => None,

        "SpeechCorrectionList" => None,
        "SpeechInputToggle" => None,

        "Close" => None,
        "New" => None,
        "Open" => None,
        "Print" => None,
        "Save" => None,
        "SpellCheck" => None,
        "MailForward" => None,
        "MailReply" => None,
        "MailSend" => None,

        "LaunchCalculator" => Some(VirtualKeyCode::Calculator),
        "LaunchCalendar" => None,
        "LaunchContacts" => None,
        "LaunchMail" => Some(VirtualKeyCode::Mail),
        "LaunchMediaPlayer" => None,
        "LaunchMusicPlayer" => None,
        "LaunchMyComputer" => Some(VirtualKeyCode::MyComputer),
        "LaunchPhone" => None,
        "LaunchScreenSaver" => None,
        "LaunchSpreadsheet" => None,
        "LaunchWebCam" => None,
        "LaunchWordProcessor" => None,
        "LaunchApplication1" => None,
        "LaunchApplication2" => None,
        "LaunchApplication3" => None,
        "LaunchApplication4" => None,
        "LaunchApplication5" => None,
        "LaunchApplication6" => None,
        "LaunchApplication7" => None,
        "LaunchApplication8" => None,
        "LaunchApplication9" => None,
        "LaunchApplication10" => None,
        "LaunchApplication11" => None,
        "LaunchApplication12" => None,
        "LaunchApplication13" => None,
        "LaunchApplication14" => None,
        "LaunchApplication15" => None,
        "LaunchApplication16" => None,

        "BrowserBack" => Some(VirtualKeyCode::WebBack),
        "BrowserFavorites" => Some(VirtualKeyCode::WebFavorites),
        "BrowserForward" => Some(VirtualKeyCode::WebForward),
        "BrowserHome" => Some(VirtualKeyCode::WebHome),
        "BrowserRefresh" => Some(VirtualKeyCode::WebRefresh),
        "BrowserSearch" => Some(VirtualKeyCode::WebSearch),
        "BrowserStop" => Some(VirtualKeyCode::WebStop),

        "Decimal" => Some(VirtualKeyCode::Decimal),
        "Key11" => None,
        "Key12" => None,
        "Multiply" | "*" => Some(VirtualKeyCode::Multiply),
        "Add" | "+" => Some(VirtualKeyCode::Add),
        // "Clear" => None,
        "Divide" => Some(VirtualKeyCode::Divide),
        "Subtract" | "-" => Some(VirtualKeyCode::Subtract),
        "Separator" => None,
        "0" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad0),
            _ => Some(VirtualKeyCode::Key0),
        },
        "1" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad1),
            _ => Some(VirtualKeyCode::Key1),
        },
        "2" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad2),
            _ => Some(VirtualKeyCode::Key2),
        },
        "3" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad3),
            _ => Some(VirtualKeyCode::Key3),
        },
        "4" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad4),
            _ => Some(VirtualKeyCode::Key4),
        },
        "5" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad5),
            _ => Some(VirtualKeyCode::Key5),
        },
        "6" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad6),
            _ => Some(VirtualKeyCode::Key6),
        },
        "7" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad7),
            _ => Some(VirtualKeyCode::Key7),
        },
        "8" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad8),
            _ => Some(VirtualKeyCode::Key8),
        },
        "9" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::Numpad9),
            _ => Some(VirtualKeyCode::Key9),
        },

        "A" | "a" => Some(VirtualKeyCode::A),
        "B" | "b" => Some(VirtualKeyCode::B),
        "C" | "c" => Some(VirtualKeyCode::C),
        "D" | "d" => Some(VirtualKeyCode::D),
        "E" | "e" => Some(VirtualKeyCode::E),
        "F" | "f" => Some(VirtualKeyCode::F),
        "G" | "g" => Some(VirtualKeyCode::G),
        "H" | "h" => Some(VirtualKeyCode::H),
        "I" | "i" => Some(VirtualKeyCode::I),
        "J" | "j" => Some(VirtualKeyCode::J),
        "K" | "k" => Some(VirtualKeyCode::K),
        "L" | "l" => Some(VirtualKeyCode::L),
        "M" | "m" => Some(VirtualKeyCode::M),
        "N" | "n" => Some(VirtualKeyCode::N),
        "O" | "o" => Some(VirtualKeyCode::O),
        "P" | "p" => Some(VirtualKeyCode::P),
        "Q" | "q" => Some(VirtualKeyCode::Q),
        "R" | "r" => Some(VirtualKeyCode::R),
        "S" | "s" => Some(VirtualKeyCode::S),
        "T" | "t" => Some(VirtualKeyCode::T),
        "U" | "u" => Some(VirtualKeyCode::U),
        "V" | "v" => Some(VirtualKeyCode::V),
        "W" | "w" => Some(VirtualKeyCode::W),
        "X" | "x" => Some(VirtualKeyCode::X),
        "Y" | "y" => Some(VirtualKeyCode::Y),
        "Z" | "z" => Some(VirtualKeyCode::Z),

        "'" => Some(VirtualKeyCode::Apostrophe),
        "\\" => Some(VirtualKeyCode::Backslash),
        ":" => Some(VirtualKeyCode::Colon),
        "," => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::NumpadComma),
            _ => Some(VirtualKeyCode::Comma),
        },
        "=" => match location {
            ffi::DOM_KEY_LOCATION_NUMPAD => Some(VirtualKeyCode::NumpadEquals),
            _ => Some(VirtualKeyCode::Equals),
        },
        "{" => Some(VirtualKeyCode::LBracket),
        "." => Some(VirtualKeyCode::Period),
        "}" => Some(VirtualKeyCode::RBracket),
        ";" => Some(VirtualKeyCode::Semicolon),
        "/" => Some(VirtualKeyCode::Slash),

        _ => None,
    }
}
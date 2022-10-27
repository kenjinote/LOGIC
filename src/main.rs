//#![windows_subsystem = "windows"]

mod node;
use crate::node::Node;
mod nodelist;
use crate::nodelist::NodeList;
mod about;
use crate::about::About;
mod debug;
use crate::debug::Debug;
mod open;
use crate::open::Open;
mod save;
use crate::save::Save;

use winapi::{
    um::{
        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW,
                  CW_USEDEFAULT, MAKEINTRESOURCEW, SendMessageW, WM_CLOSE, WM_COMMAND,
                  SW_SHOWDEFAULT, WM_PAINT, BeginPaint, EndPaint, PAINTSTRUCT, WM_CREATE,
                  WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_RBUTTONDOWN, WM_RBUTTONUP,
                  EnableMenuItem, GetMenu, MF_BYPOSITION, MF_GRAYED, MF_ENABLED, IsDialogMessageW
                  },
        wingdi::{GetStockObject, WHITE_BRUSH, TextOutW},
        libloaderapi::{GetModuleHandleW, LoadStringW, },
        shellapi::{ShellExecuteW},
    },
    shared::{
        windef::{HWND, HBRUSH},
        minwindef::{UINT, WPARAM, LPARAM, LRESULT, LOWORD, HIWORD},
    },
};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;
use winapi::um::winuser::InvalidateRect;


mod utility;
use utility::encode;
static mut hDebug: HWND = ptr::null_mut();

fn main() {
    unsafe {
        let class_name = encode("LogicWindowClass");
        if !register_wndclass(&class_name) {
            return;
        }
        let hwnd = create_window(&class_name);
        if hwnd.is_null() {
            return;
        }
        ShowWindow(hwnd, SW_NORMAL);
        UpdateWindow(hwnd);
        let mut msg : MSG = MaybeUninit::uninit().assume_init();
        loop {
            if GetMessageW(&mut msg, ptr::null_mut(), 0, 0) == 0 {
                return;
            }
            if hDebug != ptr::null_mut() {
                if IsDialogMessageW(hDebug, &mut msg) != 0 {
                    continue;
                }
            }
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }
}

unsafe fn register_wndclass(class_name: &[u16]) -> bool {
    let mut winc = mem::zeroed::<WNDCLASSW>();
    winc.style = CS_HREDRAW | CS_VREDRAW;
    winc.lpfnWndProc = Some(win_proc);
    winc.hIcon = LoadIconW(GetModuleHandleW(ptr::null_mut()), MAKEINTRESOURCEW(1));
    winc.hCursor = LoadCursorW(ptr::null_mut(), IDC_ARROW);
    winc.hbrBackground = GetStockObject(WHITE_BRUSH as i32) as HBRUSH;
    winc.lpszMenuName=3 as *const u16;
    winc.lpszClassName = class_name.as_ptr();
    RegisterClassW(&winc) > 0
}

unsafe fn create_window(class_name: &[u16]) -> HWND {
    let mut title : [u16; 256] = MaybeUninit::uninit().assume_init();
    LoadStringW(GetModuleHandleW(ptr::null_mut()), 2, title.as_mut_ptr(), 256);
    CreateWindowExW(
        0,
        class_name.as_ptr(),
        title.as_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT, 0, CW_USEDEFAULT, 0,
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
        ptr::null_mut(),
    )
}

unsafe extern "system" fn win_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    static mut NODELIST: NodeList = NodeList::new();
    static mut DRAG: bool = false;
    match msg {
        WM_CREATE => {
            let node1 = Node::new(50.0, 50.0, 100.0, 100.0);
            let node2 = Node::new(100.0, 100.0, 100.0, 100.0);

            NODELIST.add(node1);
            NODELIST.add(node2);

            let menu = GetMenu(hwnd);
            EnableMenuItem(menu, 200, MF_GRAYED);
            EnableMenuItem(menu, 201, MF_GRAYED);
        },
        WM_LBUTTONDOWN => {
            let x = LOWORD(l_param as u32) as f64;
            let y = HIWORD(l_param as u32) as f64;
            let hit = NODELIST.hit_test(x, y);
            if hit != None {
                NODELIST.select(x as f64, y as f64);
                DRAG = true;
            } else {
                NODELIST.unselectall();
            }
            InvalidateRect(hwnd, ptr::null(), 1);
        },
        WM_LBUTTONUP => {
            DRAG = false;
            if NODELIST.can_undo() {
                let menu = GetMenu(hwnd);
                EnableMenuItem(menu, 200, MF_ENABLED);
            }
        },
        WM_PAINT => {
            let mut ps : PAINTSTRUCT = MaybeUninit::uninit().assume_init();
            let hdc = BeginPaint(hwnd, &mut ps);

            NODELIST.draw(hdc);

            EndPaint(hwnd, &ps);
        },
        WM_COMMAND => {
            let id = LOWORD(w_param as u32) as i32;
            match id {
                105 => {// Exit
                    SendMessageW(hwnd, WM_CLOSE, 0,0);
                },
                101 => {// Open
                    Open::show(hwnd);
                },
                102 => {// Save
                    Save::show(hwnd);
                },
                602 => {// About
                    About::show(hwnd);
                },
                600 => {// Open URL
                    ShellExecuteW(hwnd, encode("open").as_ptr(), encode("https://github.com/kenjinote/LOGIC").as_ptr(), ptr::null(), ptr::null(), SW_SHOWDEFAULT);
                },

                200 => {// Undo
                    NODELIST.undo();
                    InvalidateRect(hwnd, ptr::null(), 1);
                    let menu = GetMenu(hwnd);
                    if NODELIST.can_undo() {
                        EnableMenuItem(menu, 200, MF_ENABLED);
                    }
                    EnableMenuItem(menu, 201, MF_ENABLED);
                },

                201 => {// Redo
                    NODELIST.redo();
                    InvalidateRect(hwnd, ptr::null(), 1);
                    let menu = GetMenu(hwnd);
                    if NODELIST.can_redo() {
                        EnableMenuItem(menu, 201, MF_ENABLED);
                    }
                    EnableMenuItem(menu, 200, MF_ENABLED);
                },

                205 => {// Delete
                    NODELIST.delete();
                    InvalidateRect(hwnd, ptr::null(), 1);
                },
                206 => {// Select All
                    NODELIST.selectall();
                    InvalidateRect(hwnd, ptr::null_mut(), 1);
                },
                207 => {// UnSelect All
                    NODELIST.unselectall();
                    InvalidateRect(hwnd, ptr::null_mut(), 1);
                },
                503 => {// Debug Window
                    hDebug = Debug::show(hwnd);
                },
                _ => {}
            }
        },
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    };
    0
}
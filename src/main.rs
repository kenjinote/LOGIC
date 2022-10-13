//#![windows_subsystem = "windows"]

mod node;
use crate::node::Node;
mod about;
use crate::about::About;

use winapi::{
    um::{
        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW,
                  CW_USEDEFAULT, MAKEINTRESOURCEW, SendMessageW, WM_CLOSE, WM_COMMAND,
                  SW_SHOWDEFAULT,
                  },
        wingdi::{GetStockObject, WHITE_BRUSH},
        libloaderapi::{GetModuleHandleW, LoadStringW, },
        shellapi::{ShellExecuteW},
    },
    shared::{
        windef::{HWND, HBRUSH},
        minwindef::{UINT, WPARAM, LPARAM, LRESULT, LOWORD},
    },
};

use std::ptr;
use std::mem;
use std::mem::MaybeUninit;

mod utility;
use utility::encode;

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
            TranslateMessage(&mut msg);
            DispatchMessageW(&mut msg);
        }
    }

    let n = Node::new(0.0, 0.0, 0.0, 0.0);

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
    match msg {
        WM_COMMAND => {
            let id = LOWORD(w_param as u32) as i32;
            if id == 105 {
                SendMessageW(hwnd, WM_CLOSE, 0,0);
            } else if id == 602 {
                About::show(hwnd);
            } else if id == 600 {
                ShellExecuteW(hwnd, encode("open").as_ptr(), encode("https://github.com/kenjinote/LOGIC").as_ptr(), ptr::null(), ptr::null(), SW_SHOWDEFAULT);
            }
        },
        WM_DESTROY => PostQuitMessage(0),
        _ => return DefWindowProcW(hwnd, msg, w_param, l_param),
    };
    0
}
use winapi::{
    um::{
        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW,
                  CW_USEDEFAULT, MAKEINTRESOURCEW, SendMessageW, WM_CLOSE, WM_COMMAND,
                  WM_INITDIALOG, DialogBoxParamW, EndDialog, SW_SHOWDEFAULT,CreateDialogParamW
        },
        wingdi::{GetStockObject, WHITE_BRUSH},
        libloaderapi::{GetModuleHandleW, LoadStringW, },
    },
    shared::{
        windef::{HWND, HBRUSH},
        minwindef::{UINT, WPARAM, LPARAM, LRESULT, LOWORD},
    },
};

use std::ptr;

use crate::utility::encode;

pub struct Debug {
}

impl Debug {
    pub unsafe fn show(hparent: HWND) -> HWND {
        CreateDialogParamW(GetModuleHandleW(ptr::null()),MAKEINTRESOURCEW(4),hparent,Some(Self::dialog_proc),0 as LPARAM)
    }
    pub unsafe extern "system" fn dialog_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
        match msg {
            WM_INITDIALOG => {
                return 1;
            },
            WM_COMMAND =>
                match LOWORD(w_param as UINT) {
                    1 | 2 => {
                        EndDialog(hwnd, LOWORD(w_param as UINT).try_into().unwrap());
                        return 1;
                    },
                    _ => (),
                },
            _ => (),
        }
        return 0
    }
}
use winapi::{
    um::{
        commdlg::{GetOpenFileNameW, OFN_FILEMUSTEXIST, OPENFILENAMEW, OFN_PATHMUSTEXIST},
        winuser::{RegisterClassW, WNDCLASSW, CS_HREDRAW, CS_VREDRAW,
                  LoadIconW, LoadCursorW, IDC_ARROW,
                  CreateWindowExW, ShowWindow, SW_NORMAL, UpdateWindow,
                  GetMessageW, TranslateMessage, DispatchMessageW, MSG,
                  WM_DESTROY, PostQuitMessage, DefWindowProcW, WS_OVERLAPPEDWINDOW,
                  CW_USEDEFAULT, MAKEINTRESOURCEW, SendMessageW, WM_CLOSE, WM_COMMAND,
                  WM_INITDIALOG, DialogBoxParamW, EndDialog, SW_SHOWDEFAULT,
                  },
        wingdi::{GetStockObject, WHITE_BRUSH},
        libloaderapi::{GetModuleHandleW, LoadStringW, },
    },
    shared::{
        windef::{HWND, HBRUSH,},
        minwindef::{UINT, WPARAM, LPARAM, LRESULT, LOWORD},
    },
};

use std::ptr;

use crate::utility::encode;
use crate::utility::decode;

pub struct Open {
}

impl Open {
    pub unsafe fn show(hparent: HWND) {

        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        let mut szFile: [u16; 260] = [0; 260];
        let lpszTitle = encode("Open");
        let lpstrFilter = encode("logic file\0*.logic\0All\0*.*\0");
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = hparent;
        ofn.lpstrFilter = lpstrFilter.as_ptr();
        ofn.lpstrFile = szFile.as_mut_ptr();
        ofn.nMaxFile = 260;
        ofn.Flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST;
        ofn.lpstrTitle = lpszTitle.as_ptr();

        if GetOpenFileNameW(&mut ofn) != 0 {
            println!("File selected: {}", decode(&szFile[0]));
        }

    }
}
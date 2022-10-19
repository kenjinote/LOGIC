use winapi::{
    um::{
        commdlg::{GetSaveFileNameW, OFN_ALLOWMULTISELECT, OFN_EXPLORER, OPENFILENAMEW},
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

pub struct Save {
}

impl Save {
    pub unsafe fn show(hparent: HWND) {

        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        let mut szFile: [u16; 260] = [0; 260];
        let lpszTitle = encode("Save");
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = hparent;
        ofn.lpstrFilter = encode("All\0*.*\0Text\0*.TXT\0").as_ptr();
        ofn.lpstrFile = szFile.as_mut_ptr();
        ofn.nMaxFile = 260;
        ofn.Flags = OFN_ALLOWMULTISELECT | OFN_EXPLORER;
        ofn.lpstrTitle = lpszTitle.as_ptr();

        if GetSaveFileNameW(&mut ofn) != 0 {
            //println!("File selected: {}", encode(&szFile));
        }

    }
}
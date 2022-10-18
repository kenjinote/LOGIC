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

pub struct Open {
}

impl Open {
    pub unsafe fn show(hparent: HWND) {

        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        let mut szFile: [u16; 260] = [0; 260];
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = hparent;
        ofn.lpstrFilter = encode("All\0*.*\0Text\0*.TXT\0").as_ptr();
        ofn.lpstrFile = szFile.as_mut_ptr();
        ofn.nMaxFile = 260;
        ofn.Flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST;
        ofn.lpstrTitle = encode("Open").as_ptr();

        if GetOpenFileNameW(&mut ofn) != 0 {
            //println!("File selected: {}", encode(&szFile));
        }
/*
        TCHAR fname[MAX_PATH] = {0};
        TCHAR ftitle[MAX_PATH] = {0};
        OPENFILENAME of = { 0 };
        lstrcpy(fname,TEXT("*.txt;*.htm*;*.cpp;*.c;*.h"));
        of.lStructSize=sizeof(OPENFILENAME);
        of.hwndOwner=hWnd;
        of.lpstrFilter=TEXT("ﾕｰｻﾞｰ指定\0*.*\0ﾃｷｽﾄﾌｧｲﾙ (*.txt)\0*.txt\0すべてのﾌｧｲﾙ (*.*)\0*.*\0\0");
        of.lpstrFile=fname;
        of.lpstrFileTitle=ftitle;
        of.nMaxFile=MAX_PATH;
        of.nMaxFileTitle=MAX_PATH;
        of.Flags=OFN_FILEMUSTEXIST|OFN_HIDEREADONLY;
        of.lpstrDefExt=TEXT("txt");
        of.lpstrTitle=TEXT("ファイルを開く");
        if(GetOpenFileName(&of)==0)return -1;
        SetWindowText(hWnd,fname);
*/
    }
}
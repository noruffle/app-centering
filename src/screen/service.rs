use winapi::um::winuser::{GetForegroundWindow, GetSystemMetrics, GetWindowRect, MoveWindow, SM_CXSCREEN, SM_CYSCREEN};
use winapi::shared::windef::{HWND, RECT};

pub struct Screen {
    pub width: i32,
    pub height: i32
}

impl Screen {
    pub fn new() -> Self {
        unsafe {
            Screen { 
                width: GetSystemMetrics(SM_CXSCREEN),
                height: GetSystemMetrics(SM_CYSCREEN) 
            }
        }
    }

    pub fn center(self) {
        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd != std::ptr::null_mut() {
                self.get(hwnd);
            } else {
                eprintln!("Can't get active window")
            }
        }
    }

    pub fn get(self, hwnd: HWND) {
        unsafe  {
            let screen = Screen::new();
        
            let mut rect: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0 };
            if GetWindowRect(hwnd, &mut rect) == 0 {
                eprintln!("Can't get screen sizes")
            }
        
            let (win_width, win_height) = (
                rect.right - rect.left,
                rect.bottom - rect.top
            );
        
            let (x, y) = (
                (screen.width - win_width) / 2,
                (screen.height - win_height) / 2
            );
        
            MoveWindow(hwnd, x, y, win_width, win_height, 1);
        }
    }
}
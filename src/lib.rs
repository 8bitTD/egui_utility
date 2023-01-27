#[derive(Debug,Clone, serde::Deserialize, serde::Serialize)] 
pub struct WindowRect{
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Default for WindowRect{
    fn default() -> Self {
        Self {
            left: 600,
            top: 200,
            right: 1100,
            bottom: 700
        }
    }
}
pub fn get_window_rect() -> WindowRect{
    let mut rect = WindowRect::default();
    unsafe{
        let hwnd = windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
        let mut w_rect = windows::Win32::Foundation::RECT::default();
        windows::Win32::UI::WindowsAndMessaging::GetWindowRect(hwnd, &mut w_rect);
        rect.left = w_rect.left;
        rect.top = w_rect.top;
        rect.right = w_rect.right;
        rect.bottom = w_rect.bottom;
    }
    rect
}
pub fn set_window_rect(rect: WindowRect){
    unsafe{
        let hwnd = windwos::Win32::UI::WindowsAndMessaging::GetForegroundWindow();
        windows::Win32::UI::WindowsAndMessaging::MoveWindow(hwnd, rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, true);
    }
}

pub fn get_rect_from_json(tn: String)-> WindowRect{//jsonからWindowのサイズを取得する処理
    let mut rect = WindowRect::default();
    let mut jsn_path = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let document = "\\Documents\\script\\Rust\\";
    let rust_path = format!("{}{}",&jsn_path, document);
    if !std::path::Path::new(&rust_path).is_dir(){Some(std::fs::create_dir_all(&rust_path));}
    jsn_path.push_str(format!("{}{}{}",document, &tn, "_ws.json").as_str());
    let contents = match std::fs::read_to_string(&jsn_path){
        Ok(contents) => contents,
        Err(_error) => {return rect;}
    };
    let wr:Result<WindowRect,_> = serde_json::from_str(&contents);
    if wr.is_ok(){
        let w = wr.unwrap();
        rect.left = w.left;
        rect.top = w.top;
        rect.right = w.right;
        rect.bottom = w.bottom;
    }
    rect
}  

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
        let hwnd = winapi::um::winuser::GetActiveWindow();
        let mut window: winapi::shared::windef::RECT = std::mem::zeroed();
        winapi::um::winuser::GetWindowRect(hwnd, &mut window);
        rect.left = window.left;
        rect.top = window.top;
        rect.right = window.right;
        rect.bottom = window.bottom;
    }
    rect
}
pub fn set_window_rect(rect: WindowRect){
    unsafe{
        let hwnd = winapi::um::winuser::GetActiveWindow();
        winapi::um::winuser::MoveWindow(hwnd,rect.left,rect.top,rect.right - rect.left,rect.bottom - rect.top,1);
    }
}

pub fn get_rect_from_json(tn: String)-> Option<WindowRect>{//jsonからWindowのサイズを取得する処理
    let mut rect: Option<WindowRect> = None;
    let mut jsn_path = dirs::home_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    let document = "\\Documents\\script\\Rust\\";
    let rust_path = format!("{}{}",&jsn_path, document);
    if !std::path::Path::new(&rust_path).is_dir(){Some(std::fs::create_dir_all(&rust_path));}
    jsn_path.push_str(format!("{}{}{}",document, &tn, "_ws.json").as_str());
    let contents = match std::fs::read_to_string(&jsn_path){
        Ok(contents) => contents,
        Err(_error) => {return None;}
    };
    let wr:Result<WindowRect,_> = serde_json::from_str(&contents);
    if wr.is_ok(){
        let w = wr.unwrap();
        rect = Some(w);
    }
    rect
}  

fn set_font(ctx: &eframe::egui::CtxRef, size: f32){
    let mut txt_font = eframe::egui::FontDefinitions::default();
    txt_font.family_and_size.insert(eframe::egui::epaint::text::TextStyle::Body,(eframe::egui::epaint::text::FontFamily::Proportional, size));
    txt_font.family_and_size.insert(eframe::egui::epaint::text::TextStyle::Button,(eframe::egui::epaint::text::FontFamily::Proportional, size));
    txt_font.font_data.insert("Meiryo".to_owned(), eframe::egui::FontData::from_static(include_bytes!("C:/Windows/Fonts/Meiryo.ttc")));
    txt_font.fonts_for_family
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default().insert(0, "Meiryo".to_owned());
    ctx.set_fonts(txt_font);
}
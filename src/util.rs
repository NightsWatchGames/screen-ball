use display_info::DisplayInfo;

pub fn primary_display() -> Option<DisplayInfo> {
    let all_displays = DisplayInfo::all();
    println!("all display info: {:?}", all_displays);
    if all_displays.is_none() {
        return None;
    }
    let all_displays = all_displays.unwrap();
    for display_info in all_displays {
        if display_info.is_primary {
            return Some(display_info);
        }
    }
    return None;
}
use display_info::DisplayInfo;

pub fn primary_display() -> Option<DisplayInfo> {
    let all_displays = DisplayInfo::all();
    println!("all display info: {:?}", all_displays);
    let all_displays = all_displays?;
    for display_info in all_displays {
        if display_info.is_primary {
            return Some(display_info);
        }
    }
    return None;
}

pub fn non_primary_displays() -> Option<Vec<DisplayInfo>> {
    let all_displays = DisplayInfo::all();
    println!("all display info: {:?}", all_displays);
    let all_displays = all_displays?;
    let mut non_primary_displays = Vec::new();
    for display_info in all_displays {
        if !display_info.is_primary {
            non_primary_displays.push(display_info);
        }
    }
    return Some(non_primary_displays);
}
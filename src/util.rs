use display_info::DisplayInfo;

// TODO replace with NonSend<EventLoopProxy> when bevy 0.14 released
pub fn primary_display() -> DisplayInfo {
    let all_displays = DisplayInfo::all().expect("Failed to get all display info");
    println!("all display info: {:?}", all_displays);
    for display_info in all_displays {
        if display_info.is_primary {
            return display_info;
        }
    }
    panic!("There is no primary display");
}

pub fn non_primary_displays() -> Vec<DisplayInfo> {
    let all_displays = DisplayInfo::all().expect("Failed to get all display info");
    let mut non_primary_displays = Vec::new();
    for display_info in all_displays {
        if !display_info.is_primary {
            non_primary_displays.push(display_info);
        }
    }
    println!("non primary display info: {:?}", non_primary_displays);
    return non_primary_displays;
}

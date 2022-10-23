use druid::widget::{Svg, SvgData};

// macros for easier loading w/out repeating
#[cfg(windows)]
macro_rules! icons_folder {
    () => {"..\\..\\..\\icons\\"}
}

#[cfg(not(windows))]
macro_rules! icons_folder {
    () => {"../../../icons/"}
}

macro_rules! get_icon {
    ($icon_name: expr) => {
        include_str!(concat!(icons_folder!(), $icon_name))
    }
}

// icons
pub static LOGO: &str = get_icon!("logo.svg");

pub fn load(svg_data: &str) -> Svg {
    match svg_data.parse::<SvgData>() {
        Ok(svg) => Svg::new(svg),
        Err(err) => {
            eprint!("Failed to load svg: {err}");
            Svg::new(SvgData::default())
        }
    }
}
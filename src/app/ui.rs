use druid::{
    Color,
    WindowDesc,
    Widget,
    WidgetExt,
    widget::{
        Flex,
        Label,
        Split,
        CrossAxisAlignment
    }
};

mod theme;
mod icons;

pub fn main_window() -> WindowDesc<()> {
    WindowDesc::new(root_widget)
        .window_size((600.0, 450.0))
        .with_min_size((450.0, 300.0))
        .show_titlebar(false)
}

fn root_widget() -> impl Widget<()> {
    // use a List widget for pages.
    let pages = Flex::column()
        .expand_height();

    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(icons::load(icons::LOGO))
        .with_default_spacer()
        .with_flex_child(pages, 1.0)
        .with_default_spacer()
        .with_child(Label::new("Close"))
        .background(theme::LIGHT);

    let topbar = Flex::row()
        .must_fill_main_axis(true)
        .background(theme::DARK);

    let main = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(topbar)
        .background(theme::DARK);

    let split = Split::columns(sidebar, main)
        .split_point(0.2)
        .bar_size(2.0)
        .min_bar_area(2.0)
        .min_size(150.0, 300.0)
        .solid_bar(true);

    split
}

fn sidebar_logo_widget() {

}
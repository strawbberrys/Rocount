use druid::{
    WindowDesc,
    Widget,
    WidgetExt,
    widget::{
        Flex,
        Label,
        Split,
        Button,
        CrossAxisAlignment,
        MainAxisAlignment,
    }
};

mod theme;
mod icon;
mod widget;

use widget::{Border, BorderPosition};

pub fn main_window() -> WindowDesc<()> {
    WindowDesc::new(root_widget)
        .window_size((600.0, 450.0))
        .with_min_size((450.0, 300.0))
        .show_titlebar(false)
}

fn root_widget() -> impl Widget<()> {
    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(icon::load(icon::LOGO))
        .with_default_spacer()
        .with_flex_child(pages_widget(), 1.0)
        .background(theme::LIGHT);

    let topbar = Flex::row()
        .must_fill_main_axis(true)
        .main_axis_alignment(MainAxisAlignment::End)
        .with_child(Button::new("-"))
        .with_child(Button::new("X"))
        .background(Border::new(BorderPosition::Bottom, theme::LIGHT, 1.0));

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

fn pages_widget() -> Flex<()> {
    let accounts_button = Button::new("Accounts");
    let settings_button = Button::new("Settings");

    let pages = Flex::column()
        .with_child(accounts_button)
        .with_default_spacer()
        .with_child(settings_button);

    pages
}
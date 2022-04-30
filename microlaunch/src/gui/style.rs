use iced::*;

macro_rules! color {
    ($nm:ident ($r:literal $g:literal $b:literal)) => {
        lazy_static::lazy_static! {
            pub static ref $nm: iced::Color = iced::Color::from_rgb8($r, $g, $b);
        }
    };

    ($nm:ident ($r:literal $g:literal $b:literal $a:literal)) => {
        lazy_static::lazy_static! {
            pub static ref $nm: iced::Color = iced::Color::from_rgba8($r, $g, $b, $a);
        }
    }
}

macro_rules! bg {
    ($nm:ident) => {
        iced::Background::Color(*$nm)
    }
}

color!(WHITE (232 232 232));
color!(GRAY (64 64 64));
color!(LIGHTGRAY (107 107 107));
color!(DARKGRAY (20 20 20));
color!(SLIGHTLYLIGHTERDARKGRAY (26 26 26));
color!(LIGHTDARKGRAY (40 40 40));
color!(TRANSPARENTLIGHTBLUEISH (46 132 201 0.40));
color!(BLACK (3 3 3));
color!(RED (235 76 59));

pub struct UlButtonStylesheet;
impl button::StyleSheet for UlButtonStylesheet {
    fn active(&self) -> button::Style {
        button::Style {
            shadow_offset: Default::default(),
            background: Some(bg!(DARKGRAY)),
            border_radius: 5.0,
            border_width: 1.0,
            border_color: *GRAY,
            text_color: *WHITE,
        }
    }

    fn hovered(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            background: Some(bg!(LIGHTDARKGRAY)),
            ..active
        }
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            shadow_offset: Vector::default(),
            background: Some(bg!(BLACK)),
            ..self.active()
        }
    }

    fn disabled(&self) -> button::Style {
        let active = self.active();

        button::Style {
            shadow_offset: Vector::default(),
            background: active.background.map(|background| match background {
                Background::Color(color) => Background::Color(Color {
                    a: color.a * 0.5,
                    ..color
                }),
            }),
            text_color: Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
    
}

pub struct UlPickListStylesheet;
impl pick_list::StyleSheet for UlPickListStylesheet {
    fn menu(&self) -> iced_style::menu::Style {
        iced_style::menu::Style {
            background: bg!(DARKGRAY),
            selected_background: bg!(LIGHTDARKGRAY),
            text_color: *LIGHTGRAY,
            selected_text_color: *WHITE,
            border_color: *GRAY,
            border_width: 1.0,
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            text_color: *WHITE,
            background: bg!(SLIGHTLYLIGHTERDARKGRAY),
            border_radius: 3.0,
            border_width: 1.0,
            border_color: *GRAY,
            icon_size: 0.5,
        }
    }

    fn hovered(&self) -> pick_list::Style {
        self.active()
    }
}

pub struct UlCheckboxStylesheet;
impl checkbox::StyleSheet for UlCheckboxStylesheet {
    fn active(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: bg!(DARKGRAY),
            checkmark_color: *WHITE,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: *GRAY,
        }
    }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style {
        checkbox::Style {
            background: bg!(SLIGHTLYLIGHTERDARKGRAY),
            checkmark_color: *WHITE,
            border_radius: 5.0,
            border_width: 1.0,
            border_color: *GRAY,
        }
    }
}

pub struct UlTextInputStylesheet;
impl text_input::StyleSheet for UlTextInputStylesheet {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: bg!(DARKGRAY),
            ..Default::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: bg!(SLIGHTLYLIGHTERDARKGRAY),
            ..Default::default()
        }
    }

    fn placeholder_color(&self) -> iced::Color {
        *LIGHTGRAY
    }

    fn value_color(&self) -> iced::Color {
        *WHITE
    }

    fn selection_color(&self) -> iced::Color {
        *TRANSPARENTLIGHTBLUEISH
    }
}
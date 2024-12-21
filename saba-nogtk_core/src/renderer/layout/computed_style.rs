use core::cell::RefCell;

use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
};

use crate::{
    error::Error,
    renderer::dom::node::{ElementKind, Node, NodeKind},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyle {
    background_color: Option<Color>,
    color: Option<Color>,
    display: Option<DisplayType>,
    font_size: Option<FontSize>,
    text_decoration: Option<TextDecoration>,
    height: Option<f64>,
    width: Option<f64>,
}

impl ComputedStyle {
    pub fn new() -> Self {
        Self {
            background_color: None,
            color: None,
            display: None,
            font_size: None,
            text_decoration: None,
            height: None,
            width: None,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = Some(color);
    }

    pub fn background_color(&self) -> Color {
        self.background_color
            .clone()
            .expect("failed to access background_color")
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn color(&self) -> Color {
        self.color.clone().expect("failed to access color")
    }

    pub fn set_display(&mut self, display: DisplayType) {
        self.display = Some(display);
    }

    pub fn display(&self) -> DisplayType {
        self.display.clone().expect("failed to access display")
    }

    pub fn font_size(&self) -> FontSize {
        self.font_size.expect("failed to access font_size")
    }

    pub fn text_decoration(&self) -> TextDecoration {
        self.text_decoration
            .expect("failed to access text_decoration")
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = Some(height);
    }

    pub fn height(&self) -> f64 {
        self.height.expect("failed to access height")
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = Some(width);
    }

    pub fn width(&self) -> f64 {
        self.width.expect("failed to access width")
    }

    pub fn defaulting(&mut self, node: &Rc<RefCell<Node>>, parent_style: Option<ComputedStyle>) {
        if let Some(parent_style) = parent_style {
            if self.background_color.is_none() && parent_style.background_color() != Color::white()
            {
                self.background_color = Some(parent_style.background_color());
            }
            if self.color.is_none() && parent_style.color() != Color::black() {
                self.color = Some(parent_style.color());
            }
            if self.font_size.is_none() && parent_style.font_size() != FontSize::Medium {
                self.font_size = Some(parent_style.font_size());
            }
            if self.text_decoration.is_none()
                && parent_style.text_decoration() != TextDecoration::None
            {
                self.text_decoration = Some(parent_style.text_decoration());
            }
        }

        if self.background_color.is_none() {
            self.background_color = Some(Color::white());
        }
        if self.color.is_none() {
            self.color = Some(Color::black());
        }
        if self.display.is_none() {
            self.display = Some(DisplayType::default(node))
        }
        if self.font_size.is_none() {
            self.font_size = Some(FontSize::default(node));
        }
        if self.text_decoration.is_none() {
            self.text_decoration = Some(TextDecoration::default(node));
        }
        if self.height.is_none() {
            self.height = Some(0.0);
        }
        if self.width.is_none() {
            self.width = Some(0.0);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    name: Option<String>,
    code: String,
}

impl Color {
    pub fn from_name(name: &str) -> Result<Self, Error> {
        let code = match name {
            "black" => "#000000".to_string(),
            "silver" => "#c0c0c0".to_string(),
            "gray" => "#808080".to_string(),
            "white" => "#ffffff".to_string(),
            "maroon" => "#800000".to_string(),
            "red" => "#ff0000".to_string(),
            "purple" => "#800080".to_string(),
            "fuchsia" => "ff00ff".to_string(),
            "green" => "#00ff00".to_string(),
            "lime" => "#00ff00".to_string(),
            "olive" => "#808000".to_string(),
            "yellow" => "#ffff00".to_string(),
            "navy" => "#000080".to_string(),
            "blue" => "#0000ff".to_string(),
            "teal" => "#008080".to_string(),
            "aqua" => "#00ffff".to_string(),
            "orange" => "ffa500".to_string(),
            "lightgray" => "#d3d3d3".to_string(),
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color name{:?} is not supproted yet",
                    name
                )))
            }
        };

        Ok(Self {
            name: Some(name.to_string()),
            code,
        })
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        if code.chars().nth(0) != Some('#') || code.len() != 7 {
            return Err(Error::UnexpectedInput(format!(
                "color code {:?} is invalid",
                code
            )));
        }

        let name = match code {
            "#000000" => "black",
            "#c0c0c0" => "silver",
            "#808080" => "gray",
            "#ffffff" => "white",
            "#800000" => "maroon",
            "#ff0000" => "red",
            "#800080" => "purple",
            "#ff00ff" => "fuchsia",
            "#00ff00" => "green",
            "#808000" => "olive",
            "#ffff00" => "yellow",
            "#000080" => "navy",
            "#0000ff" => "blue",
            "#008080" => "teal",
            "#00ffff" => "aqua",
            "ffa500" => "orange",
            "#d3d3d3" => "lightgray",
            _ => {
                return Err(Error::UnexpectedInput(format!(
                    "color code {:?} is not supproted yet",
                    code
                )))
            }
        };

        Ok(Self {
            name: Some(name.to_string()),
            code: code.to_string(),
        })
    }

    pub fn white() -> Self {
        Self {
            name: Some("white".to_string()),
            code: "#ffffff".to_string(),
        }
    }

    pub fn black() -> Self {
        Self {
            name: Some("black".to_string()),
            code: "#000000".to_string(),
        }
    }

    pub fn code_u32(&self) -> u32 {
        u32::from_str_radix(self.code.trim_start_matches('#'), 16).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FontSize {
    Medium,
    XLarge,
    XXLarge,
}

impl FontSize {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::H1 => Self::XXLarge,
                ElementKind::H2 => Self::XLarge,
                _ => Self::Medium,
            },
            _ => Self::Medium,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DisplayType {
    Block,
    Inline,
    DisplayNone,
}

impl DisplayType {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Document => Self::Block,
            NodeKind::Element(element) => {
                if element.is_block_element() {
                    Self::Block
                } else {
                    Self::Inline
                }
            }
            NodeKind::Text(_) => Self::Inline,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "block" => Ok(Self::Block),
            "inline" => Ok(Self::Inline),
            "none" => Ok(Self::DisplayNone),
            _ => Err(Error::UnexpectedInput(format!(
                "display type {:?} is not supported yet",
                s
            ))),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TextDecoration {
    None,
    Underline,
}

impl TextDecoration {
    fn default(node: &Rc<RefCell<Node>>) -> Self {
        match &node.borrow().kind() {
            NodeKind::Element(element) => match element.kind() {
                ElementKind::A => Self::Underline,
                _ => Self::None,
            },
            _ => TextDecoration::None,
        }
    }
}

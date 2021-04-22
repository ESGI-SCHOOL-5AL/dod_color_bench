#[derive(Debug, Clone, PartialEq)]
pub enum ShapeType {
    RECT,
    CIRCLE
}
#[derive(Debug, Clone, PartialEq)]
pub struct Shape {
    pub left_conner_x: i32,
    pub left_conner_y: i32,
    pub width: u16,
    pub height: u16,
    pub color: String,
    pub shape_type: ShapeType
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shapes {
    pub left_conners_x: Vec<i32>,
    pub left_conners_y: Vec<i32>,
    pub widths: Vec<u16>,
    pub heights: Vec<u16>,
    pub colors: Vec<String>,
    pub shapes_type: Vec<ShapeType>
}

impl Shapes {
    pub fn new() -> Self {
        return Shapes {
            left_conners_x: Vec::new(),
            left_conners_y: Vec::new(),
            widths: Vec::new(),
            heights: Vec::new(),
            colors: Vec::new(),
            shapes_type: Vec::new()
        }
    }
}

pub fn count_number_rect_oop(shapes: &[Shape]) -> (u16, u16) {
    return shapes.iter()
        .fold((0, 0), |acc, shape| {
            return match shape.shape_type {
                ShapeType::RECT => (acc.0 + 1, acc.1),
                ShapeType::CIRCLE => (acc.0, acc.1 + 1)
            };
        });
}

pub fn count_number_rect_dod(shapes: &[ShapeType]) -> (u16, u16) {
    return shapes.iter()
    .fold((0, 0), |acc, shape_type| {
        return match shape_type {
            ShapeType::RECT => (acc.0 + 1, acc.1),
            ShapeType::CIRCLE => (acc.0, acc.1 + 1)
        };
    });
}

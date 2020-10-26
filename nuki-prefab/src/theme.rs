use nuki::nuklear as nk;

pub enum Theme {
    White,
    Blue,
    Dark,
}

impl Into<[nk::Color; 28usize]> for Theme {
    fn into(self) -> [nk::Color; 28usize] {
        match self {
            Theme::Blue => {
                [
                    nk::color_rgba(20, 20, 20, 255),    // Text
                    nk::color_rgba(202, 212, 214, 215), // Window
                    nk::color_rgba(137, 182, 224, 220), // Header
                    nk::color_rgba(140, 159, 173, 255), // Border
                    nk::color_rgba(137, 182, 224, 255), // Button
                    nk::color_rgba(142, 187, 229, 255), // Button Hover
                    nk::color_rgba(147, 192, 234, 255), // Button Active
                    nk::color_rgba(177, 210, 210, 255), // Toggle
                    nk::color_rgba(182, 215, 215, 255), // Toggle Hover
                    nk::color_rgba(137, 182, 224, 255), // Toggle Cursor
                    nk::color_rgba(177, 210, 210, 255), // Select
                    nk::color_rgba(137, 182, 224, 255), // Select Active
                    nk::color_rgba(177, 210, 210, 255), // Slider
                    nk::color_rgba(137, 182, 224, 245), // Slider Cursor
                    nk::color_rgba(142, 188, 229, 255), // Slider Cursor Hover
                    nk::color_rgba(147, 193, 234, 255), // Slider Cusor Active
                    nk::color_rgba(210, 210, 210, 255), // Property
                    nk::color_rgba(210, 210, 210, 255), // Edit
                    nk::color_rgba(20, 20, 20, 255),    // Edit Cursor
                    nk::color_rgba(210, 210, 210, 255), // Combo
                    nk::color_rgba(210, 210, 210, 255), // Chart
                    nk::color_rgba(137, 182, 224, 255), // Chart Color
                    nk::color_rgba(255, 0, 0, 255),     // Chart Color Highlight
                    nk::color_rgba(190, 200, 200, 255), // Scrollbar
                    nk::color_rgba(64, 84, 95, 255),    // Scrollbar Cursor
                    nk::color_rgba(70, 90, 100, 255),   // Scrollbar Cursor Hover
                    nk::color_rgba(75, 95, 105, 255),   // Scrollbar Cursor Active
                    nk::color_rgba(156, 193, 220, 255), // Tab Header
                ]
            }
            _ => unimplemented!(),
        }
    }
}

impl Into<nk::ColorMap> for Theme {
    fn into(self) -> nk::ColorMap {
        let colors: [nk::Color; 28usize] = Theme::Blue.into();
        nk::ColorMap::from(colors)
    }
}

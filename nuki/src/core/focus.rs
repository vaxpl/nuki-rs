use crate::{Data, WidgetId};

/// A list to controll the focus of the widgets.
#[derive(Debug, Default)]
pub struct FocusChain {
    widgets: Vec<WidgetId>,
    actived: Option<WidgetId>,
    focused: Option<WidgetId>,
}

impl FocusChain {
    /// Construct a new focus chain.
    pub fn new() -> Self {
        Self {
            widgets: vec![],
            actived: None,
            focused: None,
        }
    }

    /// Add `widget` to the chain.
    pub fn add_widget(&mut self, widget: WidgetId) {
        println!("Widget {:?} Added", widget);
        self.widgets.push(widget)
    }

    /// Remove `widget` to the chain.
    pub fn remove_widget(&mut self, widget: WidgetId) {
        if let Some(index) = self.widgets.iter().position(|x| *x == widget) {
            self.widgets.remove(index);
        }
    }

    /// Return `true` if the `widget` was actived in the chain.
    pub fn is_actived(&self, widget: WidgetId) -> bool {
        self.actived.map_or(false, |x| x == widget)
    }

    /// Return `true` if the `widget` was focused in the chain.
    pub fn is_focused(&self, widget: WidgetId) -> bool {
        self.focused.map_or(false, |x| x == widget)
    }
}

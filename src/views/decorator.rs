use floem_reactive::create_effect;
use glazier::kurbo::{Point, Rect};

use crate::{
    animate::Animation,
    app_handle::StyleSelector,
    event::{Event, EventListener},
    menu::Menu,
    responsive::ScreenSize,
    style::Style,
    view::View,
};

pub trait Decorators: View + Sized {
    fn style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style(style);
        });
        self
    }

    fn base_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_base_style(style);
        });
        self
    }

    /// The visual style to apply when the mouse hovers over the element
    fn hover_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::Hover);
        });
        self
    }

    /// The visual style to apply when the mouse hovers over the element
    fn dragging_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::Dragging);
        });
        self
    }

    fn focus_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::Focus);
        });
        self
    }

    /// Similar to the `:focus-visible` css selector, this style only activates when tab navigation is used.
    fn focus_visible_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::FocusVisible);
        });
        self
    }

    /// Allows the element to be navigated to with the keyboard. Similar to setting tabindex="0" in html.
    fn keyboard_navigatable(self) -> Self {
        let id = self.id();
        id.keyboard_navigatable();
        self
    }

    fn draggable(self) -> Self {
        let id = self.id();
        id.draggable();
        self
    }

    fn active_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::Active);
        });
        self
    }

    fn disabled_style(self, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_style_selector(style, StyleSelector::Disabled);
        });
        self
    }

    fn responsive_style(self, size: ScreenSize, style: impl Fn() -> Style + 'static) -> Self {
        let id = self.id();
        create_effect(move |_| {
            let style = style();
            id.update_responsive_style(style, size);
        });
        self
    }

    fn disabled(self, disabled_fn: impl Fn() -> bool + 'static) -> Self {
        let id = self.id();

        create_effect(move |_| {
            let is_disabled = disabled_fn();
            id.update_disabled(is_disabled);
        });

        self
    }

    fn on_event(self, listener: EventListener, action: impl Fn(&Event) -> bool + 'static) -> Self {
        let id = self.id();
        id.update_event_listener(listener, Box::new(action));
        self
    }

    fn on_click(self, action: impl Fn(&Event) -> bool + 'static) -> Self {
        let id = self.id();
        id.update_event_listener(EventListener::Click, Box::new(action));
        self
    }

    fn on_double_click(self, action: impl Fn(&Event) -> bool + 'static) -> Self {
        let id = self.id();
        id.update_event_listener(EventListener::DoubleClick, Box::new(action));
        self
    }

    fn on_secondary_click(self, action: impl Fn(&Event) -> bool + 'static) -> Self {
        let id = self.id();
        id.update_event_listener(EventListener::SecondaryClick, Box::new(action));
        self
    }

    fn on_resize(self, action: impl Fn(Point, Rect) + 'static) -> Self {
        let id = self.id();
        id.update_resize_listener(Box::new(action));
        self
    }

    fn on_cleanup(self, action: impl Fn() + 'static) -> Self {
        let id = self.id();
        id.update_cleanup_listener(Box::new(action));
        self
    }

    fn animation(self, anim: Animation) -> Self {
        let id = self.id();
        create_effect(move |_| {
            id.update_animation(anim.clone());
        });
        self
    }

    fn window_scale(self, scale_fn: impl Fn() -> f64 + 'static) -> Self {
        let id = self.id();

        create_effect(move |_| {
            let window_scale = scale_fn();
            id.update_window_scale(window_scale);
        });
        self
    }

    /// Adds a secondary-click context menu to the view, which opens at the mouse position.
    fn context_menu(self, menu: impl Fn() -> Menu + 'static) -> Self {
        let id = self.id();
        id.update_context_menu(Box::new(menu));
        self
    }

    /// Adds a primary-click context menu, which opens below the view.
    fn popout_menu(self, menu: impl Fn() -> Menu + 'static) -> Self {
        let id = self.id();
        id.update_popout_menu(Box::new(menu));
        self
    }
}

impl<V: View> Decorators for V {}

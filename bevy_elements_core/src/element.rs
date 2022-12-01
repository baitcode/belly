use bevy::utils::HashMap;
use bevy::utils::HashSet;

use crate::property::*;
use crate::tags;
use crate::tags::*;
use bevy::prelude::*;

#[derive(Default)]
pub enum DisplayElement {
    #[default]
    Block,
    Inline,
    // TODO: deside if it event needed
    // InlineBlock,
}

#[derive(Component, Default)]
pub struct Element {
    pub name: Option<Tag>,
    pub id: Option<Tag>,
    pub classes: HashSet<Tag>,
    pub state: HashSet<Tag>,
    pub display: DisplayElement,
    pub content: Option<Entity>,
    pub styles: HashMap<Tag, PropertyValues>,
}

impl Element {
    pub fn is_virtual(&self) -> bool {
        self.name.is_none()
    }
    pub fn inline() -> Element {
        Element {
            display: DisplayElement::Inline,
            ..default()
        }
    }
    pub fn invalidate(&mut self) {}
    pub fn focused(&self) -> bool {
        self.state.contains(&tags::focus())
    }
    pub fn focus(&mut self) {
        self.state.insert(tags::focus_request());
    }
}

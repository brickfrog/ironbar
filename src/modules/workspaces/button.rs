use super::open_state::OpenState;
use crate::gtk_helpers::IronbarGtkExt;
use crate::image::IconButton;
use crate::modules::workspaces::WorkspaceItemContext;
use crate::modules::workspaces::WorkspaceRequestMessage;
use crate::try_send;
use gtk::Button as GtkButton;
use gtk::prelude::*;

const WORKSPACE_ID_KEY: &str = "workspace-id";
const WORKSPACE_NAME_KEY: &str = "workspace-name";

#[derive(Debug)]
pub struct Button {
    button: IconButton,
}

impl Button {
    pub fn new(id: i64, name: &str, open_state: OpenState, context: &WorkspaceItemContext) -> Self {
        let label = context.name_map.get(name).map_or(name, String::as_str);
        let button = IconButton::new(label, &context.icon_theme, context.icon_size);
        button.set_widget_name(name);
        button.add_class("item");

        // Store both the ID and name in the button
        button.set_tag(WORKSPACE_ID_KEY, id);
        button.set_tag(WORKSPACE_NAME_KEY, name.to_string());

        let tx = context.tx.clone();
        button.connect_clicked(move |btn| {
            // Get the current ID from the button
            let id = btn.get_tag::<i64>(WORKSPACE_ID_KEY).copied().unwrap_or(-1);

            // If this is a favorite with ID -1 (empty workspace), use the name instead
            if id == -1 {
                if let Some(name) = btn.get_tag::<String>(WORKSPACE_NAME_KEY) {
                    try_send!(tx, WorkspaceRequestMessage::Name(name.clone()));
                    return;
                }
            }

            try_send!(tx, WorkspaceRequestMessage::Id(id));
        });

        let btn = Self { button };

        btn.set_open_state(open_state);
        btn
    }

    pub fn button(&self) -> &GtkButton {
        &self.button
    }

    pub fn set_open_state(&self, open_state: OpenState) {
        if open_state.is_visible() {
            self.button.add_class("visible");
        } else {
            self.button.remove_class("visible");
        }

        if open_state == OpenState::Focused {
            self.button.add_class("focused");
        } else {
            self.button.remove_class("focused");
        }

        if open_state == OpenState::Closed {
            self.button.add_class("inactive");
        } else {
            self.button.remove_class("inactive");
        }
    }

    pub fn set_urgent(&self, urgent: bool) {
        if urgent {
            self.button.add_class("urgent");
        } else {
            self.button.remove_class("urgent");
        }
    }

    pub fn workspace_id(&self) -> i64 {
        self.button
            .get_tag::<i64>(WORKSPACE_ID_KEY)
            .copied()
            .unwrap_or(-1)
    }

    pub fn set_workspace_id(&self, id: i64) {
        self.button.set_tag(WORKSPACE_ID_KEY, id);
    }
}

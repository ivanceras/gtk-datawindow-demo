//! database connection url dialogbox
use gtk;
use gtk::prelude::*;
use gtk::Dialog;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    ButtonBox, ComboBoxText, Entry, Grid, IconSize, Image, InputPurpose, Label, Stack,
    StackSwitcher, ToolButton, Toolbar,
};

pub enum Connection {
    Url(String),
    Scheme(ConnectionScheme),
}

pub struct ConnectionScheme {
    platform: String,
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,
}

impl Connection {
    pub fn new() -> Self {
        Connection::Url("postgres://postgre".to_owned())
    }

    pub fn build_widget(&self) -> Dialog {
        let title = "Connect to Server";
        let dialog = Dialog::new();
        dialog.set_title(title);
        let area = dialog.get_content_area();
        let stack = Stack::new();
        let stack_switcher = StackSwitcher::new();
        let simple = Self::connection_scheme_widget();
        let advance = Self::connection_url_widget();
        let vbox = gtk::Box::new(Vertical, 0);
        stack.add_titled(&simple, "simple", "Simple");
        stack.add_titled(&advance, "advance", "Advance");
        let stack_box = ButtonBox::new(Horizontal);
        stack_box.add(&stack_switcher);
        let form_box = ButtonBox::new(Horizontal);
        form_box.add(&stack);
        vbox.pack_start(&stack_box, true, true, 10);
        vbox.pack_start(&form_box, true, true, 10);
        stack_switcher.set_stack(Some(&stack));
        let action_buttons = Self::create_connect_cancel_buttons();
        vbox.pack_start(&action_buttons, true, true, 20);
        area.pack_start(&vbox, true, true, 0);
        dialog.set_default_size(300, 400);
        dialog.show_all();
        dialog
    }

    fn create_connect_cancel_buttons() -> gtk::Box {
        let toolbox = Toolbar::new();
        let ok_icon = Image::from_icon_name(Some("dialog-apply"), IconSize::LargeToolbar);
        let ok_btn = ToolButton::new::<Image>(Some(&ok_icon), Some("Connect"));
        ok_btn.set_is_important(true);
        let cancel_icon = Image::from_icon_name(Some("window-close"), IconSize::LargeToolbar);
        let cancel_btn = ToolButton::new::<Image>(Some(&cancel_icon), Some("Cancel"));
        cancel_btn.set_is_important(true);
        toolbox.add(&cancel_btn);
        toolbox.add(&ok_btn);
        let hbox = gtk::Box::new(Horizontal, 0);
        hbox.pack_end(&toolbox, false, false, 0);
        hbox
    }

    fn connection_url_widget() -> gtk::Box {
        let bbox = gtk::Box::new(Horizontal, 10);
        let label = Label::new(Some("Connection Url: "));
        let entry = Entry::new();
        bbox.pack_start(&label, true, true, 0);
        bbox.pack_start(&entry, true, true, 0);
        bbox
    }

    fn connection_scheme_widget() -> Grid {
        let grid = Grid::new();

        let platform_lbl = Label::new(Some("platform"));
        let platform_cb = ComboBoxText::new();
        platform_cb.append(Some("pg"), "postgres");
        platform_cb.append(Some("my"), "mysql");
        platform_cb.append(Some("sq"), "sqlite");
        grid.attach(&platform_lbl, 0, 0, 1, 1);
        grid.attach(&platform_cb, 1, 0, 1, 1);

        let host_lbl = Label::new(Some("host"));
        let host_entry = Entry::new();
        grid.attach(&host_lbl, 0, 1, 1, 1);
        grid.attach(&host_entry, 1, 1, 1, 1);

        let port_lbl = Label::new(Some("port"));
        let port_entry = Entry::new();
        grid.attach(&port_lbl, 0, 2, 1, 1);
        grid.attach(&port_entry, 1, 2, 1, 1);

        let db_lbl = Label::new(Some("database"));
        let db_entry = Entry::new();
        grid.attach(&db_lbl, 0, 3, 1, 1);
        grid.attach(&db_entry, 1, 3, 1, 1);

        let user_lbl = Label::new(Some("user"));
        let user_entry = Entry::new();
        grid.attach(&user_lbl, 0, 4, 1, 1);
        grid.attach(&user_entry, 1, 4, 1, 1);

        let pwd_lbl = Label::new(Some("password"));
        let pwd_entry = Entry::new();
        pwd_entry.set_input_purpose(InputPurpose::Password);
        grid.attach(&pwd_lbl, 0, 5, 1, 1);
        grid.attach(&pwd_entry, 1, 5, 1, 1);

        grid
    }
}

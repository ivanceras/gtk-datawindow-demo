use crate::connection::Connection;
use crate::tab::DataTab;
use glib::Type;
use glib::Value;
use gtk;
use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::WindowPosition::Center;
use gtk::WindowType::Toplevel;
use gtk::{
    Adjustment, Button, CellRendererText, IconSize, Image, Inhibit, Label, ListStore, Notebook,
    ReliefStyle, ScrolledWindow, ToolButton, Toolbar, TreeView, TreeViewColumn, Window,
};

#[derive(Clone)]
pub struct DataWindow {
    window_list: TreeView,
    list_widget: ScrolledWindow,
    toolbar: Toolbar,
    open_tabs: Notebook,
    pub window: Window,
}

impl DataWindow {
    pub fn new() -> Self {
        let (window_list, list_widget) = Self::create_window_list();
        let toolbar = Self::create_toolbar();
        let open_tabs = Self::create_tabs();
        let window = Window::new(Toplevel);

        DataWindow {
            window_list: window_list,
            list_widget: list_widget,
            toolbar: toolbar,
            open_tabs: open_tabs,
            window: window,
        }
    }

    pub fn build_widget<'a>(&'a self) {
        self.window.set_title("DataWindow - GTK");
        self.window.set_default_size(1024, 768);
        let vbox = gtk::Box::new(Vertical, 0);
        vbox.add(&self.toolbar);
        self.window.add(&vbox);
        let hbox = gtk::Box::new(Horizontal, 0);
        vbox.add(&hbox);
        hbox.add(&self.list_widget);
        hbox.add(&self.open_tabs);
        self.window.set_position(Center);
        for i in 0..3 {
            self.add_data_tab(&format!("Window {}", i));
        }
        self.init_window_list_selection();
        let _this = self.clone();
        self.window.show_all();
        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
    }

    // this is a list of the items to be viewed
    fn create_window_list() -> (TreeView, ScrolledWindow) {
        let tree = TreeView::new();
        let column_types = [Type::String];
        let store = ListStore::new(&column_types);
        //let model = store.get_model();
        tree.set_model(Some(&store));
        tree.set_headers_visible(false);
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();
        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", 0);
        tree.append_column(&column);
        let _selection = tree.get_selection();
        for i in 0..50 {
            let iter = store.append();
            store.set_value(&iter, 0, &Value::from(&format!("Window {}", i)));
        }
        let scroll = ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
        scroll.add(&tree);
        scroll.set_size_request(200, 600);
        (tree, scroll)
    }

    fn init_window_list_selection(&self) {
        let this = self.clone();
        self.window_list.get_selection().connect_changed(move |_| {
            let (model, iter) = this.window_list.get_selection().get_selected().unwrap();
            if let Some(_path) = model.get_path(&iter) {
                let value = model.get_value(&iter, 0);
                let value_string: String = value
                    .get()
                    .expect("must not error")
                    .expect("must be a string");
                this.add_data_tab(&value_string);
                this.window.show_all(); //Notes: Important to call show all in order to redraw the added tabs
            }
        });
    }

    // add a toolbar to the vertical box of the window
    fn create_toolbar() -> Toolbar {
        let toolbar = Toolbar::new();
        let open_icon = Image::from_icon_name(Some("network-server"), IconSize::LargeToolbar);
        let open_button = ToolButton::new::<Image>(Some(&open_icon), Some("Connect to Server"));
        open_button.set_is_important(true);
        open_button.connect_clicked(move |_| {
            println!("Openning connection box");
            let connection = Connection::new();
            connection.build_widget();
        });
        toolbar.add(&open_button);
        toolbar
    }

    // add tabs to the horizontal box that divites the list from the contents of that list
    fn create_tabs() -> Notebook {
        let notebook = Notebook::new();
        notebook.set_scrollable(true);
        notebook
    }

    pub fn add_data_tab(&self, title: &str) {
        println!("Adding {}", title);
        let tab = gtk::Box::new(Horizontal, 0);
        let sheet = Label::new(Some(title));
        let close_btn = Button::new();

        close_btn.set_relief(ReliefStyle::None);
        close_btn.set_focus_on_click(false);
        let close_image = Image::from_icon_name(Some("window-close"), IconSize::Menu);
        close_btn.add(&close_image);
        tab.add(&sheet);
        tab.add(&close_btn);
        let data_tab = DataTab::new();
        let data_widget = data_tab.build_widget();
        data_widget.show_all(); // needed in order for setting current page works
        self.open_tabs.append_page(&data_widget, Some(&tab));
        self.open_tabs.set_tab_reorderable(&data_widget, true);
        self.open_tabs.set_tab_detachable(&data_widget, true);
        self.open_tabs.popup_enable();
        let page_num = self.open_tabs.page_num(&data_widget);
        println!("page num: {:?}", page_num);
        self.open_tabs.set_current_page(page_num);
        let notebook_clone = self.open_tabs.clone();
        close_btn.connect_clicked(move |_| {
            let index = notebook_clone.page_num(&data_widget);
            notebook_clone.remove_page(index);
        });
        tab.show_all();
    }
}

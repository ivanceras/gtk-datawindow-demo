#![deny(warnings)]












use gtk_datawindow_demo::window::DataWindow;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK");
        return;
    }

    let cwindow = DataWindow::new();
    cwindow.build_widget();
    gtk::main();
}

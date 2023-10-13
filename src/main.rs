use std::env;
use std::path::{Path, PathBuf};
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, ImageBuf, Data, Lens};

#[derive(Clone, Data, Lens)]
struct AppState {
    counter: i32,
    color: String,
}

#[allow(unused_parens)]
fn main() {
    let args: Vec<String> = env::args().collect();

    if (args.len() > 1 && Path::new(&args[1]).exists()) {
        println!("Found image at {:?} and opening...", args[1]);
    } else {
        println!("Opening blank canvas...");
    }

    let _ = start_ui();
}

fn start_ui() -> Result<(), PlatformError> {
    // Window builder. We set title and size
    let main_window = WindowDesc::new(ui_builder())
        .title("Rusty Tiler")
        .window_size((500.0, 500.0));

    // let icon_path = PathBuf::from("logo.png");

    // Data to be used in the app (=state)
    let data: AppState = AppState {
        counter: 0,
        color: String::from("#ff0000"),
    };

    // Run the app
    return AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data);
}

fn ui_builder() -> impl Widget<AppState> {
    // The label text will be computed dynamically based on the current locale and count
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &AppState, _env| (*data).counter.into());
    let label = Label::new(text).padding(5.0).center();

    // Two buttons with on_click callback
    let button_plus = Button::new("+1")
        .on_click(|_ctx, data: &mut AppState, _env| (*data).counter += 1)
        .padding(5.0);
    let button_minus = Button::new("-1")
        .on_click(|_ctx, data: &mut AppState, _env| (*data).counter -= 1)
        .padding(5.0);

    // Container for the two buttons
    let buttons_row = Flex::row()
        .with_child(button_plus)
        .with_spacer(1.0)
        .with_child(button_minus);

    // Text input for the hex color string
    let text_input = TextBox::new()
        .fix_width(100.0)
        .lens(AppState::color);

    // Container for the whole UI
    let ui = Flex::column()
        .with_child(label)
        .with_child(buttons_row)
        .with_child(text_input);
    
    return ui;
}

// fn gpt_garbage() -> impl Widget<AppState> {
//     // The label text will be computed dynamically based on the current locale and count
//     let text = LocalizedString::new("hello-counter")
//         .with_arg("count", |data: &Counter, _env| (*data).0.into());
//     let label = Label::new(text).padding(5.0).center();

//     // Two buttons with on_click callback
//     let button_plus = Button::new("+1")
//         .on_click(|_ctx, data: &mut AppState, _env| data.counter += 1)
//         .padding(5.0);
//     let button_minus = Button::new("-1")
//         .on_click(|_ctx, data: &mut AppState, _env| data.counter -= 1)
//         .padding(5.0);

//     // Container for the two buttons
//     let button_container = Flex::row()
//         .with_child(button_plus)
//         .with_spacer(1.0)
//         .with_child(button_minus);

//     // Text input for the hex color string
//     let text_input = TextBox::new()
//         .fix_width(100.0)
//         .lens(AppState::hex_string);

//     // Colored square that displays the selected color
//     let color_square = druid::widget::Fill::new(|data: &AppState, _env| data.color);

//     // Combine the components into a single UI
//     let ui = Flex::column()
//         .with_child(counter_label)
//         .with_child(button_container)
//         .with_child(text_input)
//         .with_child(color_square);

//     return ui;
// }
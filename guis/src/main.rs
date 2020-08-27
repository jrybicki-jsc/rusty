extern crate web_view;

use web_view::*;

fn main() {
    let res = web_view::builder()
        .title("Doors")
        .content(Content::Html(include_str!("pages/index.html")))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(0)
        .invoke_handler(invoke_handler)
        .run()
        .unwrap();
    println!("res: {:?}", res)
}

fn invoke_handler(wv: &mut WebView<usize>, arg: &str) -> WVResult {
    if arg == "exit" {
        println!("exiting!");
        wv.exit();
    }
    println!("Arg is: {}", arg);
    Ok(())
}

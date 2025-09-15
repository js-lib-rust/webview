#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod ipc;
mod logger;
mod service;

use std::{borrow::Cow, cell::RefCell, rc::Rc};

use clap::Parser;
use include_dir::{Dir, include_dir};
use log::{debug, info, trace};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    http::{Request, Response},
    webview::{WebView, WebViewBuilder},
};

static ASSETS_DIR: Dir = include_dir!("assets");

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short = 'v',
        long,
        default_value = "off",
        help = "logging level: off, error, warn, info, debug, trace"
    )]
    log_level: String,

    #[arg(
        short = 'f',
        long,
        help = "logging file path -- if not specified print logs to console"
    )]
    log_file: Option<String>,
}

fn main() -> wry::Result<()> {
    let args = Args::parse();
    logger::init(&args.log_level, &args.log_file);
    trace!("main()");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Webview Demo")
        .with_inner_size(wry::application::dpi::LogicalSize::new(1400, 820))
        .build(&event_loop)?;

    let webview_shared: Rc<RefCell<Option<WebView>>> = Rc::new(RefCell::new(None));
    let webview_clone = webview_shared.clone();

    let html_url = "app://localhost/index.htm";
    info!("Loading from: {}", html_url);
    let webview = WebViewBuilder::new(window)?
        .with_url(&html_url)?
        .with_custom_protocol("app".into(), custom_protocol_handler)
        .with_initialization_script(include_str!("init.js"))
        .with_ipc_handler(move |_window, request| {
            if let Some(web_view) = webview_clone.borrow().as_ref() {
                ipc::ipc_handler(web_view, request);
            }
        })
        .with_devtools(true)
        .build()?;

    *webview_shared.borrow_mut() = Some(webview);

    // Run event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Webview demo started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn custom_protocol_handler(
    request: &Request<Vec<u8>>,
) -> wry::Result<Response<Cow<'static, [u8]>>> {
    trace!(
        "custom_protocol_handler(request: &Request<Vec<u8>>) -> Result<Response<Cow<'static, [u8]>>>"
    );

    let path = request.uri().path();
    let extension = path.rsplit('.').next().unwrap_or("");
    let Some(file) = ASSETS_DIR.get_file(&path[1..]) else {
        let body = Cow::Owned(Vec::new());
        let response = Response::builder().status(404).body(body)?;
        return Ok(response);
    };
    let buffer = file.contents().to_vec();
    debug!("load file {}: {}", path, buffer.len());

    let response = Response::builder()
        .status(200)
        .header("Content-Type", mime_type(extension))
        .header("Content-Length", buffer.len().to_string())
        .header("Cache-Control", "private, max-age=3600")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(Cow::Owned(buffer))?;

    Ok(response)
}

fn mime_type(extension: &str) -> &'static str {
    match extension {
        "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "ico" => "image/x-icon",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
}

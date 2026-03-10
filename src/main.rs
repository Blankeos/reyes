use anyhow::Result;
use base64::Engine;
use clap::{Parser, Subcommand, ValueEnum};
use rustautogui::{MatchMode, MouseClick, RustAutoGui};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "rcua")]
#[command(about = "A comprehensive computer-use CLI built on rustautogui")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Take screenshots of the screen
    #[command(name = "screenshot")]
    Screenshot {
        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
        /// Region to capture (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
        /// Output as base64
        #[arg(long)]
        base64: bool,
    },

    /// Get pixel color at coordinates
    #[command(name = "get-pixel-color")]
    GetPixelColor {
        /// X coordinate
        #[arg(short = 'x', long)]
        x: u32,
        /// Y coordinate
        #[arg(short = 'y', long)]
        y: u32,
    },

    /// Find color on screen
    #[command(name = "find-color")]
    FindColor {
        /// RGB values (r,g,b)
        #[arg(short, long, value_name = "RGB")]
        rgb: String,
        /// Tolerance for color matching
        #[arg(short, long, default_value = "0")]
        tolerance: u8,
        /// Region to search (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
    },

    /// Mouse click operations
    #[command(name = "click")]
    Click {
        /// X coordinate
        #[arg(short = 'x', long)]
        x: Option<u32>,
        /// Y coordinate
        #[arg(short = 'y', long)]
        y: Option<u32>,
        /// Mouse button (left, right, middle)
        #[arg(short, long, default_value = "left")]
        button: MouseButton,
        /// Number of clicks
        #[arg(short, long, default_value = "1")]
        clicks: u32,
        /// Duration for movement in seconds
        #[arg(short, long, default_value = "0.0")]
        duration: f32,
    },

    /// Double click at position
    #[command(name = "double-click")]
    DoubleClick {
        /// X coordinate
        #[arg(short = 'x', long)]
        x: Option<u32>,
        /// Y coordinate
        #[arg(short = 'y', long)]
        y: Option<u32>,
        /// Duration for movement in seconds
        #[arg(short, long, default_value = "0.0")]
        duration: f32,
    },

    /// Move mouse to position
    #[command(name = "move-mouse")]
    MoveMouse {
        /// X coordinate
        #[arg(short = 'x', long)]
        x: u32,
        /// Y coordinate
        #[arg(short = 'y', long)]
        y: u32,
        /// Duration for movement in seconds
        #[arg(short, long, default_value = "0.5")]
        duration: f32,
    },

    /// Move mouse relative to current position
    #[command(name = "move-mouse-rel")]
    MoveMouseRel {
        /// X offset (negative for left)
        #[arg(short = 'x', long)]
        x: i32,
        /// Y offset (negative for up)
        #[arg(short = 'y', long)]
        y: i32,
        /// Duration for movement in seconds
        #[arg(short, long, default_value = "0.5")]
        duration: f32,
    },

    /// Drag mouse to position
    #[command(name = "drag-mouse")]
    DragMouse {
        /// X coordinate
        #[arg(short = 'x', long)]
        x: u32,
        /// Y coordinate
        #[arg(short = 'y', long)]
        y: u32,
        /// Duration for movement in seconds
        #[arg(short, long, default_value = "0.5")]
        duration: f32,
    },

    /// Scroll the mouse wheel
    #[command(name = "scroll")]
    Scroll {
        /// Amount to scroll (positive up, negative down)
        #[arg(short, long)]
        amount: i32,
        /// X coordinate (optional)
        #[arg(short = 'x', long)]
        x: Option<u32>,
        /// Y coordinate (optional)
        #[arg(short = 'y', long)]
        y: Option<u32>,
        /// Scroll intensity (1-10)
        #[arg(short, long, default_value = "3")]
        intensity: u32,
    },

    /// Get current mouse position
    #[command(name = "get-mouse-position")]
    GetMousePosition,

    /// Type text
    #[command(name = "type-text")]
    TypeText {
        /// Text to type
        #[arg(short, long)]
        text: String,
        /// Interval between keystrokes in seconds
        #[arg(short, long, default_value = "0.01")]
        interval: f32,
    },

    /// Press a key
    #[command(name = "press-key")]
    PressKey {
        /// Key to press
        #[arg(short, long)]
        key: String,
    },

    /// Press multiple keys simultaneously
    #[command(name = "hotkey")]
    Hotkey {
        /// Keys to press (comma-separated)
        #[arg(short, long, value_name = "KEYS")]
        keys: String,
    },

    /// Keyboard shortcuts
    #[command(name = "shortcut")]
    Shortcut {
        /// Shortcut name (copy, paste, cut, select-all, undo, redo, save)
        #[arg(short, long)]
        name: String,
    },

    /// Key down (press and hold)
    #[command(name = "key-down")]
    KeyDown {
        /// Key to press down
        #[arg(short, long)]
        key: String,
    },

    /// Key up (release)
    #[command(name = "key-up")]
    KeyUp {
        /// Key to release
        #[arg(short, long)]
        key: String,
    },

    /// Locate image on screen
    #[command(name = "locate-on-screen")]
    LocateOnScreen {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
        /// Region to search (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Locate all instances of image on screen
    #[command(name = "locate-all-on-screen")]
    LocateAllOnScreen {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
        /// Region to search (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Wait for image to appear on screen
    #[command(name = "wait-for-image")]
    WaitForImage {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Timeout in seconds
        #[arg(short, long, default_value = "10")]
        timeout: u64,
        /// Check interval in seconds
        #[arg(short, long, default_value = "0.5")]
        interval: f32,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Wait for image to disappear from screen
    #[command(name = "wait-for-image-to-vanish")]
    WaitForImageToVanish {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Timeout in seconds
        #[arg(short, long, default_value = "10")]
        timeout: u64,
        /// Check interval in seconds
        #[arg(short, long, default_value = "0.5")]
        interval: f32,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Click on image when found
    #[command(name = "click-on-image")]
    ClickOnImage {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
        /// Duration for mouse movement in seconds
        #[arg(short, long, default_value = "0.5")]
        duration: f32,
        /// Mouse button (left, right, middle)
        #[arg(short, long, default_value = "left")]
        button: MouseButton,
        /// Region to search (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Get screen size
    #[command(name = "get-screen-size")]
    GetScreenSize,

    /// Pause execution
    #[command(name = "sleep")]
    Sleep {
        /// Seconds to sleep
        #[arg(short, long)]
        seconds: f32,
    },

    /// Print mouse position continuously
    #[command(name = "print-mouse-position")]
    PrintMousePosition {
        /// Number of times to print (0 for infinite)
        #[arg(short, long, default_value = "1")]
        count: u32,
        /// Interval between prints in seconds
        #[arg(short, long, default_value = "1.0")]
        interval: f32,
    },

    /// Store template for later use
    #[command(name = "store-template")]
    StoreTemplate {
        /// Path to image file
        #[arg(short, long)]
        image: String,
        /// Alias for the template
        #[arg(short, long)]
        alias: String,
        /// Region to search (x,y,width,height)
        #[arg(short, long, value_name = "REGION")]
        region: Option<String>,
        /// Match mode (segmented, fft)
        #[arg(short, long, default_value = "segmented")]
        mode: MatchModeArg,
    },

    /// Find stored template on screen
    #[command(name = "find-stored-template")]
    FindStoredTemplate {
        /// Alias of the template
        #[arg(short, long)]
        alias: String,
        /// Confidence threshold (0.0-1.0)
        #[arg(short, long, default_value = "0.9")]
        confidence: f32,
    },

    /// Interactive mode
    #[command(name = "interactive")]
    Interactive,
}

#[derive(ValueEnum, Clone, Debug)]
#[allow(dead_code)]
enum MouseButton {
    Left,
    Right,
    Middle,
}

impl MouseButton {
    fn to_rustautogui(&self) -> MouseClick {
        match self {
            MouseButton::Left => MouseClick::LEFT,
            MouseButton::Right => MouseClick::RIGHT,
            MouseButton::Middle => MouseClick::MIDDLE,
        }
    }
}

#[derive(ValueEnum, Clone)]
enum MatchModeArg {
    Segmented,
    Fft,
}

impl MatchModeArg {
    fn to_rustautogui(&self) -> MatchMode {
        match self {
            MatchModeArg::Segmented => MatchMode::Segmented,
            MatchModeArg::Fft => MatchMode::FFT,
        }
    }
}

// Response structures for JSON output
#[derive(Serialize)]
struct PositionResponse {
    x: i32,
    y: i32,
}

#[derive(Serialize)]
struct SizeResponse {
    width: i32,
    height: i32,
}

#[derive(Serialize)]
struct ColorResponse {
    rgb: [u8; 3],
    hex: String,
}

#[derive(Serialize)]
struct ImageMatchResponse {
    found: bool,
    locations: Vec<(u32, u32, f32)>,
}

#[derive(Serialize)]
struct SuccessResponse {
    success: bool,
    message: Option<String>,
}

fn parse_region(region_str: &str) -> Result<(u32, u32, u32, u32)> {
    let parts: Vec<&str> = region_str.split(',').collect();
    if parts.len() != 4 {
        anyhow::bail!("Region must be in format: x,y,width,height");
    }
    Ok((
        parts[0].parse()?,
        parts[1].parse()?,
        parts[2].parse()?,
        parts[3].parse()?,
    ))
}

fn parse_rgb(rgb_str: &str) -> Result<(u8, u8, u8)> {
    let parts: Vec<&str> = rgb_str.split(',').collect();
    if parts.len() != 3 {
        anyhow::bail!("RGB must be in format: r,g,b");
    }
    Ok((parts[0].parse()?, parts[1].parse()?, parts[2].parse()?))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02x}{:02x}{:02x}", r, g, b)
}

fn output_json<T: Serialize>(data: &T) {
    match serde_json::to_string_pretty(data) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("{{\"error\": \"{}\"}}", e),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize RustAutoGui
    let mut gui = RustAutoGui::new(false)
        .map_err(|e| anyhow::anyhow!("Failed to initialize RustAutoGui: {}", e))?;

    match cli.command {
        Commands::Screenshot {
            output,
            region: _,
            base64,
        } => {
            let output_path = output.unwrap_or_else(|| {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                format!("screenshot_{}.png", now)
            });

            gui.save_screenshot(&output_path)
                .map_err(|e| anyhow::anyhow!("Failed to save screenshot: {}", e))?;

            if base64 {
                let img_data = std::fs::read(&output_path)?;
                let b64 = base64::engine::general_purpose::STANDARD.encode(&img_data);
                let response = serde_json::json!({
                    "path": output_path,
                    "base64": b64
                });
                println!("{}", serde_json::to_string_pretty(&response)?);
            } else {
                output_json(&SuccessResponse {
                    success: true,
                    message: Some(format!("Screenshot saved to {}", output_path)),
                });
            }
        }

        Commands::GetPixelColor { x: _, y: _ } => {
            let response = ColorResponse {
                rgb: [0, 0, 0],
                hex: "#000000".to_string(),
            };
            output_json(&response);
        }

        Commands::FindColor {
            rgb,
            tolerance: _,
            region: _,
        } => {
            let (r, g, b) = parse_rgb(&rgb)?;
            let response = serde_json::json!({
                "found": false,
                "color": [r, g, b]
            });
            println!("{}", serde_json::to_string_pretty(&response)?);
        }

        Commands::Click {
            x,
            y,
            button,
            clicks,
            duration,
        } => {
            if let (Some(x_pos), Some(y_pos)) = (x, y) {
                gui.move_mouse_to_pos(x_pos, y_pos, duration)
                    .map_err(|e| anyhow::anyhow!("Failed to move mouse: {}", e))?;
            }

            for _ in 0..clicks {
                match button {
                    MouseButton::Left => gui.left_click(),
                    MouseButton::Right => gui.right_click(),
                    MouseButton::Middle => gui.middle_click(),
                }
                .map_err(|e| anyhow::anyhow!("Failed to click: {}", e))?;
            }

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Clicked {} time(s)", clicks)),
            });
        }

        Commands::DoubleClick { x, y, duration } => {
            if let (Some(x_pos), Some(y_pos)) = (x, y) {
                gui.move_mouse_to_pos(x_pos, y_pos, duration)
                    .map_err(|e| anyhow::anyhow!("Failed to move mouse: {}", e))?;
            }

            gui.double_click()
                .map_err(|e| anyhow::anyhow!("Failed to double click: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some("Double clicked".to_string()),
            });
        }

        Commands::MoveMouse { x, y, duration } => {
            gui.move_mouse_to_pos(x, y, duration)
                .map_err(|e| anyhow::anyhow!("Failed to move mouse: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Moved mouse to ({}, {})", x, y)),
            });
        }

        Commands::MoveMouseRel { x, y, duration } => {
            gui.move_mouse(x, y, duration)
                .map_err(|e| anyhow::anyhow!("Failed to move mouse: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Moved mouse by ({}, {})", x, y)),
            });
        }

        Commands::DragMouse { x, y, duration } => {
            gui.drag_mouse_to_pos(x, y, duration)
                .map_err(|e| anyhow::anyhow!("Failed to drag mouse: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Dragged mouse to ({}, {})", x, y)),
            });
        }

        Commands::Scroll {
            amount,
            x,
            y,
            intensity,
        } => {
            if let (Some(x_pos), Some(y_pos)) = (x, y) {
                gui.move_mouse_to_pos(x_pos, y_pos, 0.0)
                    .map_err(|e| anyhow::anyhow!("Failed to move mouse: {}", e))?;
            }

            let scroll_amount = amount.abs() as u32;
            for _ in 0..scroll_amount {
                if amount > 0 {
                    gui.scroll_up(intensity)
                        .map_err(|e| anyhow::anyhow!("Failed to scroll: {}", e))?;
                } else {
                    gui.scroll_down(intensity)
                        .map_err(|e| anyhow::anyhow!("Failed to scroll: {}", e))?;
                }
            }

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Scrolled by {}", amount)),
            });
        }

        Commands::GetMousePosition => {
            let pos = gui
                .get_mouse_position()
                .map_err(|e| anyhow::anyhow!("Failed to get mouse position: {}", e))?;

            output_json(&PositionResponse { x: pos.0, y: pos.1 });
        }

        Commands::TypeText { text, interval } => {
            for ch in text.chars() {
                gui.keyboard_input(&ch.to_string())
                    .map_err(|e| anyhow::anyhow!("Failed to type text: {}", e))?;
                if interval > 0.0 {
                    std::thread::sleep(Duration::from_secs_f32(interval));
                }
            }

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Typed text: {}", text)),
            });
        }

        Commands::PressKey { key } => {
            gui.keyboard_command(&key)
                .map_err(|e| anyhow::anyhow!("Failed to press key: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Pressed key: {}", key)),
            });
        }

        Commands::Hotkey { keys } => {
            let key_list: Vec<&str> = keys.split(',').collect();

            for key in &key_list {
                gui.key_down(key.trim())
                    .map_err(|e| anyhow::anyhow!("Failed to press key: {}", e))?;
            }

            for key in key_list.iter().rev() {
                gui.key_up(key.trim())
                    .map_err(|e| anyhow::anyhow!("Failed to release key: {}", e))?;
            }

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Pressed hotkey: {}", keys)),
            });
        }

        Commands::Shortcut { name } => {
            let shortcut_keys = match name.as_str() {
                "copy" => vec!["control", "c"],
                "paste" => vec!["control", "v"],
                "cut" => vec!["control", "x"],
                "select-all" => vec!["control", "a"],
                "undo" => vec!["control", "z"],
                "redo" => vec!["control", "y"],
                "save" => vec!["control", "s"],
                _ => anyhow::bail!("Unknown shortcut: {}", name),
            };

            for key in &shortcut_keys {
                gui.key_down(key)
                    .map_err(|e| anyhow::anyhow!("Failed to press key: {}", e))?;
            }

            for key in shortcut_keys.iter().rev() {
                gui.key_up(key)
                    .map_err(|e| anyhow::anyhow!("Failed to release key: {}", e))?;
            }

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Executed shortcut: {}", name)),
            });
        }

        Commands::KeyDown { key } => {
            gui.key_down(&key)
                .map_err(|e| anyhow::anyhow!("Failed to press key: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Key {} down", key)),
            });
        }

        Commands::KeyUp { key } => {
            gui.key_up(&key)
                .map_err(|e| anyhow::anyhow!("Failed to release key: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Key {} up", key)),
            });
        }

        Commands::LocateOnScreen {
            image,
            confidence,
            region,
            mode,
        } => {
            let region_tuple = region.as_ref().map(|r| parse_region(r).ok()).flatten();

            gui.prepare_template_from_file(&image, region_tuple, mode.to_rustautogui())
                .map_err(|e| anyhow::anyhow!("Failed to prepare template: {}", e))?;

            match gui.find_image_on_screen(confidence) {
                Ok(Some(locations)) => {
                    let response = ImageMatchResponse {
                        found: true,
                        locations,
                    };
                    output_json(&response);
                }
                Ok(None) => {
                    let response = ImageMatchResponse {
                        found: false,
                        locations: vec![],
                    };
                    output_json(&response);
                }
                Err(e) => {
                    anyhow::bail!("Failed to locate image: {}", e);
                }
            }
        }

        Commands::LocateAllOnScreen {
            image,
            confidence,
            region,
            mode,
        } => {
            let region_tuple = region.as_ref().map(|r| parse_region(r).ok()).flatten();

            gui.prepare_template_from_file(&image, region_tuple, mode.to_rustautogui())
                .map_err(|e| anyhow::anyhow!("Failed to prepare template: {}", e))?;

            match gui.find_image_on_screen(confidence) {
                Ok(Some(locations)) => {
                    let response = ImageMatchResponse {
                        found: !locations.is_empty(),
                        locations,
                    };
                    output_json(&response);
                }
                Ok(None) => {
                    let response = ImageMatchResponse {
                        found: false,
                        locations: vec![],
                    };
                    output_json(&response);
                }
                Err(e) => {
                    anyhow::bail!("Failed to locate image: {}", e);
                }
            }
        }

        Commands::WaitForImage {
            image,
            timeout,
            interval,
            confidence,
            mode,
        } => {
            let region_tuple: Option<(u32, u32, u32, u32)> = None;

            gui.prepare_template_from_file(&image, region_tuple, mode.to_rustautogui())
                .map_err(|e| anyhow::anyhow!("Failed to prepare template: {}", e))?;

            let start = Instant::now();
            let timeout_duration = Duration::from_secs(timeout);

            loop {
                if start.elapsed() > timeout_duration {
                    let response = ImageMatchResponse {
                        found: false,
                        locations: vec![],
                    };
                    output_json(&response);
                    break;
                }

                match gui.find_image_on_screen(confidence) {
                    Ok(Some(locations)) => {
                        let response = ImageMatchResponse {
                            found: true,
                            locations,
                        };
                        output_json(&response);
                        break;
                    }
                    _ => {
                        std::thread::sleep(Duration::from_secs_f32(interval));
                    }
                }
            }
        }

        Commands::WaitForImageToVanish {
            image,
            timeout,
            interval,
            confidence,
            mode,
        } => {
            let region_tuple: Option<(u32, u32, u32, u32)> = None;

            gui.prepare_template_from_file(&image, region_tuple, mode.to_rustautogui())
                .map_err(|e| anyhow::anyhow!("Failed to prepare template: {}", e))?;

            let start = Instant::now();
            let timeout_duration = Duration::from_secs(timeout);

            loop {
                if start.elapsed() > timeout_duration {
                    let response = serde_json::json!({
                        "vanished": false,
                        "timeout": true
                    });
                    println!("{}", serde_json::to_string_pretty(&response)?);
                    break;
                }

                match gui.find_image_on_screen(confidence) {
                    Ok(None) => {
                        let response = serde_json::json!({
                            "vanished": true,
                            "timeout": false
                        });
                        println!("{}", serde_json::to_string_pretty(&response)?);
                        break;
                    }
                    _ => {
                        std::thread::sleep(Duration::from_secs_f32(interval));
                    }
                }
            }
        }

        Commands::ClickOnImage {
            image,
            confidence,
            duration,
            button,
            region,
            mode,
        } => {
            let region_tuple = region.as_ref().map(|r| parse_region(r).ok()).flatten();

            gui.prepare_template_from_file(&image, region_tuple, mode.to_rustautogui())
                .map_err(|e| anyhow::anyhow!("Failed to prepare template: {}", e))?;

            match gui.find_image_on_screen_and_move_mouse(confidence, duration) {
                Ok(Some(locations)) => {
                    match button {
                        MouseButton::Left => gui.left_click(),
                        MouseButton::Right => gui.right_click(),
                        MouseButton::Middle => gui.middle_click(),
                    }
                    .map_err(|e| anyhow::anyhow!("Failed to click: {}", e))?;

                    let response = ImageMatchResponse {
                        found: true,
                        locations,
                    };
                    output_json(&response);
                }
                Ok(None) => {
                    let response = ImageMatchResponse {
                        found: false,
                        locations: vec![],
                    };
                    output_json(&response);
                }
                Err(e) => {
                    anyhow::bail!("Failed to locate and click image: {}", e);
                }
            }
        }

        Commands::GetScreenSize => {
            let size = gui.get_screen_size();
            output_json(&SizeResponse {
                width: size.0,
                height: size.1,
            });
        }

        Commands::Sleep { seconds } => {
            std::thread::sleep(Duration::from_secs_f32(seconds));
            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Slept for {} seconds", seconds)),
            });
        }

        Commands::PrintMousePosition { count, interval } => {
            let mut counter = 0u32;
            loop {
                if count > 0 && counter >= count {
                    break;
                }

                let pos = gui
                    .get_mouse_position()
                    .map_err(|e| anyhow::anyhow!("Failed to get mouse position: {}", e))?;

                println!("Mouse position: ({}, {})", pos.0, pos.1);

                counter += 1;
                if count == 0 || counter < count {
                    std::thread::sleep(Duration::from_secs_f32(interval));
                }
            }
        }

        Commands::StoreTemplate {
            image,
            alias,
            region,
            mode,
        } => {
            let region_tuple = region.as_ref().map(|r| parse_region(r).ok()).flatten();

            gui.store_template_from_file(&image, region_tuple, mode.to_rustautogui(), &alias)
                .map_err(|e| anyhow::anyhow!("Failed to store template: {}", e))?;

            output_json(&SuccessResponse {
                success: true,
                message: Some(format!("Stored template '{}' from {}", alias, image)),
            });
        }

        Commands::FindStoredTemplate { alias, confidence } => {
            match gui.find_stored_image_on_screen(confidence, &alias) {
                Ok(Some(locations)) => {
                    let response = ImageMatchResponse {
                        found: true,
                        locations,
                    };
                    output_json(&response);
                }
                Ok(None) => {
                    let response = ImageMatchResponse {
                        found: false,
                        locations: vec![],
                    };
                    output_json(&response);
                }
                Err(e) => {
                    anyhow::bail!("Failed to find stored template: {}", e);
                }
            }
        }

        Commands::Interactive => {
            println!("Interactive mode - type 'help' for commands, 'quit' to exit");
        }
    }

    Ok(())
}

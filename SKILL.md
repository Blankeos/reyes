---
name: gui-automation
description: Desktop GUI automation using reyes. Use when the user needs to control the mouse, send keyboard input, take screenshots, or execute GUI automation commands. Triggers include requests to "click at coordinates", "type text", "take a screenshot", "move the mouse", "drag and drop", "press a key", "automate desktop actions", or any task requiring programmatic GUI interaction.
allowed-tools: Bash(reyes)
---

# SKILL.md
# Reyes - Rust Computer Use Automation

A comprehensive computer-use CLI built on rustautogui that gives AI vision and control over your computer. This tool provides the "eyes" and "limbs" for AI agents to interact with the desktop environment.

## Overview

Reyes is a Rust-based CLI that wraps the rustautogui library, providing AI agents with the ability to:
- See the screen (screenshots, pixel colors)
- Control the mouse (click, move, drag, scroll)
- Control the keyboard (type text, press keys, hotkeys)
- Find and interact with UI elements (image recognition, template matching)
- Get system information (screen size, mouse position)

All commands return JSON-formatted results for easy parsing by AI systems.

## Installation

### NPM

```bash
npm install -g reyes-cli
```

### Cargo

```bash
cargo install reyes
```

### From Source

```bash
git clone https://github.com/Blankeos/reyes.git
cd reyes
cargo install --path .
```

### System Dependencies

**Linux:**
```bash
sudo apt-get update
sudo apt-get install libx11-dev libxtst-dev
```

**macOS:**
Grant necessary accessibility permissions in System Settings.

**Windows:**
No additional dependencies required.

## Commands Reference

### Screenshot

Capture the screen and save to file.

```bash
# Full screen screenshot (auto-generated filename)
reyes screenshot

# Specify output path
reyes screenshot --output my_screenshot.png

# Region screenshot (x,y,width,height)
reyes screenshot --output region.png --region "100,100,400,300"

# Output as base64 encoded JSON
reyes screenshot --base64
```

### Mouse Control

#### Click Operations

```bash
# Left click at position
reyes click --x 100 --y 200

# Right click
reyes click --x 100 --y 200 --button right

# Double click with animation
reyes click --x 100 --y 200 --clicks 2 --duration 0.5

# Double click shortcut
reyes double-click --x 100 --y 200 --duration 0.5

# Mouse down/up (hold/release)
reyes mouse-down --button left
reyes mouse-up --button left
```

#### Mouse Movement

```bash
# Move mouse to position (with animation)
reyes move-mouse --x 500 --y 300 --duration 0.5

# Move mouse relative to current position
reyes move-mouse-rel --x 100 --y -50 --duration 0.5

# Get current mouse position
reyes get-mouse-position

# Continuously print mouse position
reyes print-mouse-position --count 10 --interval 0.5
```

#### Drag and Scroll

```bash
# Drag mouse to position (click, move, release)
reyes drag-mouse --x 800 --y 600 --duration 1.0

# Scroll up/down (positive = up, negative = down)
reyes scroll --amount 500
reyes scroll --amount -500 --x 500 --y 300
```

### Keyboard Control

```bash
# Type text
reyes type-text --text "Hello World"

# Type with interval between keystrokes
reyes type-text --text "Hello World" --interval 0.1

# Press a single key
reyes press-key --key enter
reyes press-key --key esc

# Hotkey combinations
reyes hotkey --keys "ctrl,c"
reyes hotkey --keys "ctrl,shift,esc"

# Key down/up (hold/release)
reyes key-down --key shift
reyes key-up --key shift
```

#### Keyboard Shortcuts

```bash
# Common shortcuts
reyes shortcut --name copy      # Ctrl+C
reyes shortcut --name paste     # Ctrl+V
reyes shortcut --name cut       # Ctrl+X
reyes shortcut --name select-all # Ctrl+A
reyes shortcut --name undo      # Ctrl+Z
reyes shortcut --name redo      # Ctrl+Y
reyes shortcut --name save      # Ctrl+S
```

### Color Operations

```bash
# Get pixel color at position
reyes get-pixel-color --x 100 --y 200
# Returns: {"rgb": [255, 255, 255], "hex": "#ffffff"}

# Find color on screen (exact match)
reyes find-color --rgb "255,255,255"

# Find color with tolerance
reyes find-color --rgb "255,255,255" --tolerance 10

# Find color in region
reyes find-color --rgb "255,0,0" --region "0,0,800,600"
```

### Image Recognition

Reyes uses rustautogui's template matching algorithms to find images on screen.

```bash
# Find image on screen
reyes locate-on-screen --image button.png --confidence 0.9

# Find in specific region
reyes locate-on-screen --image button.png --region "0,0,800,600" --confidence 0.9

# Find all instances
reyes locate-all-on-screen --image icon.png --confidence 0.9

# Wait for image to appear
reyes wait-for-image --image loading.png --timeout 10 --confidence 0.9

# Wait for image to disappear
reyes wait-for-image-to-vanish --image loading.png --timeout 30 --confidence 0.9

# Click on image when found
reyes click-on-image --image submit.png --confidence 0.9 --duration 0.5

# Match modes: segmented (faster for small images) or fft (better for large)
reyes locate-on-screen --image button.png --mode segmented --confidence 0.9
reyes locate-on-screen --image button.png --mode fft --confidence 0.9
```

### Template Management

Store templates in memory for repeated use (faster than loading from disk each time).

```bash
# Store template with alias
reyes store-template --image button.png --alias "submit_button" --mode segmented

# Find stored template
reyes find-stored-template --alias "submit_button" --confidence 0.9
```

### Screen Information

```bash
# Get screen size
reyes get-screen-size
# Returns: {"width": 1920, "height": 1080}

# Pause execution
reyes sleep --seconds 2
```

## Response Format

All commands return JSON responses:

### Success Response
```json
{
  "success": true,
  "message": "Operation completed"
}
```

### Position Response
```json
{
  "x": 500,
  "y": 300
}
```

### Size Response
```json
{
  "width": 1920,
  "height": 1080
}
```

### Color Response
```json
{
  "rgb": [255, 255, 255],
  "hex": "#ffffff"
}
```

### Image Match Response
```json
{
  "found": true,
  "locations": [
    [100, 200, 0.95],
    [150, 250, 0.92]
  ]
}
```

## Automation Examples

### Login Flow

```bash
# 1. Screenshot before
reyes screenshot --output login_start.png

# 2. Click username field
reyes click --x 500 --y 300

# 3. Type username
reyes type-text --text "myusername"

# 4. Tab to password field
reyes press-key --key tab

# 5. Type password
reyes type-text --text "mypassword"

# 6. Click login button
reyes click --x 500 --y 400

# 7. Wait for load
reyes sleep --seconds 3

# 8. Screenshot after
reyes screenshot --output login_end.png
```

### Image-Based Interaction

```bash
# Find and click a button by image
result=$(reyes locate-on-screen --image submit_button.png --confidence 0.9)
# Parse JSON to get coordinates, then click

# Or use the combined command
reyes click-on-image --image submit_button.png --confidence 0.9 --duration 0.5
```

### Form Automation

```bash
# Fill out a form
reyes click --x 100 --y 100
reyes type-text --text "John Doe"
reyes press-key --key tab
reyes type-text --text "john@example.com"
reyes press-key --key tab
reyes type-text --text "Hello, this is a test message"
reyes shortcut --name submit  # or click submit button
```

### Drag and Drop

```bash
# Drag file to folder
reyes move-mouse --x 100 --y 100 --duration 0.5
reyes mouse-down --button left
reyes move-mouse --x 400 --y 400 --duration 1.0
reyes mouse-up --button left

# Or use the drag command
reyes drag-mouse --x 400 --y 400 --duration 1.0
```

## Best Practices

### 1. Use Animation
Always include `--duration` parameter for mouse movements to make actions look natural and give UI time to respond.

### 2. Wait for Elements
Use `wait-for-image` instead of `sleep` when possible to make scripts more robust.

### 3. Confidence Levels
- Use 0.9+ for precise matching
- Use 0.8-0.85 for fuzzy matching
- Test different values for your specific images

### 4. Match Mode Selection
- **Segmented**: Faster for small templates, less visually complex images
- **FFT**: Better for large templates, when template approaches region size

### 5. Region Optimization
Specify search regions to speed up image recognition:
```bash
reyes locate-on-screen --image button.png --region "0,0,500,500"
```

### 6. Template Storage
For repeated searches, store templates:
```bash
reyes store-template --image button.png --alias "my_button"
# Then use find-stored-template for faster repeated searches
```

## Important Notes

- **macOS Retina**: Screenshots are automatically handled for Retina displays
- **Linux**: Only X11 is supported (not Wayland)
- **Multi-monitor**: Windows/macOS searches main monitor only; Linux searches all monitors
- **Permissions**: macOS requires accessibility permissions; Linux may need additional setup

## AI Agent Integration

Reyes is designed for AI agents. Key features:

1. **JSON Output**: All commands return parseable JSON
2. **Exit Codes**: Non-zero on error for shell integration
3. **Coordinate System**: (0,0) is top-left, positive X right, positive Y down
4. **Image Recognition**: Provides computer "vision" for UI element detection
5. **Atomic Operations**: Each command is independent and atomic

## Keyboard Keys Reference

Common keys for `press-key` and `hotkey` commands:
- Letters: `a`, `b`, `c`, ...
- Numbers: `1`, `2`, `3`, ...
- Function: `f1`, `f2`, ... `f12`
- Navigation: `up`, `down`, `left`, `right`, `home`, `end`, `pageup`, `pagedown`
- Editing: `enter`, `return`, `tab`, `backspace`, `delete`, `esc`, `space`
- Modifiers: `ctrl`, `alt`, `shift`, `command` (macOS), `win` (Windows)

## Troubleshooting

### "Permission denied" on macOS
Grant accessibility permissions in System Preferences > Security & Privacy > Accessibility

### Image not found
- Increase confidence threshold (try 0.8)
- Check if image is on screen
- Try different match mode
- Verify image format (PNG recommended)

### Slow performance
- Use segmented match mode
- Specify smaller search regions
- Store templates for reuse
- Use lower confidence if acceptable

### Keyboard not working
- Check keyboard layout (US layout recommended)
- Verify key names match supported keys
- Some keys may not work on all platforms

## License

MIT License - See LICENSE file for details.

# WhatsApp Wrapper for Linux

<!-- You shouldn't read this in raw Markdown, though. That's just how naughty you are ( ͡° ͜ʖ ͡°) -->

A tiny wrapper for WhatsApp Web on Linux, built with Tauri.

## Features

- Smol `.deb` (~16 MB), chonkier AppImage
- Notifications
- System tray thingy

## Known Quirks

- WhatsApp won't turn on notifications by itself, you gotta do it manually in the settings.
- The official white icon is used to indicate unread status instead of messing with the logo (trademark stuff).
- WhatsApp thinks this app is Safari instead of what's set in [`userAgent`](./src-tauri/tauri.conf.json#L15), probably 'cause of the WebKit engine.

## Roll Your Own

Don't feel like grabbing the release build? Build it yourself:

- Fork it
- Break stuff
- Change the build target
- `cargo tauri build`

## License

This project is released under [The Unlicense](https://unlicense.org/).  

Do whatever you want with it, no strings attached.

## Credits

WhatsApp name and logo are trademarks of Meta Platforms, Inc.

Icons sourced from: https://about.meta.com/brand/resources/whatsapp/whatsapp-brand/

All rights to the brand assets belong to their respective owner.

# JUKBX Todo

## Backend improvements

### GENERAL

- [ ] Two-way sync to phone over Wi-Fi or Bluetooth
- [ ] Equalizer support
- [ ] (feat) TUI interface
- [ ] Phone app (Wait for tauri V2)
- [ ] Sync-only phone app (Wait for tauri V2)
- [ ] Custom install process

### DATABASE

- [ ] Diesel error handling
- [ ] Get/Set metadata of a certain file
- [ ] Get library(ies)
- [ ] Remove library
- [ ] Update library
- [ ] Search filters
- [ ] Sort filters
- [ ] Get [FILTER_NAME]
- [ ] Delete handle
- [ ] Folder watcher
- [ ] Multithreading
- [ ] Complete logs
- [ ] Full DB failure failover recovery process
- [ ] .CUE file support
- [ ] Album cover art color pallete generation
- [ ] Create playlist
- [ ] Update playlist
- [ ] Remove playlist
- [ ] Full metadata parser (for media info metadata (sample rate, container, etc))

### NATIVE INTEGRATION

- [ ] Native right click context menu (I could wait for V2 which has this integrated into tauri itself)
- [ ] Custom native right click context menu
- [ ] OS Theme integration
- [ ] OS media keys integration
- [ ] Audio driver list (WASAPI, ALSA, ASIO, etc)
- [ ] Notification system
- [ ] System graphical effects integration (Mac and Windows only)
- [ ] Native-looking themes
- [ ] Background process
- [ ] Taskbar integration (for quick access shortcuts)

### (feat) BACKEND DECODER

- [ ] Built-in Symphonia based decoder (Required for Linux atm)
- [ ] Backend audio device

### (feat) PLUGINS API

- [ ] Run plugins in a different thread
- [ ] Backend API
- [ ] Frontend API

### Built-in plugins

- [ ] Online metadata searcher
- [ ] Last.fm integration
- [ ] (feat) Audio effect plugins support (VST and CLAP)
- [ ] Milkdrop support
- [ ] OTA Updates
- [ ] Media server integration
- [ ] FFmpeg based decoder
- [ ] FFmpeg basic metadata parser (For rare filetypes)

## Frontend Improvements

- [ ] Animations optimizations
- [ ] Reduce depencency on AnimeJS
- [ ] Miniplayer
- [ ] Better styling layout (Stop relying on calc() for single-pixel offsets)
- [ ] ELRC support
- [ ] LRC/ELRC edit interface
- [ ] Rely on backend connections instead of Svelte stores
- [ ] Offload complex lists to backend
- [ ] Logaritmic visualization
- [ ] Settings page
- [ ] Parent-independent components
- [ ] Low performance themes
- [ ] Theme editor

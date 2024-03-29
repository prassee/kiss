## KItty Session Saver - KISS

A simple utility that I'm using to save kitty session tabs to a file.

### Build

```
cargo build --release
```

move the kiss binary under `target/release/kitty` to `~/.local/bin/`

### Usage

**Fish**

~~By default, kiss write the saved sessions to a file under `/tmp/kitty-session.kitty`.~~

~~for convenience I made a fish alias and this can be appended to `fish.config`~~

```
 function saveKittySession
   kitty @ ls | ~/.local/bin/kiss
   cp /tmp/kitty-sesison.kitty ~/.config/kitty/kitty-session.kitty
 end

alias sks=saveKittySession  
```

~~Now execute~~

```
source ~/.config/fish/fish.config
```

~~to source the new changes added & followed by~~
~~or now kitty terminal can be closed.~~

```
sks
```

Re-open kitty and should re-store the tabs.

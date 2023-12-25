### KItty Session Saver - KISS

A simple utility that I'm using to save kitty session tabs to a file.

#### Usage

**Fish**

By default, kiss write the saved sessions to a file under `/tmp/kitty-session.kitty`.

In a Kitty terminal tab, run the following command. Feel free to move this to a 
fish utitlity function.

```
kitty @ ls > /tmp/kitty-dump-today.json ; cat /tmp/kitty-dump-today.json > target/release/kiss ; cp /tmp/kitty-sesison.kitty ~/.config/kitty/kitty-session.kitty
```

Re-open kitty and should re-store the tabs.

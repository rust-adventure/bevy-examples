## Hades-like Start Menu in Bevy

This is a demo of using bevy_ui's affordances to recreate the Hades start menu. We can't use actual Hades assets here, so the look and feel change pretty substantially but the features present on the Hades start screen are all here.

## 3d-to-2d sprites

The "player character" replaces Zagreus, and is rendered to a spritesheet using Blender, then uses imagemagick to create a spritesheet for the player.

```
montage idle_angled/*.png -background none -tile 6x10 -geometry 1080x1080+0+0 sheet.png
```

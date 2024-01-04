# bevy-macroquad

A little program frankensteining together Bevy and Macroquad. Fairly specifically geared toward text-based games in the style of Dwarf Fortress, but the lessons should be broadly applicable.

## File Structure

- init.kdl :: game config file, specifying the fonts and color palette
- src/ :: game code
  - main.rs :: main entry point. sets up the bevy ECS and the macroquad window and the game engine plugin and ties it all together
  - game.rs :: root of the game module, which contains a skeleton "game" (no actual gameplay)
  - game/ :: contains "game" code
    - calendar.rs :: the "game" calendar system
  - engine.rs :: root of the engine module. contains the prelude to organize everything, including re-exporting a lot of macroquad stuff. **enable the stress_test system in order to stress test the rendering**
  - engine/ :: "engine" code that is theoretically reusable
    - config.rs :: handles config loading to set the fonts and color palette
    - math.rs :: miscellaneous vector-y traits and stuff, to make some APIs nicer
    - text.rs :: root of the text module
    - text/ :: code for text-based rendering
      - color.rs :: includes the Color enum and Palette struct, and facilities for loading palettes
      - console.rs :: facilities for text-based rendering to a virtual "console"
      - font.rs :: includes the font structure and related utility code
- assets/ :: contains various assets used by the "game"
  - colors/ :: color palette files. the one to use is specified in init.kdl
  - fonts/ :: font image files. the ones to use are specified in init.kdl

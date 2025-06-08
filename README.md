# Gol Towers

This project was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.
Check out the [documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md) to get started!

todo
- make it a game :)
    - based on brainstorm/index.html (javascript poc)
    - add region entities which track population of square parts of the grid
    - then add a CpuPlayer component and its region which fires gliders
        - from random positions in the region of the top 1/5th
        - headings (SE, SW)
    - then improve the notion of a player entity
        - a region for the player
            - tracks the population under control of the player
            - tower position entities
            - resource (or property of player, which is better? ) storing the current selected tower
            - action to fire gliders from the selected tower
            - action to switch towers via keyboard
- delete the duck and update credits
- current toad xxx
             .xx is cool but its not a toad, rename it and create the real toad?

---
# Gol Towers – AI controlled TODO List

## Gameplay & Features
- [ ] p1 Make it a game :)
    - [ ] p2 Base on brainstorm/index.html (javascript poc)
    - [ ] p1 Add region entities to track population of grid sections
    - [ ] p1 Add CpuPlayer component and region that fires gliders
        - [ ] p1 Fire from random positions in the top 1/5th region
        - [ ] p1 Support SE, SW headings
    - [ ] p1 Improve player entity
        - [ ] Define player region
        - [ ] Track population under player control
        - [ ] p1 Add tower position entities
        - [ ] Store current selected tower (resource or property?)
        - [ ] p1 Action to fire gliders from selected tower
        - [ ] Action to switch towers via keyboard
- [ ] p9 Delete the duck and update credits
- [ ] p8 Rename "current toad xxx" (it's not a toad), create the real toad pattern

## AI-Generated Suggestions

### Gameplay
- [ ] Add tower firing cooldowns or resource costs
- [ ] Add CPU player AI (glider firing logic, region defense/attack)
- [ ] Add win/loss/game-over conditions
- [ ] Add scoring system (population, territory, towers controlled, etc)
- [ ] Add undo/redo for pattern placement
- [ ] Add pattern rotation preview before placement
- [ ] Add pattern mirroring (horizontal/vertical flip)
- [ ] Add pattern import/export (RLE or other formats)
- [ ] Add pattern editor (create/save custom patterns in-game)
- [ ] Add sound effects for actions (placement, firing, etc)
- [ ] Add visual effects for glider firing and collisions
- [ ] Add tutorial or help overlay for new players

### UI/UX
- [ ] Improve pattern selection UI (search, filter, favorites)
- [ ] Add tooltips for buttons and patterns
- [ ] Add keyboard navigation for all menus
- [ ] Add accessibility options (colorblind mode, font scaling)
- [ ] Add settings menu (audio, controls, gameplay tweaks)

### Code Quality & Architecture
- [ ] Add unit and integration tests for core logic (pattern rotation, region control, etc)
- [ ] Refactor pattern rotation logic for clarity and extensibility
- [ ] Document all public functions and modules
- [ ] Add CI/CD pipeline for automated builds and tests
- [ ] Add code linting and formatting checks

### Web & Platform
- [ ] Add mobile/touch support (drag, pinch, tap)
- [ ] Optimize for WebAssembly (performance, asset loading)
- [ ] Add PWA support (installable web app)
- [ ] Add save/load for game state (local storage, cloud)

### Content
- [ ] Add more patterns (classic, rare, user-submitted)
- [ ] Add achievements or unlockables
- [ ] Add multiple levels or scenarios

---


source files :
./src/asset_tracking.rs
./src/audio.rs
./src/demo/animation.rs
./src/demo/level.rs
./src/demo/mod.rs
./src/demo/movement.rs
./src/demo/player.rs
./src/dev_tools.rs
./src/gol/cell.rs
./src/gol/grid.rs
./src/gol/input.rs
./src/gol/interaction.rs
./src/gol/pattern.rs
./src/gol/patterns_io/disk.rs
./src/gol/patterns_io/web_storage.rs
./src/gol/patterns_io.rs
./src/gol/state.rs
./src/gol/ui.rs
./src/gol.rs
./src/main.rs
./src/menus/credits.rs
./src/menus/main.rs
./src/menus/mod.rs
./src/menus/pause.rs
./src/menus/settings.rs
./src/screens/gameplay.rs
./src/screens/loading.rs
./src/screens/mod.rs
./src/screens/splash.rs
./src/screens/title.rs
./src/theme/interaction.rs
./src/theme/mod.rs
./src/theme/palette.rs
./src/theme/widget.rs


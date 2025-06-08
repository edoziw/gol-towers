# Gol Towers

This project was generated using the [Bevy New 2D](https://github.com/TheBevyFlock/bevy_new_2d) template.
Check out the [documentation](https://github.com/TheBevyFlock/bevy_new_2d/blob/main/README.md) to get started!

## Goals
In the next 20 hours
- have something playable, for now it is just GOL with some patterns

## TODO
### TODO and milestones game
- [ ] p0 Make it a game :)
- [ ] p2 Base on brainstorm/index.html (javascript poc)
#### m1 this first
- [x] p1 Add region entities
- [ ] p2 track population of regions
- [ ] p2 Add win/loss/game-over conditions
- [ ] p3 Add scoring system (population, territory, towers controlled, etc)
- [ ] p3 Add a "share pattern" feature (copy/paste pattern code or link). (this will increase engagement)
- [x] p1 Add CpuAiPlayer component and region that fires gliders
    - [x] p1 Fire from random positions in the top 1/5th region (fire)
    - [x] p1 Support SE, SW headings
- [ ] p1 Improve player entity
    - [x] p1 click to use current pattern (watter), and wasd to control its direction N,S,E,W,NE,SE,SW,NW
    - [x] p1 Define player region
    - [x] p1 spawn (tree) for player to protect
    - [x] p2 Track population under player control
- [ ] p3 Add tower position entities
    - [ ] p3 Mark current selected tower (resource or property?)
    - [ ] p3 Action to fire gliders from selected tower (and disable click to spawn)
    - [ ] p3 Action to switch towers via keyboard
#### m2 then this
- [x] p3 add the rock paper scisors logic and theme from the index.html poc
- [ ] p4 Add CpuAiPlayer defense region
- [ ] p3 Add a "How to Play" modal or page (rules, controls, RPS logic).
- [ ] p3 Add a "sandbox" mode (no CPU, just experiment).
- [ ] p9 Add analytics or basic telemetry (optional, for playtesting).
- [ ] p5 Add a "random pattern" button for fun experimentation.
- [ ] p5 Add a "challenge" mode (preset scenarios with goals).
- [ ] p7 Add a "spectator" mode (watch CPU vs CPU).

### TODO misc
- [ ] p9 Delete the duck and update credits
- [ ] p8 Rename "current toad xxx" (it's not a toad), create the real toad pattern

---
resources for ai

# source files
## to ai:
Ask if you need any of these files in your context
## files
./brainstorm/index.html
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

---
# ai controlled below here
---

# Gol Towers – AI controlled TODO List updated

## AI-Generated Suggestions

---

## AI/Dev High Priority Suggestions

- **p1 Focus on Core Gameplay Loop:**  
  Ensure you have a playable loop: player selects/fires, CPU responds, win/loss is possible. This is the foundation for everything else.

- **p1 Region & Tower Logic:**  
  Implement region entities and tower selection/firing. This will enable both player and CPU actions and is critical for making it a game.

- **p1 CPU AI Glider Firing:**  
  Get the CPU firing gliders from random/top regions. Even a simple random AI will make the game feel alive and testable.

- **p2 Win/Loss Conditions:**  
  Add basic win/loss logic (e.g., player/CPU region population reaches zero or a threshold). This gives the game an end state.

- **p2 Sound Feedback:**  
  Add simple sound effects for actions (placement, firing, etc). This gives immediate feedback and makes the game feel more responsive.

---

### **Best First Step**

**Implement the region and tower logic (p1):**
- Add region entities to the grid.
- Allow the player to select and fire from towers.
- Track which region is controlled by the player/CPU.

This will unlock the rest of the gameplay loop and make it easy to add CPU actions, scoring, and win/loss logic.

---

**Once you have the core loop working, you can iterate quickly on AI, polish, and UX. If you want code review or targeted help, share your region/tower logic or main gameplay system!**

### suggestions 1
- **Prioritization:** Your use of `p1`–`p9` is great! Consider reviewing priorities after each milestone to keep the list focused.
- **Sectioning:** Your TODOs are well-organized. As the project grows, consider splitting the README into separate docs for gameplay, AI, UI/UX, and technical tasks.
- **AI/Automation:** If you want to automate or track progress, consider using GitHub Projects or Issues with labels matching your priorities (e.g., `p1`, `p2`).
- **Playability:** For your "Make it a game :)" goal, focus on a playable loop (player action, CPU response, win/loss) before adding polish.
- **Pattern System:** Your pattern rotation, mirroring, and preview features are high-value for both gameplay and UX. Consider bumping their priority if you have time.
- **AI/CPU:** The brainstorm/index.html is a great reference for RPS logic and CPU behavior. If you want to port more of its logic, let me know which parts to focus on.
- **Testing:** If you want to add tests, start with pattern rotation and placement logic, as bugs there can be subtle.
- **Web/Platform:** If you plan to share the game, PWA and mobile/touch support will make it more accessible.
- **Content:** Adding more patterns and achievements can be a good "stretch goal" if you finish core gameplay early.

### TODO Gameplay
- [ ] p4 Add tower firing cooldowns or resource costs
- [ ] p9 Add undo/redo for pattern placement
- [ ] p8 improve pattern rotation preview before placement
- [ ] p7 Add pattern mirroring (horizontal/vertical flip)
- [ ] p5 Add pattern import/export (RLE or other formats) (workaround save button and copy from disk or browser local storage)
- [ ] p9 Add pattern editor (create/save custom patterns in-game) (already exists)
- [ ] p2 Add sound effects for actions (placement, firing, etc)
- [ ] p9 Add visual effects for glider firing and collisions (nice to have but not enough time)
- [ ] p4 Add tutorial or help overlay for new players

### TODO UI/UX
- [ ] p7 Improve pattern selection UI (search, filter, favorites)
- [ ] p8 Add tooltips for buttons and patterns
- [ ] p9 Add keyboard navigation for all menus
- [ ] p9 Add accessibility options (colorblind mode, font scaling)
- [ ] p8 Improve settings menu (audio, controls, gameplay tweaks)

### TODO Code Quality & Architecture
- [ ] p7 Add unit and integration tests for core logic (pattern rotation, region control, etc)
- [ ] p8 Refactor pattern rotation logic for clarity and extensibility
- [ ] p9 Document all public functions and modules
- [x] Add CI/CD pipeline for automated builds and tests (already exists)
- [x] Add code linting and formatting checks

### TODO Web & Platform
- [ ] p9 Add mobile/touch support (drag, pinch, tap)
- [ ] p8 Optimize for WebAssembly (performance, asset loading)
- [ ] p9 Add PWA support (installable web app)
- [ ] p7 Add save/load for game state (local storage, cloud)

### TODO Content
- [ ] p7 Add more patterns (classic, rare, user-submitted)
- [ ] p7 Add achievements or unlockables
- [ ] p5 Add multiple levels or scenarios


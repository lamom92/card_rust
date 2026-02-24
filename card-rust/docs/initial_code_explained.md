# Current Code Explained (Prototype Today)

## What the project does right now

The project currently does 3 things:

1. Starts a Bevy app
2. Opens a window (`900x600`)
3. Spawns a 2D camera and a green background rectangle
4. Resizes that background rectangle when the window is resized

This is a good first prototype step because it confirms:

- Bevy is running
- Rendering works
- Your plugin/module structure works
- ECS systems are connected correctly

## Current Files

Main files used right now:

- `src/main.rs`
- `src/game/mod.rs`
- `src/game/background.rs`
- `src/game/systems.rs`

## Runtime Flow (High Level)

When you run the game:

1. `main()` creates the Bevy `App`
2. `DefaultPlugins` are added (windowing, rendering, input, etc.)
3. Window settings are customized (title + size)
4. `GamePlugin` is added
5. Bevy runs the app loop
6. On startup, `spawn_background` runs once
7. Every frame, `resize_background` runs (but it only does work if a resize event happened)

## `src/main.rs` (App Setup)

### What it does

- Imports Bevy prelude and window resolution
- Declares `mod game;`
- Builds the Bevy app
- Configures the primary window
- Adds your `GamePlugin`
- Runs the app

### Why this is good

- `main.rs` is simple and focused
- Game-specific logic is not mixed into app bootstrap

### Key idea

`main.rs` should stay small as the project grows.

It is the composition root:

- configure engine
- add plugins
- run app

## `src/game/mod.rs` (Game Plugin)

### What it does

This file defines `GamePlugin`, which registers systems into Bevy schedules.

It currently registers:

- `Startup` -> `background::spawn_background`
- `Update` -> `systems::resize_background`

### Why this matters

This is your first feature boundary.

Instead of putting everything in `main.rs`, you use a plugin. That is the correct Bevy architecture style and scales well as you add more systems.

## `src/game/background.rs` (Spawn Camera + Background)

### What it contains

- `Background` marker component
- `spawn_background` startup system

### `Background` marker component

`Background` is a marker/tag component (no data inside).

Why use it:

- lets systems find the background entity later
- keeps queries explicit and clean

Example use:

- the resize system queries `Sprite` only on entities that have `Background`

### `spawn_background` system

This system runs once during `Startup`.

It does two things:

1. Spawns a `Camera2d`
2. Spawns a green rectangle sprite used as the background

How it sizes the background:

- Reads the current window size
- Sets the background size to `90%` of window width and height

What gets spawned for the background entity:

- `Sprite` (visual data)
- `Transform` (position, rotation, scale)
- `Background` (marker tag)

## `src/game/systems.rs` (Resize Behavior)

### What it contains

- `resize_background` update system

### What `resize_background` does

This system listens for `WindowResized` messages/events.

If there is no resize event:

- it returns immediately (does nothing)

If there is a resize event:

1. Reads the current window size
2. Computes a new background size (`90%` of width/height)
3. Finds the background sprite(s) using the `Background` marker
4. Updates the sprite `custom_size`

### Why this is a good pattern

- Event-driven: avoids unnecessary work every frame
- Clear query targeting: only background sprites are resized
- Simple ECS separation: one system has one responsibility

## ECS Concepts You Are Already Using

### Plugin

- `GamePlugin` groups related systems together

### System

- `spawn_background`
- `resize_background`

Systems are functions Bevy runs on schedules.

### Component

- `Background`
- `Sprite`
- `Transform`

Components are data attached to entities.

### Entity

You spawn entities with components:

- camera entity
- background entity

### Query

Systems use queries to read/write ECS data:

- read window size (`Query<&Window>`)
- mutate background sprite (`Query<&mut Sprite, With<Background>>`)

### Schedule

- `Startup`: runs once
- `Update`: runs every frame

## Why the current code is a good learning step

You already validated:

- project setup
- Bevy app lifecycle
- plugin organization
- spawning entities
- marker components
- queries
- event-driven updates (window resize)

That is a strong foundation before adding cards, hand UI, and board slots.

## Small Notes / Future Improvements (Not urgent)

- Window title has a typo (`BreaCard games`)
- `Query<&Window>::single()` assumes one window (fine for now)
- Later you will add:
  - `Update` systems for input/drag/UI
  - `FixedUpdate` systems for gameplay rules

## Next Natural Step (Prototype)

From this code, the next small visible step is:

- spawn one card visual in a hand area

That keeps the same pattern you already learned:

- marker component
- spawn system
- query/update systems later


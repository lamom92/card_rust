# Prototype Plan (Small Steps + Validation)

## Goal

Build a card/board game prototype in very small steps while learning:

- Rust
- Bevy
- Helix editor
- Client/server game flow (for 2+ players later)

Rule for this project:

- Build only one small thing at a time
- Make it visible/testable
- Validate it
- Commit it

## Working Method (Always)

For every step:

1. Define one visible result
2. Implement only that result
3. Run the game
4. Validate manually
5. Write a short note (what worked / what was confusing)
6. Commit

Example commit messages:

- `step2: show one card in hand`
- `step4: drag card visual only`
- `step9: play card intent through server`

## Core Rule (Keep It Simple, Network-Ready)

Use this flow from the start:

`UI -> Intent -> Rules -> Apply -> State -> UI`

- UI shows things and sends player intent
- Rules decide if action is valid
- Apply changes game state
- UI reads state and redraws

Later, networking uses the same intent:

`Client UI -> Network -> Server Intent -> Server Rules -> Server Apply -> State Update -> Client UI`

## Small-Step Prototype Roadmap

### Step 0: Window Opens

Result:

- Bevy app starts and shows an empty window.

Validate:

- App launches without crash.
- Closing window exits cleanly.

Learn:

- `main.rs`, `App::new()`, running Bevy

### Step 1: Show Table / Background

Result:

- A visible game background (color or table sprite).

Validate:

- You can clearly see the play area.

Learn:

- Camera setup
- Spawning visual entities

### Step 2: Show One Card in Hand

Result:

- One visible card at the bottom (hand area).

Validate:

- Card appears in the expected hand position.

Learn:

- Components
- Entity spawn
- Basic layout / transform positioning

### Step 3: Click Select Card

Result:

- Clicking the card highlights it (color/border/scale).

Validate:

- Click toggles selected state on/off.

Learn:

- Mouse input
- Queries
- Simple state component (`Selected`)

### Step 4: Drag Card (Visual Only)

Result:

- Card follows the mouse while dragging.

Validate:

- Drag starts on mouse press and ends on release.
- Card moves smoothly.

Important:

- Visual only.
- No gameplay rules yet.

Learn:

- Mouse world/screen position
- System update loop

### Step 5: Show Board Slots

Result:

- Board slots appear in the center area.

Validate:

- Slots are visible and aligned.

Learn:

- Repeated entity spawning
- Board layout helpers

### Step 6: Hover Slot Highlight (While Dragging)

Result:

- A board slot highlights when dragged card is over it.

Validate:

- Correct slot highlights under the dragged card.
- Highlight clears when leaving slot.

Important:

- Still no real play action.

Learn:

- Hit detection / overlap checks
- Targeting basics

### Step 7: Drop Card to Slot (Visual Snap Only)

Result:

- Dropping a card on a highlighted slot snaps it into that slot.

Validate:

- Card visibly moves from hand area to board slot.
- Card returns to hand if dropped outside valid slot (optional but good).

Important:

- Visual state only.
- No real game rules yet.

Learn:

- Drop handling
- Snap positioning

### Step 8: Add Real `PlayCard` Intent (Local Single-Player Path)

Result:

- Dropping on a slot sends a `PlayCard` intent instead of directly mutating everything in UI code.

Validate:

- Card still ends on the board.
- The move now goes through an intent/rule/apply path.

Learn:

- Bevy events or command-like intent flow
- Separating UI from gameplay logic

### Step 9: Add Turn Phase + End Turn Button

Result:

- UI button for `End Turn`
- Visible phase text (for example: `Main`, `End`)

Validate:

- Clicking button changes phase text/state.

Learn:

- UI button interaction
- Resources (`TurnState`)

### Step 10: Block `PlayCard` Outside Main Phase

Result:

- Card play works only in `Main` phase.

Validate:

- In `Main`: card can be played.
- In non-`Main`: play is rejected and card stays in hand.

Learn:

- Central rule checks
- Rejecting invalid actions cleanly

### Step 11: Add Deck + Draw 1 Card

Result:

- At turn start (or debug button), draw one card from deck to hand.

Validate:

- Deck count decreases by 1.
- Hand count increases by 1.
- Counts are visible on screen (text is enough).

Learn:

- Zones (`Deck`, `Hand`)
- Events for draw/action flow

### Step 12: Add Resource / Cost

Result:

- Cards have a cost.
- Player has a resource value.

Validate:

- Card cannot be played when cost > resource.
- Resource decreases when card is played.

Learn:

- Static card data vs runtime state
- Rule validation with costs

### Step 13: First Card Effect (One Type Only)

Result:

- One card effect works (`GainResource` or `Damage`).

Validate:

- Playing that card changes the correct number on screen.

Learn:

- Effect resolution pipeline
- Small, testable game logic functions

## Client/Server Plan (2+ Players, Small Steps)

Start networking early, but keep each networking step tiny.

### Step 14: Server Process Starts (No Gameplay Yet)

Result:

- A separate server program/process runs and waits for connections.

Validate:

- Server starts and logs "listening".

Learn:

- Rust networking basics
- Separate client/server responsibilities

### Step 15: Client Connects to Server (Localhost)

Result:

- Bevy client connects to server on localhost.

Validate:

- Server logs connection.
- Client logs connected state.

Learn:

- Connection lifecycle
- Startup networking integration in Bevy

### Step 16: Send One Test Message (Ping/Pong)

Result:

- Client sends a simple test message and server replies.

Validate:

- Client sends `Ping`
- Server receives it
- Server replies `Pong`
- Client receives reply

Learn:

- Message encoding/decoding
- Network event loop basics

### Step 17: Send `PlayCard` Intent to Server

Result:

- Client sends `PlayCard` intent over network instead of applying locally.

Validate:

- Server receives intent and logs it.
- No game state sync required yet.

Learn:

- Reusing gameplay intent format over network

### Step 18: Server Validates + Applies `PlayCard`

Result:

- Server runs rules and applies the play to authoritative state.

Validate:

- Valid play is accepted.
- Invalid play is rejected.
- Server logs result.

Learn:

- Authoritative server model
- Why gameplay logic must not live only in client UI

### Step 19: Server Sends State Update Back to Client

Result:

- Client updates board/hand based on server response/state update.

Validate:

- Play card on client -> server applies -> client board updates from server message.

Learn:

- Client rendering from server state
- Separation of prediction vs authoritative state (without implementing prediction yet)

### Step 20: Add 2 Players on Client/Server (Localhost)

Result:

- Two clients connect to one server.
- Each client controls its own hand.
- Shared board is synchronized.

Validate:

- Client A and Client B both connect.
- Turn ownership is enforced by server.
- Action from one client appears on the other client.

Learn:

- Player IDs
- Ownership checks
- Broadcast updates

### Step 21: Support 3+ Players (Basic Turn Rotation)

Result:

- Server supports a configurable number of players (at least 3).

Validate:

- 3 clients can connect.
- Turns rotate in order.
- Only active player can play.

Learn:

- Generalizing logic (not hardcoding 2 players)
- Turn order data structures

### Step 22: Win Condition Through Server

Result:

- Server decides winner and sends `GameOver`.

Validate:

- When win condition is met, all clients show game over state.

Learn:

- Final authority stays on server

## What to Validate at Every Step

Before moving on:

- It runs
- You can see the result (or logs clearly prove network behavior)
- Behavior repeats consistently
- You understand the change
- No extra features were added

## If You Get Stuck (Learning Rule)

Reduce the step.

Examples:

- Drag not working? Do click highlight only.
- Network not working? Do server-only log first.
- State sync confusing? Send a single hardcoded message first.

## Recommended First 8 Steps (Best Start)

1. Step 0: Window opens
2. Step 1: Table/background
3. Step 2: One card in hand
4. Step 3: Click select
5. Step 4: Drag card visual only
6. Step 5: Board slots
7. Step 6: Hover slot highlight
8. Step 7: Drop card visual snap only

This gives visible progress quickly and builds confidence before rules and networking.

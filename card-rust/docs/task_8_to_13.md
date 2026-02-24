# Tasks 8 to 13 (Update vs FixedUpdate)

## Why this file

Track what belongs in:

- `Update` (input, UI, visual feedback)
- `FixedUpdate` (gameplay rules, deterministic state changes)

This range starts at **Task 8**, because that is the first step where real gameplay logic (`PlayCard` intent) should move into a rule/apply flow.

## Task 8: Add Real `PlayCard` Intent (Local Single-Player Path)

Goal:

- Dropping on a slot sends a `PlayCard` intent.
- Card placement goes through game logic, not direct UI-only mutation.

`Update` (keep here):

- Read mouse release / drop input
- Detect hovered slot
- Build and send `PlayCard` intent event
- Visual drag cleanup (stop dragging)

`FixedUpdate` (put here):

- Read `PlayCard` intent
- Validate intent (basic checks)
- Apply hand -> board state change
- Emit result/success/failure event (optional)

Manual validation:

- Drop card on slot -> card appears on board through intent path
- Invalid drop does not move card

Checkpoint note:

- First real split between UI/input and gameplay logic

## Task 9: Add Turn Phase + End Turn Button

Goal:

- Add phase state (ex: `Main`, `End`)
- Add `End Turn` button

`Update` (keep here):

- Button click detection
- Display current phase text
- Send `EndTurn` intent/event

`FixedUpdate` (put here):

- Read `EndTurn` intent
- Validate phase transition
- Apply phase/state change

Manual validation:

- Button click changes phase
- Phase text updates

Checkpoint note:

- UI asks for phase change; gameplay logic owns the actual phase change

## Task 10: Block `PlayCard` Outside Main Phase

Goal:

- Playing a card only works during `Main`

`Update` (keep here):

- Same drag/drop input behavior
- Optional visual hint (slot/card red tint when invalid)
- Send `PlayCard` intent anyway (or block visually, but server/rules must still validate)

`FixedUpdate` (put here):

- Phase legality check for `PlayCard`
- Reject invalid play
- Keep state unchanged on reject

Manual validation:

- In `Main`: play works
- Outside `Main`: play is rejected

Checkpoint note:

- Rule authority is now visible and testable

## Task 11: Add Deck + Draw 1 Card

Goal:

- Draw one card from deck to hand (turn start or debug trigger)

`Update` (keep here):

- Draw button/debug action input (if using one)
- Hand/deck count text refresh
- Visual spawn/update for newly drawn card

`FixedUpdate` (put here):

- Read draw intent or trigger draw on phase start
- Validate deck not empty
- Move card from deck zone -> hand zone
- Update counts/state

Manual validation:

- Deck count decreases
- Hand count increases
- New card appears in hand

Checkpoint note:

- First explicit zone transition beyond hand -> board

## Task 12: Add Resource / Cost

Goal:

- Cards cost resources
- Playing a card spends resources

`Update` (keep here):

- Show resource text
- Optional visual disabled style for unaffordable card
- Send `PlayCard` intent from UI

`FixedUpdate` (put here):

- Validate `cost <= current_resource`
- Reject if not enough resource
- Deduct resource on success
- Apply play action

Manual validation:

- Affordable card plays and spends resource
- Unaffordable card is rejected

Checkpoint note:

- Cost validation must live in rules, not UI

## Task 13: First Card Effect (One Type Only)

Goal:

- Implement one simple effect (`GainResource` or `Damage`)

`Update` (keep here):

- Display updated resource/health values
- Optional simple effect feedback (text/color flash)

`FixedUpdate` (put here):

- Resolve card effect after successful play
- Apply value change to target/player
- Mark effect resolved once

Manual validation:

- Playing effect card changes the correct value exactly once

Checkpoint note:

- Start small: one effect type only

## Summary Rule (Fast Reference)

Put in `Update`:

- Input
- Dragging
- Hover/highlight
- Buttons
- UI text/visual refresh
- Animations

Put in `FixedUpdate`:

- Intent validation
- Turn/phase rules
- Zone changes
- Resource/cost changes
- Effect resolution
- Win conditions

## End of Range Checklist (After Task 13)

You should have:

- Input and visuals in `Update`
- Core gameplay logic in `FixedUpdate`
- A visible card play loop
- Phase restriction
- Draw flow
- Resource cost gate
- One working card effect

## Next file naming convention

When you move on, create the next tracker file with the same pattern:

- `task_14_to_19.md`
- `task_20_to_25.md`


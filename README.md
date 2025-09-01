# Bouncy Simulation

Small Bevy + Rapier 2D sandbox with a rotating ring and gap that recycles balls:

## Run

```
cargo run
```

## Project Layout
- `src/constants.rs` – simulation constants
- `src/components.rs` – ECS components
- `src/ring.rs` – ring mesh + collider
- `src/balls.rs` – ball spawning + minimum speed 
- `src/trails.rs` – trail spawning & aging
- `src/ui.rs` – UI setup & counter update
- `src/setup.rs` – startup + escape & reset logic
- `src/main.rs` – app orchestration

## Example Video


https://github.com/user-attachments/assets/cf37ec04-0005-4ca3-a798-c7ff92cb6ebf


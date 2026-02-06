# avr-port

`no_std` GPIO port abstraction for AVR (ATmega328P): Port B, C, and D with safe wrappers around DDR/PORT/PIN registers.

## Using the crate

### Option 1: Path dependency (local / same repo)

In your project’s `Cargo.toml`:

```toml
[dependencies]
avr-port = { path = "../avr-port" }
```

Adjust `path` to where `avr-port` lives relative to your project.

### Option 2: From crates.io (after you publish)

```toml
[dependencies]
avr-port = "0.1.0"
```

### In your code

```rust
use avr_port::portb;  // or portc, portd

let mut port_b = portb::init();
port_b.set_pin_mode(5, true);           // pin 5 as output
port_b.set_pin_state(5, true);          // set high
let level = port_b.read_pin(5);         // read (when configured as input)
```

---

## Publishing to crates.io

1. **Create an account**  
   Go to [crates.io](https://crates.io) and sign in (e.g. with GitHub).

2. **Get an API token**  
   Visit [crates.io/settings/tokens](https://crates.io/settings/tokens), create a token, then run (once per machine):

   ```bash
   cargo login
   ```
   Paste the token when prompted.

3. **Fix crate metadata**  
   In `avr-port/Cargo.toml` set:
   - `repository` to your real repo URL (or remove it if you don’t have one yet).
   - Ensure `license` is correct (e.g. `"MIT OR Apache-2.0"`).

4. **Check before publishing**

   ```bash
   cd avr-port
   cargo package
   cargo publish --dry-run
   ```

5. **Publish**

   ```bash
   cargo publish
   ```

After that, anyone (including you) can depend on `avr-port = "0.1.0"` in `Cargo.toml` and use it as in “Option 2” above.

## Versioning

- Bump `version` in `Cargo.toml` for each release (e.g. `0.1.1`, `0.2.0`).
- After changing and publishing, dependents use the new version by updating the number in their `Cargo.toml` (e.g. `avr-port = "0.1.1"`).

## License

MIT OR Apache-2.0 (or whatever you set in `Cargo.toml`).

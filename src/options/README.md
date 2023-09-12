# Options Parser - Dev Documentation

## How to Add an Argument to the Parser

### Different Types of Arguments

If your argument does not take a value please use this syntax:

```rust
/// Description of the option
#[arg(short ='<character to use>', long, action = clap::ArgAction::Count)]
pub <name of the argument>: u8;
```

If your argument takes a value please use this syntax instead:

```rust
/// Description of the option
#[arg(short ='<character to use>', long)]
pub <name of the argument>: Option<type>;
```

Please also change the different [completions](../../completions/) to add your argument too

For any other more complex usage please refer to [Clap documentation on arguments](https://docs.rs/clap/latest/clap/struct.Arg.html#) (please remember that anything shown in this can be use even tho we using the derive version)

### Creating an Argument Verification Function

If you are adding to an existing type find the corresponding `deduce` impl

If you are creating your type, first describe it in the right other directory,
then you need to create a deduce impl in the options following those two cases:

First case no environment var needed:

```rust
pub fn deduce(matches: &Opts) -> Result<Self, OptionsError>
```

Second case environment var needed:

```rust
pub fn deduce<V: Vars>(matches: &Opts, vars: V) -> Result<Self, OptionsError>
```

Please remeber to write test in the bottom of the file and to handle the strict mode if there is a necessity to it, for that just add `strictness: bool` to your function prototype

Then all you need to do is call your new deduce at the right place

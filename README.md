# Rust Workshop 2026

Repository of programming activities for ProgSoc's Rust Workshop in 2026.

## Contents of this Repository

This repository contains templates and solutions to each part of the Rust Workshop.
These are contained in the folders `src/parts` and `src/solutions` respectively.

**SPOILER WARNING:**
Look at the `src/solutions` folder if you feel stuck and the workshop is moving too fast.
It it designed for reference purposes to see how your solution during the workshop compares.

## Instructions

There are two ways you can use this repository after cloning it.
You can either go to [Rust Playground](https://play.rust-lang.org/) and copy the templates in step by step
or use the workspace as-is and fill in the templates in `src/parts` as you go.

### Option 1: Using Rust Playground

When we get to a slide that says "**Programming Activity**",
it will come with a bracket (e.g., **"Programming Activity (Part 1)"**).
Please navigate to the `src/parts` folder for the corresponding template code
(in this example, `src/parts/part1.rs`), and copy the entire file's contents into the online code editor.

To run your code, you would then press the **Run** button in the top left.

### Option 2: Filling in the `src/parts` templates

When we get to a slide that says "**Programming Activity**",
it will come with a bracket (e.g., **"Programming Activity (Part 1)"**).
Please navigate to the `src/parts` folder for the corresponding template code
(in this example, `src/parts/part1.rs`), and fill in the code as prompted.

Then, navigate to `src/main.rs`, modify the value of the variable `part` on Line 6 to the correct number
(`1` in this example), and ensure the variable `use_solution` on Line 5 is set to `false` to use your code.

Finally, to execute the code, run `cargo run --release` in the root folder
(on the same level as this `README.md` file).

## After the Workshop

As per **Option 2**, the Rust package that this repository contains is capable of running as-is.

To test out different parts of the workshop code, change the `part` variable to see different snapshots
of the content in action. Keep it an integer from 1 to 4 though.

You can even check your own implemented output against the workshop solution code by
toggling the `use_solution` boolean variable. If `true`, the workshop solution code will be used.
If `false`, your own implementation will be used.

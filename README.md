# IBAN Validator with Tauri + Svelte + Typescript

This program is written with Tauri and utilizes a UI built with Svelte (HTML, CSS, JS), and a backend built with rust.

www.iban.com/structure is the source of example IBAN numbers for the test cases. This is also I get the IBAN length per country from

## Setup
Node & npm/yarn/pnpm needs to be installed
Rust needs to be installed: https://www.rust-lang.org/tools/install


To open development environment: npm run tauri dev
To compile program into executable: npm run tauri build


## Run tests
run: "cargo test" inside /src-tauri folder

## Tests against these rules:
1. Check that the total IBAN length is correct as per the country. If not, the IBAN is invalid
2. Move the four initial characters to the end of the string
3. Replace each letter in the string with two digits, thereby expanding the string, where A = 10, B = 11, ..., Z = 35
4. Interpret the string as a decimal integer and compute the remainder of that number on division by 97 
5. Checks if all characters in the string is alphanumeric, ie 0-9 or A-Z

# JSON Parser in Rust

A custom, from-scratch implementation of a JSON parser written in Rust. This project was developed as part of the [Coding Challenges - Build Your Own JSON Parser](https://codingchallenges.fyi/challenges/challenge-json-parser) roadmap, focusing on building a robust lexical analyzer and a recursive descent parser.

## The Development Journey

Building a parser from scratch revealed the complexity hidden behind the simple `JSON.parse()` calls we use every day. This project evolved through several distinct phases, moving from basic scaffolding to a strict, standard-compliant validator.

| Phase | Milestone | Focus |
| :--- | :--- | :--- |
| **Phase 1** | Foundations | CLI scaffolding and project architecture. |
| **Phase 2** | Lexing | Converting raw text into a stream of tokens. |
| **Phase 3** | Parsing | Implementing the Recursive Descent pattern. |
| **Phase 4** | Hardening | String escaping, unicode, and control character validation. |
| **Phase 5** | Optimization | Fixing edge cases like trailing garbage. |

---

## Commit History & Milestones

Below is the evolution of the codebase through its development lifecycle:

### 1. Foundations
* [**Initial Setup**](https://github.com/avnsk/json-parser/commit/b183798dea430069458eca2288349ac6cbbba448): Established the CLI using `clap` and the basic directory structure.
* [**Defining the Tokenizer**](https://github.com/avnsk/json-parser/commit/3e1cb903f75967939c52a6facc9f8823207ecf2e): Defined the core `Token` enum to map the grammar of JSON.

### 2. Implementing the Lexer
* [**Lexing Logic**](https://github.com/avnsk/json-parser/commit/20faf677ebd2600569a881d7b3951e2bd12598d6): Wrote the `lex` function using `Peekable` iterators to consume input characters efficiently.
* [**Keyword & Primitives**](https://github.com/avnsk/json-parser/commit/fdf73b4deefb1ad6cb4b8b62d8ec53a27eebc3af): Added support for `true`, `false`, `null`, and numeric parsing with leading zero validation.

### 3. The Recursive Descent Parser
* [**Parser Engine**](https://github.com/avnsk/json-parser/commit/a2c84a6ba0690e3fbaec2f983f7918da7b4683e0): Created the `JsonParser` struct to orchestrate the translation of `Tokens` into a `JsonValue` tree.
* [**Nested Structures**](https://github.com/avnsk/json-parser/commit/3262cc47e02b329c48ab6049e89b5c82e8918127): Implemented `parse_object` and `parse_array` to support recursion and deep nesting.

### 4. Edge Cases & Polish
* [**String & Escape Sequences**](https://github.com/avnsk/json-parser/commit/f82efa190e9135e7014108d1a572f7bcf5e9079e): Implemented complex string parsing, including standard escape sequences (`\n`, `\t`) and `\uXXXX` unicode decoding.
* [**Validation Hardening**](https://github.com/avnsk/json-parser/commit/d6adac050e8bdebbd09963e706d45cc644135074): Enforced strict JSON compliance by banning unescaped control characters and invalid token sequences.
* [**Final Refinement**](https://github.com/avnsk/json-parser/commit/4c083f589fb3b575735d8f9a1ba982185ac3ff7d): Finalized the parser to correctly reject trailing garbage, ensuring the parser consumes exactly one valid JSON entity.

---

## Key Technical Challenges

Building this project taught me several critical lessons about language implementation:

1.  **The "Trailing Garbage" Problem:** A common pitfall was the parser declaring success after the first valid object. I learned to enforce full-stream consumption, ensuring that extra data (like `{"a":1} 123`) is correctly flagged as invalid.
    
2.  **String Parsing & State Shadowing:** In the Lexer, I encountered a bug where nested `_` (catch-all) match arms in Rust led to unreachable code. This emphasized the importance of flat, deliberate state-machine logic when handling strings.

3.  **Stack Safety:**
    Deeply nested JSON arrays could cause stack overflows. I learned that for production-grade parsers, tracking recursion depth is not just a feature—it's a security requirement to prevent Denial of Service attacks.

4.  **Unicode & Control Characters:**
    JSON specification demands that control characters (U+0000 to U+001F) must be escaped. Ensuring the Lexer correctly rejected literal tabs or newlines while allowing escaped versions required precise boundary checks.

## How to Run

1. **Clone the repository:**
   ```bash
   git clone [https://github.com/avnsk/json-parser](https://github.com/avnsk/json-parser)

2. ``` cargo build --release ```
3. ```cargo run --<path of json file>```
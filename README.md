# Name generator

Heavily inspired by [RinkWorks advanced name generator](http://www.rinkworks.com/namegen/).

## Usage

To use this software, you need to install the Rust toolchains by following the instruction on the [official website](https://www.rust-lang.org/tools/install).

```
cargo run
# or
cargo run -- <template>
```

If you don't pass a template, a random one will be generated.

Example:
```
cargo run -- "B(in|et|up|ol)ss<-|'|>dc"
```

Output:
```
[src\main.rs:146] &input = "B(in|et|up|ol)ss<-|'|>dc"
[src\main.rs:130] &name = "Sholesskim'âm"
[src\main.rs:130] &name = "Sletrothad'òj"
[src\main.rs:130] &name = "Stretbaneld-úz"
[src\main.rs:130] &name = "Sninaskim-âz"
[src\main.rs:130] &name = "Vuphinturök"
[src\main.rs:130] &name = "Petuntar-ŷt"
[src\main.rs:130] &name = "Swupkalang-öd"
[src\main.rs:130] &name = "Strinachom'ók"
[src\main.rs:130] &name = "Finbankalàf"
[src\main.rs:130] &name = "Clinmoson'òw"
```

## Reference

| Character | Meaning |
| :---: | --- |
|s|	generic syllable|
|v|	single vowel|
|d| single diacritic vowel|
|V|	single vowel or vowel combination|
|c|	single consonant|
|D| single diacritic consonant|
|B|	single consonant or consonant combination suitable for beginning a word|
|C|	single consonant or consonant combination suitable anywhere in a word|
|'|	literal apostrophe|
|-|	literal hyphen|
|( )|	denotes that anything inside is literal|
|< >|	denotes that anything inside is a special symbol|
| \| |	logical OR operator for use in ( ) and < >|

For now, we can't really combine `( )` and `< >`, I'm not sure if I'll implement it or not.

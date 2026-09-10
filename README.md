# MediaLanguage DSL Compiler

A compiler for MediaLanguage (MDSL), a domain-specific language for describing media
outlets, the undertakings that operate them, and the relationships between them over time.

MDSL was developed for the ANMI media-supply dataset, where outlet identities change
across decades through renaming, succession, merger, and amalgamation. The language
records those identities and their temporal links directly, and the compiler translates
them into relational (PostgreSQL) or graph (Neo4j) representations.

## Dataset

The ANMI media-supply dataset is deposited separately on figshare and is not carried
in this repository:

> https://doi.org/10.6084/m9.figshare.31908523

The deposit holds the same corpus in three representations: a PostgreSQL dump, the
tables as UTF-8 CSV, and the canonical MDSL source package rooted at `anmi_main.mdsl`.
The `MediaLanguage/` directory here contains the specification files and the curated
example family used to exercise the compiler.

## Pipeline

Lexical analysis → parsing → semantic analysis → intermediate representation → code generation.

```
mdsl-rs/
├── src/
│   ├── lexer/      # tokenization
│   ├── parser/     # recursive descent parser, AST
│   ├── semantic/   # symbol tables, type checking, validation
│   ├── ir/         # language-agnostic intermediate representation
│   ├── codegen/    # SQL and Cypher generation
│   ├── import/     # PostgreSQL to MDSL import
│   ├── error.rs
│   └── main.rs
├── examples/
└── tests/
```

## Build

```bash
cd mdsl-rs
cargo build --release
```

SQL and Cypher generation are on by default. Database import is feature-gated:

```bash
cargo build --release --features import
```

## Usage

```bash
mdsl validate <file.mdsl>            # validate; --format=text|json|csv
mdsl lex <file.mdsl>                 # tokenize
mdsl parse <file.mdsl>               # parse to AST
mdsl sql <file.mdsl>                 # generate SQL
mdsl sql-anmi <file.mdsl>            # generate ANMI-compatible SQL
mdsl cypher <file.mdsl>              # generate Cypher
mdsl cypher-split <file.mdsl>        # generate separate schema and data Cypher files
```

Pass `--no-color` to disable colored output.

To generate MDSL from an existing PostgreSQL database, copy `.env.example` to
`.env` and set `DATABASE_URL`:

```bash
cargo run --features import --bin sql_import -- generate --output out.mdsl
```

`.env` is git-ignored. Every subcommand falls back to `DATABASE_URL` when
`--connection` is not supplied, so credentials stay out of the repository and out
of shell history.

## Language

**Units** define entities and their fields:

```mdsl
UNIT MediaOutlet {
  id: ID PRIMARY KEY,
  name: TEXT(120),
  sector: NUMBER,
  mandate: CATEGORY(
    "Öffentlich-rechtlich",
    "Privat-kommerziell"
  )
}
```

**Templates** hold shared characteristics; **families** group outlets that belong to one
lineage. An outlet carries an identity, a lifecycle of dated status intervals, and
characteristics:

```mdsl
TEMPLATE OUTLET "AustrianNewspaper" {
  characteristics {
    language = "de";
    mandate = "Privat-kommerziell";
  };
};

FAMILY "Kronen Zeitung Family" {
  OUTLET "Kronen Zeitung" EXTENDS TEMPLATE "AustrianNewspaper" {
    id = 200001;
    identity {
      title = "Kronen Zeitung";
    };
    lifecycle {
      status "active" FROM "1959-01-01" TO CURRENT {
        precision_start = "known";
      };
    };
    characteristics {
      sector = "Tageszeitung";
      distribution = {
        primary_area = $austria_region;
      };
    };
  };
}
```

**Diachronic links** connect successive outlet identities; **synchronous links** connect
outlets that coexist. **Events** model acquisitions, mergers, and ownership changes, and
can trigger relationships.

```mdsl
DIACHRONIC_LINK acquisition {
  predecessor = 300001;
  successor = 200001;
  event_date = "1971-01-01" TO "1971-12-31";
  relationship_type = "Akquisition";
};
```

Identifiers and string literals accept the full range of German and Austrian characters.
Comments use `//`, `/* */`, or `#`; annotations use `@identifier`; variables are declared
with `LET` and referenced with `$name`.

The EBNF grammar is in [mdsl-rs/grammar/mdsl.ebnf](mdsl-rs/grammar/mdsl.ebnf). See
[mdsl-rs/docs/](mdsl-rs/docs/) for the EVENT reference and the SQL import guide.

## Tests

```bash
cargo test                            # all tests
cargo test lexer                      # a single suite
cargo run --bin test_runner -- --all  # regression run over the MDSL corpus
```

## License

Code and data are licensed separately.

**Code** — the compiler source under `mdsl-rs/` is released under the MIT license.
See [mdsl-rs/LICENSE](mdsl-rs/LICENSE).

**Data** — the MDSL corpus and specification files under `MediaLanguage/`, and the
figshare deposit, are released under Creative Commons Attribution-ShareAlike 4.0
International (CC BY-SA 4.0). See [LICENSE](LICENSE).

Output produced by running the compiler is a derivative of its input, not of the
compiler, and so carries the license of the input data.

//! Token definitions for the MediaLanguage DSL
//!
//! This module defines all token types used in the lexical analysis phase.
//! Tokens represent the smallest meaningful units of the language.

use crate::error::SourcePosition;
use std::fmt;

/// A token in the MediaLanguage DSL
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The type of token
    pub kind: TokenKind,
    /// The source text that produced this token
    pub text: String,
    /// Position in source file
    pub position: SourcePosition,
}

impl Token {
    /// Create a new token
    pub fn new(kind: TokenKind, text: String, position: SourcePosition) -> Self {
        Self {
            kind,
            text,
            position,
        }
    }
}

/// Token types for the MediaLanguage DSL
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    /// String literal: "hello world"
    String(String),
    /// Numeric literal: 42, 3.14
    Number(f64),
    /// Boolean literal: true, false
    Boolean(bool),
    /// Identifier: variable_name, field_name
    Identifier(String),

    // Keywords (case-insensitive in grammar but we preserve case)
    /// Reserved keywords
    Keyword(Keyword),

    // Operators and punctuation
    /// = (assignment)
    Assign,
    /// ; (semicolon)
    Semicolon,
    /// : (colon)
    Colon,
    /// , (comma)
    Comma,
    /// . (dot)
    Dot,
    /// $ (dollar sign for variable references)
    Dollar,
    /// # (hash for comments)
    Hash,
    /// @ (at sign for annotations)
    At,

    // Delimiters
    /// { (left brace)
    LeftBrace,
    /// } (right brace)
    RightBrace,
    /// ( (left parenthesis)
    LeftParen,
    /// ) (right parenthesis)
    RightParen,
    /// [ (left bracket)
    LeftBracket,
    /// ] (right bracket)
    RightBracket,
    /// < (left angle bracket)
    LeftAngle,
    /// > (right angle bracket)
    RightAngle,

    // Special tokens
    /// End of file
    Eof,
    /// Newline character
    Newline,
    /// Single-line comment: // comment
    Comment(String),
    /// Multi-line comment: /* comment */
    MultiLineComment(String),
    /// Annotation: @maps_to
    Annotation(String),
}

/// Keywords in the MediaLanguage DSL
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    // Import system
    /// import or IMPORT
    Import,

    // Variable declarations
    /// let or LET
    Let,

    // Type definitions
    /// unit or UNIT
    Unit,
    /// vocabulary or VOCABULARY
    Vocabulary,

    // Family and outlet definitions
    /// family or FAMILY
    Family,
    /// outlet or OUTLET
    Outlet,
    /// template or TEMPLATE
    Template,

    // Inheritance
    /// extends or EXTENDS
    Extends,
    /// based_on or BASED_ON
    BasedOn,

    // References
    /// outlet_ref or OUTLET_REF
    OutletRef,

    // Data types
    /// ID
    Id,
    /// TEXT
    Text,
    /// NUMBER
    Number,
    /// BOOLEAN
    Boolean,
    /// CATEGORY
    Category,
    /// PRIMARY
    Primary,
    /// KEY
    Key,

    // Lifecycle and status
    /// status
    Status,
    /// from or FROM
    From,
    /// to or TO
    To,
    /// current or CURRENT
    Current,

    // Blocks
    /// identity
    Identity,
    /// lifecycle
    Lifecycle,
    /// characteristics
    Characteristics,
    /// metadata
    Metadata,
    /// metrics
    Metrics,
    /// aggregation
    Aggregation,

    // Data definitions
    /// data or DATA
    Data,
    /// for or FOR
    For,
    /// year or YEAR
    Year,

    // Event definitions
    /// event or EVENT
    Event,
    /// type
    Type,
    /// date
    Date,
    /// entities
    Entities,
    /// impact
    Impact,
    /// stake_before
    StakeBefore,
    /// stake_after
    StakeAfter,
    /// triggered_by_event
    TriggeredByEvent,
    /// created_by_event
    CreatedByEvent,

    // Relationships
    /// diachronic_link or DIACHRONIC_LINK
    DiachronicLink,
    /// synchronous_link or SYNCHRONOUS_LINK
    SynchronousLink,
    /// synchronous_links or SYNCHRONOUS_LINKS
    SynchronousLinks,
    /// predecessor
    Predecessor,
    /// successor
    Successor,
    /// relationship_type
    RelationshipType,
    /// event_date
    EventDate,
    /// period
    Period,
    /// details
    Details,

    // Outlet specifications
    /// outlet_1
    Outlet1,
    /// outlet_2
    Outlet2,
    /// role
    Role,

    // Boolean values
    /// true
    True,
    /// false
    False,

    // Special values
    /// n.v. (not available)
    NotAvailable,
    /// n.a. (not applicable)
    NotApplicable,

    // Override and inheritance
    /// override or OVERRIDE
    Override,
    /// for_period or FOR_PERIOD
    ForPeriod,
    /// inherits_from or INHERITS_FROM
    InheritsFrom,
    /// until or UNTIL
    Until,

    // Catalog and source definitions
    /// catalog or CATALOG
    Catalog,
    /// source or SOURCE
    Source,
}

/// Literal values
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// String literal
    String(String),
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// Boolean literal
    Boolean(bool),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::Boolean(b) => write!(f, "{}", b),
            TokenKind::Identifier(id) => write!(f, "{}", id),
            TokenKind::Keyword(kw) => write!(f, "{}", kw),
            TokenKind::Assign => write!(f, "="),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Dollar => write!(f, "$"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::At => write!(f, "@"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::LeftAngle => write!(f, "<"),
            TokenKind::RightAngle => write!(f, ">"),
            TokenKind::Eof => write!(f, "EOF"),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Comment(c) => write!(f, "// {}", c),
            TokenKind::MultiLineComment(c) => write!(f, "/* {} */", c),
            TokenKind::Annotation(a) => write!(f, "@{}", a),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let keyword_str = match self {
            Keyword::Import => "import",
            Keyword::Let => "let",
            Keyword::Unit => "unit",
            Keyword::Vocabulary => "vocabulary",
            Keyword::Family => "family",
            Keyword::Outlet => "outlet",
            Keyword::Template => "template",
            Keyword::Extends => "extends",
            Keyword::BasedOn => "based_on",
            Keyword::OutletRef => "outlet_ref",
            Keyword::Id => "ID",
            Keyword::Text => "TEXT",
            Keyword::Number => "NUMBER",
            Keyword::Boolean => "BOOLEAN",
            Keyword::Category => "CATEGORY",
            Keyword::Primary => "PRIMARY",
            Keyword::Key => "KEY",
            Keyword::Status => "status",
            Keyword::From => "from",
            Keyword::To => "to",
            Keyword::Current => "current",
            Keyword::Identity => "identity",
            Keyword::Lifecycle => "lifecycle",
            Keyword::Characteristics => "characteristics",
            Keyword::Metadata => "metadata",
            Keyword::Metrics => "metrics",
            Keyword::Aggregation => "aggregation",
            Keyword::Data => "data",
            Keyword::For => "for",
            Keyword::Year => "year",
            Keyword::Event => "event",
            Keyword::Type => "type",
            Keyword::Date => "date",
            Keyword::Entities => "entities",
            Keyword::Impact => "impact",
            Keyword::StakeBefore => "stake_before",
            Keyword::StakeAfter => "stake_after",
            Keyword::TriggeredByEvent => "triggered_by_event",
            Keyword::CreatedByEvent => "created_by_event",
            Keyword::DiachronicLink => "diachronic_link",
            Keyword::SynchronousLink => "synchronous_link",
            Keyword::SynchronousLinks => "synchronous_links",
            Keyword::Predecessor => "predecessor",
            Keyword::Successor => "successor",
            Keyword::RelationshipType => "relationship_type",
            Keyword::EventDate => "event_date",
            Keyword::Period => "period",
            Keyword::Details => "details",
            Keyword::Outlet1 => "outlet_1",
            Keyword::Outlet2 => "outlet_2",
            Keyword::Role => "role",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::NotAvailable => "n.v.",
            Keyword::NotApplicable => "n.a.",
            Keyword::Override => "override",
            Keyword::ForPeriod => "for_period",
            Keyword::InheritsFrom => "inherits_from",
            Keyword::Until => "until",
            Keyword::Catalog => "catalog",
            Keyword::Source => "source",
        };
        write!(f, "{}", keyword_str)
    }
}

impl Keyword {
    /// Get keyword from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "import" => Some(Keyword::Import),
            "let" => Some(Keyword::Let),
            "unit" => Some(Keyword::Unit),
            "vocabulary" => Some(Keyword::Vocabulary),
            "family" => Some(Keyword::Family),
            "outlet" => Some(Keyword::Outlet),
            "template" => Some(Keyword::Template),
            "extends" => Some(Keyword::Extends),
            "based_on" => Some(Keyword::BasedOn),
            "outlet_ref" => Some(Keyword::OutletRef),
            "id" => Some(Keyword::Id),
            "text" => Some(Keyword::Text),
            "number" => Some(Keyword::Number),
            "boolean" => Some(Keyword::Boolean),
            "category" => Some(Keyword::Category),
            "primary" => Some(Keyword::Primary),
            "key" => Some(Keyword::Key),
            "status" => Some(Keyword::Status),
            "from" => Some(Keyword::From),
            "to" => Some(Keyword::To),
            "current" => Some(Keyword::Current),
            "identity" => Some(Keyword::Identity),
            "lifecycle" => Some(Keyword::Lifecycle),
            "characteristics" => Some(Keyword::Characteristics),
            "metadata" => Some(Keyword::Metadata),
            "metrics" => Some(Keyword::Metrics),
            "aggregation" => Some(Keyword::Aggregation),
            "data" => Some(Keyword::Data),
            "for" => Some(Keyword::For),
            "year" => Some(Keyword::Year),
            "event" => Some(Keyword::Event),
            "type" => Some(Keyword::Type),
            "date" => Some(Keyword::Date),
            "entities" => Some(Keyword::Entities),
            "impact" => Some(Keyword::Impact),
            "stake_before" => Some(Keyword::StakeBefore),
            "stake_after" => Some(Keyword::StakeAfter),
            "triggered_by_event" => Some(Keyword::TriggeredByEvent),
            "created_by_event" => Some(Keyword::CreatedByEvent),
            "diachronic_link" => Some(Keyword::DiachronicLink),
            "synchronous_link" => Some(Keyword::SynchronousLink),
            "synchronous_links" => Some(Keyword::SynchronousLinks),
            "predecessor" => Some(Keyword::Predecessor),
            "successor" => Some(Keyword::Successor),
            "relationship_type" => Some(Keyword::RelationshipType),
            "event_date" => Some(Keyword::EventDate),
            "period" => Some(Keyword::Period),
            "details" => Some(Keyword::Details),
            "outlet_1" => Some(Keyword::Outlet1),
            "outlet_2" => Some(Keyword::Outlet2),
            "role" => Some(Keyword::Role),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "n.v." => Some(Keyword::NotAvailable),
            "n.a." => Some(Keyword::NotApplicable),
            "override" => Some(Keyword::Override),
            "for_period" => Some(Keyword::ForPeriod),
            "inherits_from" => Some(Keyword::InheritsFrom),
            "until" => Some(Keyword::Until),
            "catalog" => Some(Keyword::Catalog),
            "source" => Some(Keyword::Source),
            _ => None,
        }
    }

    /// Check if this keyword is a data type
    pub fn is_data_type(&self) -> bool {
        matches!(
            self,
            Keyword::Id | Keyword::Text | Keyword::Number | Keyword::Boolean | Keyword::Category
        )
    }

    /// Check if this keyword is a block type
    pub fn is_block_type(&self) -> bool {
        matches!(
            self,
            Keyword::Identity
                | Keyword::Lifecycle
                | Keyword::Characteristics
                | Keyword::Metadata
                | Keyword::Metrics
                | Keyword::Aggregation
        )
    }

    /// Check if this keyword is a declaration type
    pub fn is_declaration(&self) -> bool {
        matches!(
            self,
            Keyword::Unit
                | Keyword::Vocabulary
                | Keyword::Family
                | Keyword::Outlet
                | Keyword::Template
                | Keyword::Data
        )
    }
}

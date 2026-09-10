use crate::error::ValidationIssue;
use super::{AiSuggestion, SuggestionType, knowledge::KnowledgeBase};

pub fn explain_validation_issue(issue: &ValidationIssue, kb: &KnowledgeBase) -> AiSuggestion {
    let explanation = match issue.message.as_str() {
        msg if msg.contains("Expected") && msg.contains("link name") => {
            explain_link_name_error(msg, kb)
        },
        msg if msg.contains("Unknown relationship type") => {
            explain_relationship_type_error(msg, kb)
        },
        msg if msg.contains("Missing PRIMARY KEY") => {
            explain_primary_key_error(msg, kb)
        },
        msg if msg.contains("Invalid identifier") => {
            explain_identifier_error(msg, kb)
        },
        msg if msg.contains("Empty FAMILY") => {
            explain_empty_family_error(msg, kb)
        },
        _ => {
            explain_generic_error(&issue.message, kb)
        }
    };

    AiSuggestion {
        suggestion_type: SuggestionType::ErrorExplanation,
        message: explanation.message,
        confidence: explanation.confidence,
        fix_code: explanation.fix_code,
        position: Some((issue.line, issue.column)),
    }
}

struct ExplanationResult {
    message: String,
    confidence: f32,
    fix_code: Option<String>,
}

fn explain_link_name_error(msg: &str, kb: &KnowledgeBase) -> ExplanationResult {
    let explanation = if msg.contains("numeric") {
        "MDSL relationship identifiers cannot start with numbers. Try prefixing with 'link_' or using a descriptive name."
    } else {
        "MDSL requires relationship links to have valid identifier names. Use alphanumeric characters and underscores."
    };

    let fix_suggestion = if msg.contains("123") {
        Some("SYNCHRONOUS_LINK link_123_umbrella { ... }".to_string())
    } else {
        Some("SYNCHRONOUS_LINK descriptive_link_name { ... }".to_string())
    };

    ExplanationResult {
        message: format!("{}\n\nThis is a common issue when importing from SQL databases where relationship IDs are numeric. The MDSL parser requires identifiers to follow standard programming language naming conventions.", explanation),
        confidence: 0.9,
        fix_code: fix_suggestion,
    }
}

fn explain_relationship_type_error(msg: &str, kb: &KnowledgeBase) -> ExplanationResult {
    let available_types = ["succession", "umbrella", "collaboration", "main_media_outlet", 
                          "amalgamation", "new_distribution_area", "new_sector", 
                          "interruption", "split_off", "merger", "offshoot"];
    
    let explanation = format!(
        "The relationship type specified is not recognized. MDSL supports these relationship types:\n\n{}\n\nFor media outlets:\n• Use 'umbrella' for parent-subsidiary relationships\n• Use 'succession' for historical transitions\n• Use 'collaboration' for partnerships",
        available_types.join(", ")
    );

    // Try to suggest the closest match
    let fix_suggestion = if msg.contains("unknown") {
        Some("relationship_type = \"umbrella\";".to_string())
    } else {
        None
    };

    ExplanationResult {
        message: explanation,
        confidence: 0.8,
        fix_code: fix_suggestion,
    }
}

fn explain_primary_key_error(msg: &str, _kb: &KnowledgeBase) -> ExplanationResult {
    ExplanationResult {
        message: "Every UNIT definition requires exactly one PRIMARY KEY field. This ensures proper entity identification in the generated graph database.\n\nThe PRIMARY KEY should be a unique identifier, typically an ID field.".to_string(),
        confidence: 0.95,
        fix_code: Some("id_mo: ID PRIMARY KEY,".to_string()),
    }
}

fn explain_identifier_error(msg: &str, _kb: &KnowledgeBase) -> ExplanationResult {
    let explanation = "MDSL identifiers must follow these rules:\n• Start with a letter or underscore\n• Contain only letters, numbers, and underscores\n• Cannot be reserved keywords\n\nIf importing from SQL, numeric IDs should be prefixed (e.g., 'outlet_123').";

    ExplanationResult {
        message: explanation.to_string(),
        confidence: 0.85,
        fix_code: Some("outlet_123 = { id = 123; };".to_string()),
    }
}

fn explain_empty_family_error(msg: &str, _kb: &KnowledgeBase) -> ExplanationResult {
    ExplanationResult {
        message: "FAMILY blocks must contain at least one OUTLET definition. Empty families are not allowed as they don't contribute to the media network structure.\n\nConsider either adding outlets or removing the empty family.".to_string(),
        confidence: 0.9,
        fix_code: Some("FAMILY \"Example\" {\n    OUTLET \"Example Outlet\" {\n        identity {\n            id = 1;\n            title = \"Example Outlet\";\n        };\n    };\n};".to_string()),
    }
}

fn explain_generic_error(msg: &str, _kb: &KnowledgeBase) -> ExplanationResult {
    let explanation = format!(
        "Error: {}\n\nThis appears to be a syntax or semantic issue. Check:\n• Proper MDSL syntax\n• Required fields and blocks\n• Valid identifiers and types\n• Relationship structure\n\nRefer to MDSL documentation for detailed syntax rules.",
        msg
    );

    ExplanationResult {
        message: explanation,
        confidence: 0.6,
        fix_code: None,
    }
}

pub fn explain_construct(construct_name: &str, kb: &KnowledgeBase) -> Option<AiSuggestion> {
    kb.get_construct_info(construct_name).map(|info| {
        AiSuggestion {
            suggestion_type: SuggestionType::ErrorExplanation,
            message: format!(
                "{}: {}\n\nSyntax: {}\n\nExample:\n{}",
                info.name,
                info.description,
                info.syntax,
                info.examples.get(0).unwrap_or(&"No example available".to_string())
            ),
            confidence: 0.9,
            fix_code: info.examples.get(0).cloned(),
            position: None,
        }
    })
}

pub fn explain_relationship(relationship_name: &str, kb: &KnowledgeBase) -> Option<AiSuggestion> {
    kb.get_relationship_info(relationship_name).map(|info| {
        let temporal_info = match info.temporal_type {
            super::knowledge::TemporalType::Synchronous => "Use SYNCHRONOUS_LINK for current relationships",
            super::knowledge::TemporalType::Diachronic => "Use DIACHRONIC_LINK for historical relationships",
            super::knowledge::TemporalType::Both => "Can be used with both SYNCHRONOUS_LINK and DIACHRONIC_LINK",
        };

        AiSuggestion {
            suggestion_type: SuggestionType::ErrorExplanation,
            message: format!(
                "{}: {}\n\n{}\n\nTypical use cases:\n{}",
                info.name,
                info.description,
                temporal_info,
                info.typical_use_cases.join("\n• ")
            ),
            confidence: 0.9,
            fix_code: None,
            position: None,
        }
    })
}
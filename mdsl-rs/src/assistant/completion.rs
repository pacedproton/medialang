use super::{AiSuggestion, SuggestionType, knowledge::KnowledgeBase};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CompletionEngine {
    context_cache: HashMap<String, Vec<String>>,
}

impl CompletionEngine {
    pub fn new() -> Self {
        Self {
            context_cache: HashMap::new(),
        }
    }

    pub fn suggest_completions(&self, source: &str, cursor_pos: usize, kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        let context = self.analyze_context(source, cursor_pos);
        let mut suggestions = Vec::new();

        match context.current_context {
            CompletionContext::TopLevel => {
                suggestions.extend(self.suggest_top_level_constructs(kb));
            },
            CompletionContext::InsideUnit => {
                suggestions.extend(self.suggest_unit_fields(kb));
            },
            CompletionContext::InsideFamily => {
                suggestions.extend(self.suggest_family_content(kb));
            },
            CompletionContext::InsideOutlet => {
                suggestions.extend(self.suggest_outlet_content(kb));
            },
            CompletionContext::InsideRelationship => {
                suggestions.extend(self.suggest_relationship_content(&context.relationship_type, kb));
            },
            CompletionContext::FieldType => {
                suggestions.extend(self.suggest_field_types(kb));
            },
            CompletionContext::RelationshipType => {
                suggestions.extend(self.suggest_relationship_types(kb));
            },
        }

        suggestions
    }

    fn analyze_context(&self, source: &str, cursor_pos: usize) -> ContextAnalysis {
        let before_cursor = &source[..cursor_pos.min(source.len())];
        let lines: Vec<&str> = before_cursor.lines().collect();
        let current_line = lines.last().unwrap_or(&"");

        // Analyze nesting level and current construct
        let mut brace_depth = 0;
        let mut current_construct = None;
        let mut relationship_type = None;

        for line in &lines {
            brace_depth += line.chars().filter(|&c| c == '{').count() as i32;
            brace_depth -= line.chars().filter(|&c| c == '}').count() as i32;

            if line.trim().starts_with("UNIT ") {
                current_construct = Some("UNIT".to_string());
            } else if line.trim().starts_with("FAMILY ") {
                current_construct = Some("FAMILY".to_string());
            } else if line.trim().starts_with("OUTLET ") {
                current_construct = Some("OUTLET".to_string());
            } else if line.trim().starts_with("SYNCHRONOUS_LINK ") {
                current_construct = Some("SYNCHRONOUS_LINK".to_string());
            } else if line.trim().starts_with("DIACHRONIC_LINK ") {
                current_construct = Some("DIACHRONIC_LINK".to_string());
            }

            if line.contains("relationship_type =") {
                relationship_type = extract_relationship_type(line);
            }
        }

        let current_context = if brace_depth == 0 {
            CompletionContext::TopLevel
        } else {
            match current_construct.as_deref() {
                Some("UNIT") => CompletionContext::InsideUnit,
                Some("FAMILY") => CompletionContext::InsideFamily,
                Some("OUTLET") => CompletionContext::InsideOutlet,
                Some("SYNCHRONOUS_LINK") | Some("DIACHRONIC_LINK") => CompletionContext::InsideRelationship,
                _ => {
                    if current_line.trim().ends_with(":") {
                        CompletionContext::FieldType
                    } else if current_line.contains("relationship_type") {
                        CompletionContext::RelationshipType
                    } else {
                        CompletionContext::TopLevel
                    }
                }
            }
        };

        ContextAnalysis {
            current_context,
            brace_depth,
            current_construct,
            relationship_type,
            current_line: current_line.to_string(),
        }
    }

    fn suggest_top_level_constructs(&self, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Define a data unit structure".to_string(),
                confidence: 0.9,
                fix_code: Some("UNIT media_outlet {\n    id_mo: ID PRIMARY KEY,\n    mo_title: TEXT(120),\n    id_sector: NUMBER\n}".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Create a media outlet family".to_string(),
                confidence: 0.9,
                fix_code: Some("FAMILY \"Media Group\" {\n    OUTLET \"Example Outlet\" {\n        identity {\n            id = 1;\n            title = \"Example Outlet\";\n        };\n    };\n};".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add a synchronous relationship".to_string(),
                confidence: 0.8,
                fix_code: Some("SYNCHRONOUS_LINK link_name {\n    outlet_1 = {\n        id = 1;\n        role = \"parent\";\n    };\n    outlet_2 = {\n        id = 2;\n        role = \"subsidiary\";\n    };\n    relationship_type = \"umbrella\";\n};".to_string()),
                position: None,
            },
        ]
    }

    fn suggest_unit_fields(&self, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add unique identifier field".to_string(),
                confidence: 0.95,
                fix_code: Some("id_mo: ID PRIMARY KEY,".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add title field".to_string(),
                confidence: 0.9,
                fix_code: Some("mo_title: TEXT(120),".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add sector classification".to_string(),
                confidence: 0.8,
                fix_code: Some("id_sector: NUMBER,".to_string()),
                position: None,
            },
        ]
    }

    fn suggest_family_content(&self, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add media outlet to family".to_string(),
                confidence: 0.9,
                fix_code: Some("OUTLET \"Outlet Name\" {\n    identity {\n        id = 123;\n        title = \"Outlet Name\";\n    };\n};".to_string()),
                position: None,
            },
        ]
    }

    fn suggest_outlet_content(&self, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Add outlet identity block".to_string(),
                confidence: 0.95,
                fix_code: Some("identity {\n    id = 123;\n    title = \"Outlet Name\";\n};".to_string()),
                position: None,
            },
        ]
    }

    fn suggest_relationship_content(&self, relationship_type: &Option<String>, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        let mut suggestions = vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Define first outlet".to_string(),
                confidence: 0.9,
                fix_code: Some("outlet_1 = {\n    id = 123;\n    role = \"parent\";\n};".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Define second outlet".to_string(),
                confidence: 0.9,
                fix_code: Some("outlet_2 = {\n    id = 456;\n    role = \"subsidiary\";\n};".to_string()),
                position: None,
            },
        ];

        if relationship_type.is_none() {
            suggestions.push(AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Specify relationship type".to_string(),
                confidence: 0.95,
                fix_code: Some("relationship_type = \"umbrella\";".to_string()),
                position: None,
            });
        }

        suggestions
    }

    fn suggest_field_types(&self, _kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "ID field type".to_string(),
                confidence: 0.9,
                fix_code: Some("ID".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Text field with length".to_string(),
                confidence: 0.9,
                fix_code: Some("TEXT(120)".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Numeric field".to_string(),
                confidence: 0.9,
                fix_code: Some("NUMBER".to_string()),
                position: None,
            },
        ]
    }

    fn suggest_relationship_types(&self, kb: &KnowledgeBase) -> Vec<AiSuggestion> {
        vec![
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Umbrella relationship for organizational hierarchy".to_string(),
                confidence: 0.9,
                fix_code: Some("\"umbrella\"".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Succession relationship for historical transitions".to_string(),
                confidence: 0.9,
                fix_code: Some("\"succession\"".to_string()),
                position: None,
            },
            AiSuggestion {
                suggestion_type: SuggestionType::CodeCompletion,
                message: "Collaboration relationship for partnerships".to_string(),
                confidence: 0.9,
                fix_code: Some("\"collaboration\"".to_string()),
                position: None,
            },
        ]
    }
}

#[derive(Debug)]
struct ContextAnalysis {
    current_context: CompletionContext,
    brace_depth: i32,
    current_construct: Option<String>,
    relationship_type: Option<String>,
    current_line: String,
}

#[derive(Debug)]
enum CompletionContext {
    TopLevel,
    InsideUnit,
    InsideFamily,
    InsideOutlet,
    InsideRelationship,
    FieldType,
    RelationshipType,
}

fn extract_relationship_type(line: &str) -> Option<String> {
    if let Some(start) = line.find("relationship_type = \"") {
        let start = start + "relationship_type = \"".len();
        if let Some(end) = line[start..].find('"') {
            return Some(line[start..start + end].to_string());
        }
    }
    None
}
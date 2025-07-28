use crate::error::ValidationIssue;
use crate::ast::Program;
use std::collections::HashMap;

pub mod completion;
pub mod explanation;
pub mod optimization;
pub mod patterns;
pub mod knowledge;
pub mod api;

#[derive(Debug, Clone)]
pub struct AiSuggestion {
    pub suggestion_type: SuggestionType,
    pub message: String,
    pub confidence: f32,
    pub fix_code: Option<String>,
    pub position: Option<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub enum SuggestionType {
    ErrorExplanation,
    CodeCompletion,
    Optimization,
    BestPractice,
    PatternRecognition,
    PerformanceHint,
}

#[derive(Debug)]
pub struct AiAssistant {
    knowledge_base: knowledge::KnowledgeBase,
    pattern_engine: patterns::PatternEngine,
    completion_engine: completion::CompletionEngine,
}

impl AiAssistant {
    pub fn new() -> Self {
        Self {
            knowledge_base: knowledge::KnowledgeBase::new(),
            pattern_engine: patterns::PatternEngine::new(),
            completion_engine: completion::CompletionEngine::new(),
        }
    }

    pub fn analyze_program(&self, program: &Program) -> Vec<AiSuggestion> {
        let mut suggestions = Vec::new();
        
        suggestions.extend(self.pattern_engine.analyze_patterns(program));
        suggestions.extend(self.optimization::suggest_optimizations(program));
        
        suggestions
    }

    pub fn explain_error(&self, issue: &ValidationIssue) -> AiSuggestion {
        self.explanation::explain_validation_issue(issue, &self.knowledge_base)
    }

    pub fn suggest_completion(&self, source: &str, cursor_pos: usize) -> Vec<AiSuggestion> {
        self.completion_engine.suggest_completions(source, cursor_pos, &self.knowledge_base)
    }

    pub fn analyze_import_schema(&self, schema_info: &HashMap<String, Vec<String>>) -> Vec<AiSuggestion> {
        self.patterns::analyze_database_schema(schema_info, &self.knowledge_base)
    }
}

impl Default for AiAssistant {
    fn default() -> Self {
        Self::new()
    }
}
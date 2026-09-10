use std::collections::HashMap;

#[derive(Debug)]
pub struct KnowledgeBase {
    mdsl_constructs: HashMap<String, ConstructInfo>,
    relationship_types: HashMap<String, RelationshipInfo>,
    common_patterns: Vec<PatternInfo>,
    best_practices: Vec<BestPractice>,
}

#[derive(Debug, Clone)]
pub struct ConstructInfo {
    pub name: String,
    pub description: String,
    pub syntax: String,
    pub examples: Vec<String>,
    pub common_errors: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RelationshipInfo {
    pub name: String,
    pub description: String,
    pub temporal_type: TemporalType,
    pub typical_use_cases: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum TemporalType {
    Synchronous,
    Diachronic,
    Both,
}

#[derive(Debug, Clone)]
pub struct PatternInfo {
    pub name: String,
    pub description: String,
    pub indicators: Vec<String>,
    pub suggested_structure: String,
}

#[derive(Debug, Clone)]
pub struct BestPractice {
    pub category: String,
    pub rule: String,
    pub explanation: String,
    pub examples: Vec<String>,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        let mut kb = Self {
            mdsl_constructs: HashMap::new(),
            relationship_types: HashMap::new(),
            common_patterns: Vec::new(),
            best_practices: Vec::new(),
        };
        
        kb.initialize_constructs();
        kb.initialize_relationships();
        kb.initialize_patterns();
        kb.initialize_best_practices();
        
        kb
    }

    fn initialize_constructs(&mut self) {
        self.mdsl_constructs.insert("UNIT".to_string(), ConstructInfo {
            name: "UNIT".to_string(),
            description: "Defines the structure of data entities in MDSL".to_string(),
            syntax: "UNIT name { field: TYPE, ... }".to_string(),
            examples: vec![
                "UNIT media_outlet { id_mo: ID PRIMARY KEY, mo_title: TEXT(120) }".to_string(),
            ],
            common_errors: vec![
                "Missing PRIMARY KEY specification".to_string(),
                "Invalid field type syntax".to_string(),
            ],
        });

        self.mdsl_constructs.insert("FAMILY".to_string(), ConstructInfo {
            name: "FAMILY".to_string(),
            description: "Groups related media outlets with their relationships".to_string(),
            syntax: "FAMILY \"name\" { OUTLET ... }".to_string(),
            examples: vec![
                "FAMILY \"ORF Network\" { OUTLET \"ORF 1\" { ... }; }".to_string(),
            ],
            common_errors: vec![
                "Missing quotes around family name".to_string(),
                "Empty FAMILY block".to_string(),
            ],
        });

        self.mdsl_constructs.insert("OUTLET".to_string(), ConstructInfo {
            name: "OUTLET".to_string(),
            description: "Defines a specific media outlet within a family".to_string(),
            syntax: "OUTLET \"name\" { identity { id = ...; title = ...; }; }".to_string(),
            examples: vec![
                "OUTLET \"ORF 1\" { identity { id = 300013; title = \"ORF 1\"; }; }".to_string(),
            ],
            common_errors: vec![
                "Missing identity block".to_string(),
                "Invalid ID format".to_string(),
            ],
        });
    }

    fn initialize_relationships(&mut self) {
        self.relationship_types.insert("succession".to_string(), RelationshipInfo {
            name: "succession".to_string(),
            description: "Historical transition from one outlet to another".to_string(),
            temporal_type: TemporalType::Diachronic,
            typical_use_cases: vec![
                "Channel rebrand or reorganization".to_string(),
                "Legal entity changes".to_string(),
            ],
        });

        self.relationship_types.insert("umbrella".to_string(), RelationshipInfo {
            name: "umbrella".to_string(),
            description: "Parent-subsidiary organizational structure".to_string(),
            temporal_type: TemporalType::Synchronous,
            typical_use_cases: vec![
                "Broadcasting corporation structure".to_string(),
                "Media conglomerate ownership".to_string(),
            ],
        });

        self.relationship_types.insert("collaboration".to_string(), RelationshipInfo {
            name: "collaboration".to_string(),
            description: "Partnership or cooperative relationship".to_string(),
            temporal_type: TemporalType::Synchronous,
            typical_use_cases: vec![
                "Content sharing agreements".to_string(),
                "Joint broadcasting initiatives".to_string(),
            ],
        });
    }

    fn initialize_patterns(&mut self) {
        self.common_patterns.push(PatternInfo {
            name: "Broadcasting Corporation".to_string(),
            description: "Large media organization with multiple channels/stations".to_string(),
            indicators: vec![
                "Multiple outlets with similar naming".to_string(),
                "Central umbrella organization".to_string(),
                "Mix of TV and radio channels".to_string(),
            ],
            suggested_structure: "Use umbrella relationships from main corporation to individual channels".to_string(),
        });

        self.common_patterns.push(PatternInfo {
            name: "Media Evolution".to_string(),
            description: "Historical progression of media outlets over time".to_string(),
            indicators: vec![
                "Sequential naming (v1, v2, etc.)".to_string(),
                "Date-based transitions".to_string(),
                "Rebrand or format changes".to_string(),
            ],
            suggested_structure: "Use DIACHRONIC_LINK succession relationships with event dates".to_string(),
        });
    }

    fn initialize_best_practices(&mut self) {
        self.best_practices.push(BestPractice {
            category: "Naming".to_string(),
            rule: "Use consistent identifier prefixes".to_string(),
            explanation: "Group related entities with common prefixes for better organization".to_string(),
            examples: vec![
                "mdsl_media_outlet for generated nodes".to_string(),
                "link_ prefix for relationship identifiers".to_string(),
            ],
        });

        self.best_practices.push(BestPractice {
            category: "Relationships".to_string(),
            rule: "Choose appropriate temporal types".to_string(),
            explanation: "Use SYNCHRONOUS_LINK for current relationships, DIACHRONIC_LINK for historical".to_string(),
            examples: vec![
                "Umbrella: SYNCHRONOUS_LINK (current ownership)".to_string(),
                "Succession: DIACHRONIC_LINK (historical transition)".to_string(),
            ],
        });

        self.best_practices.push(BestPractice {
            category: "Performance".to_string(),
            rule: "Include unique constraints in generated Cypher".to_string(),
            explanation: "Add ID constraints to prevent duplicate nodes and improve query performance".to_string(),
            examples: vec![
                "CREATE CONSTRAINT media_outlet_id_unique FOR (o:media_outlet) REQUIRE o.id_mo IS UNIQUE".to_string(),
            ],
        });
    }

    pub fn get_construct_info(&self, construct: &str) -> Option<&ConstructInfo> {
        self.mdsl_constructs.get(construct)
    }

    pub fn get_relationship_info(&self, relationship: &str) -> Option<&RelationshipInfo> {
        self.relationship_types.get(relationship)
    }

    pub fn get_relevant_patterns(&self, indicators: &[String]) -> Vec<&PatternInfo> {
        self.common_patterns.iter()
            .filter(|pattern| {
                pattern.indicators.iter().any(|indicator| 
                    indicators.iter().any(|user_indicator| 
                        user_indicator.to_lowercase().contains(&indicator.to_lowercase())
                    )
                )
            })
            .collect()
    }

    pub fn get_best_practices(&self, category: Option<&str>) -> Vec<&BestPractice> {
        match category {
            Some(cat) => self.best_practices.iter()
                .filter(|bp| bp.category.eq_ignore_ascii_case(cat))
                .collect(),
            None => self.best_practices.iter().collect(),
        }
    }
}
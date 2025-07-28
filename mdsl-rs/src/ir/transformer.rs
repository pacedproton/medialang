//! AST to IR transformer

use crate::error::Result;
use crate::ir::nodes::*;
use crate::parser::ast::*;

/// Transform AST to IR
pub fn transform(ast: &Program) -> Result<IRProgram> {
    let mut transformer = Transformer::new();
    transformer.transform_program(ast)
}

/// AST to IR transformer
pub struct Transformer;

impl Transformer {
    /// Create a new transformer
    pub fn new() -> Self {
        Self
    }

    /// Transform a program
    pub fn transform_program(&mut self, program: &Program) -> Result<IRProgram> {
        let mut imports = Vec::new();
        let mut variables = Vec::new();
        let mut templates = Vec::new();
        let mut units = Vec::new();
        let mut vocabularies = Vec::new();
        let mut families = Vec::new();
        let mut events = Vec::new();
        let mut top_level_relationships = Vec::new();

        for statement in &program.statements {
            match statement {
                Statement::Import(import) => {
                    imports.push(self.transform_import(import)?);
                }
                Statement::Variable(var) => {
                    variables.push(self.transform_variable(var)?);
                }
                Statement::Template(template) => {
                    templates.push(self.transform_template(template)?);
                }
                Statement::Unit(unit) => {
                    units.push(self.transform_unit(unit)?);
                }
                Statement::Vocabulary(vocab) => {
                    vocabularies.push(self.transform_vocabulary(vocab)?);
                }
                Statement::Family(family) => {
                    families.push(self.transform_family(family)?);
                }
                Statement::Relationship(relationship) => {
                    // Handle top-level relationships
                    top_level_relationships.push(self.transform_relationship(relationship)?);
                }
                Statement::Event(event) => {
                    events.push(self.transform_event(event)?);
                }
                _ => {
                    // Skip other statements for now
                }
            }
        }

        // If we have top-level relationships, create a default family or add them to existing families
        if !top_level_relationships.is_empty() {
            if let Some(first_family) = families.first_mut() {
                // Add relationships to the first family
                first_family.relationships.extend(top_level_relationships);
            } else {
                // Create a default family for top-level relationships
                families.push(IRFamily {
                    name: "Global Relationships".to_string(),
                    outlets: Vec::new(),
                    relationships: top_level_relationships,
                    data_blocks: Vec::new(),
                    comment: Some("Auto-generated family for top-level relationships".to_string()),
                });
            }
        }

        Ok(IRProgram {
            imports,
            variables,
            templates,
            units,
            vocabularies,
            families,
            events,
        })
    }

    /// Transform an import statement
    fn transform_import(&mut self, import: &ImportStatement) -> Result<IRImport> {
        Ok(IRImport {
            path: import.path.clone(),
        })
    }

    /// Transform a variable statement
    fn transform_variable(&mut self, var: &VariableDeclaration) -> Result<IRVariable> {
        Ok(IRVariable {
            name: var.name.clone(),
            value: self.transform_expression(&var.value)?,
        })
    }

    /// Transform a template statement
    fn transform_template(&mut self, template: &TemplateDeclaration) -> Result<IRTemplate> {
        let mut blocks = Vec::new();

        for block in &template.blocks {
            match block {
                OutletBlock::Characteristics(chars) => {
                    let mut characteristics = Vec::new();
                    for field in &chars.fields {
                        if let CharacteristicField::Assignment { name, value, .. } = field {
                            characteristics.push(IRCharacteristic {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                    }
                    blocks.push(IRTemplateBlock::Characteristics(characteristics));
                }
                OutletBlock::Metadata(meta) => {
                    let mut metadata = Vec::new();
                    for field in &meta.fields {
                        if let MetadataField::Assignment { name, value, .. } = field {
                            metadata.push(IRMetadata {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                    }
                    blocks.push(IRTemplateBlock::Metadata(metadata));
                }
                _ => {
                    // Skip other blocks for now
                }
            }
        }

        Ok(IRTemplate {
            name: template.name.clone(),
            template_type: "OUTLET".to_string(), // Templates are always OUTLET type in this DSL
            blocks,
        })
    }

    /// Transform a unit declaration
    fn transform_unit(&mut self, unit: &UnitDeclaration) -> Result<IRUnit> {
        let mut fields = Vec::new();

        for field in &unit.fields {
            fields.push(IRField {
                name: field.name.clone(),
                field_type: self.transform_field_type(&field.field_type)?,
                is_primary_key: field.is_primary_key,
            });
        }

        Ok(IRUnit {
            name: unit.name.clone(),
            fields,
        })
    }

    /// Transform a field type
    fn transform_field_type(&mut self, field_type: &FieldType) -> Result<IRFieldType> {
        match field_type {
            FieldType::Id => Ok(IRFieldType::Id),
            FieldType::Text(length) => Ok(IRFieldType::Text(*length)),
            FieldType::Number => Ok(IRFieldType::Number),
            FieldType::Boolean => Ok(IRFieldType::Boolean),
            FieldType::Category(values) => Ok(IRFieldType::Category(values.clone())),
        }
    }

    /// Transform a vocabulary declaration
    fn transform_vocabulary(&mut self, vocab: &VocabularyDeclaration) -> Result<IRVocabulary> {
        let mut entries = Vec::new();

        // For now, combine all entries from all bodies
        // In the future, we might want to create separate IRVocabulary instances for each body
        for body in &vocab.bodies {
            for entry in &body.entries {
                entries.push(IRVocabularyEntry {
                    key: match &entry.key {
                        VocabularyKey::Number(n) => IRVocabularyKey::Number(*n),
                        VocabularyKey::String(s) => IRVocabularyKey::String(s.clone()),
                    },
                    value: entry.value.clone(),
                });
            }
        }

        // Use the first body's name, or the vocabulary name if no bodies
        let body_name = vocab
            .bodies
            .first()
            .map(|b| b.name.clone())
            .unwrap_or_else(|| vocab.name.clone());

        Ok(IRVocabulary {
            name: vocab.name.clone(),
            body_name,
            entries,
        })
    }

    /// Transform a family declaration
    fn transform_family(&mut self, family: &FamilyDeclaration) -> Result<IRFamily> {
        let mut outlets = Vec::new();
        let mut relationships = Vec::new();
        let mut data_blocks = Vec::new();

        // Extract comment from family members
        let mut comment = None;
        for member in &family.members {
            if let FamilyMember::Comment(comment_stmt) = member {
                comment = Some(comment_stmt.text.clone());
                break;
            }
        }

        for member in &family.members {
            match member {
                FamilyMember::Outlet(outlet) => {
                    outlets.push(self.transform_outlet(outlet)?);
                }
                FamilyMember::Relationship(rel) => {
                    relationships.push(self.transform_relationship(rel)?);
                }
                FamilyMember::Data(data) => {
                    data_blocks.push(self.transform_data_declaration(data)?);
                }
                _ => {
                    // Skip other members for now
                }
            }
        }

        Ok(IRFamily {
            name: family.name.clone(),
            comment,
            outlets,
            relationships,
            data_blocks,
        })
    }

    /// Transform an outlet declaration
    fn transform_outlet(&mut self, outlet: &OutletDeclaration) -> Result<IROutlet> {
        let mut blocks = Vec::new();

        // Extract ID and inheritance information
        let mut id = None;
        let mut template_ref = None;
        let mut base_ref = None;

        if let Some(inheritance) = &outlet.inheritance {
            match inheritance {
                InheritanceClause::ExtendsTemplate(template_name) => {
                    template_ref = Some(template_name.clone());
                }
                InheritanceClause::BasedOn(base_id) => {
                    base_ref = Some(*base_id as u32);
                }
            }
        }

        for block in &outlet.blocks {
            match block {
                OutletBlock::Identity(identity) => {
                    let mut fields = Vec::new();
                    for field in &identity.fields {
                        match field {
                            IdentityField::Assignment { name, value, .. } => {
                                // Check if this is the ID field
                                if name == "id" {
                                    if let Expression::Number(n) = value {
                                        id = Some(*n as u32);
                                    }
                                }
                                fields.push(IRIdentityField {
                                    name: name.clone(),
                                    value: self.transform_expression(value)?,
                                });
                            }
                            _ => {
                                // Skip other field types for now
                            }
                        }
                    }
                    blocks.push(IROutletBlock::Identity(fields));
                }
                OutletBlock::Lifecycle(lifecycle) => {
                    let mut statuses = Vec::new();
                    for entry in &lifecycle.entries {
                        let start_date = match &entry.from {
                            DateExpression::Literal(date) => Some(date.clone()),
                            DateExpression::Current => Some("CURRENT".to_string()),
                        };
                        let end_date = entry.to.as_ref().map(|to| match to {
                            DateExpression::Literal(date) => date.clone(),
                            DateExpression::Current => "CURRENT".to_string(),
                        });

                        // Extract precision and comment from attributes
                        let mut precision_start = None;
                        let mut precision_end = None;
                        let mut comment = None;

                        for attr in &entry.attributes {
                            if let LifecycleAttribute::Assignment { name, value, .. } = attr {
                                match name.as_str() {
                                    "precision_start" => {
                                        if let Expression::String(s) = value {
                                            precision_start = Some(s.clone());
                                        }
                                    }
                                    "precision_end" => {
                                        if let Expression::String(s) = value {
                                            precision_end = Some(s.clone());
                                        }
                                    }
                                    "comment" => {
                                        if let Expression::String(s) = value {
                                            comment = Some(s.clone());
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }

                        statuses.push(IRLifecycleStatus {
                            status: entry.status.clone(),
                            start_date,
                            end_date,
                            precision_start,
                            precision_end,
                            comment,
                        });
                    }
                    blocks.push(IROutletBlock::Lifecycle(statuses));
                }
                OutletBlock::Characteristics(chars) => {
                    let mut characteristics = Vec::new();
                    for field in &chars.fields {
                        if let CharacteristicField::Assignment { name, value, .. } = field {
                            characteristics.push(IRCharacteristic {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                    }
                    blocks.push(IROutletBlock::Characteristics(characteristics));
                }
                OutletBlock::Metadata(meta) => {
                    let mut metadata = Vec::new();
                    for field in &meta.fields {
                        if let MetadataField::Assignment { name, value, .. } = field {
                            metadata.push(IRMetadata {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                    }
                    blocks.push(IROutletBlock::Metadata(metadata));
                }
                _ => {
                    // Skip other blocks for now
                }
            }
        }

        Ok(IROutlet {
            name: outlet.name.clone(),
            id,
            template_ref,
            base_ref,
            blocks,
        })
    }

    /// Transform a data declaration
    fn transform_data_declaration(&mut self, data: &DataDeclaration) -> Result<IRDataBlock> {
        let mut aggregation = Vec::new();
        let mut years = Vec::new();
        let mut maps_to = None;

        for block in &data.blocks {
            match block {
                DataBlock::Annotation(annotation) => {
                    if annotation.name == "maps_to" {
                        maps_to = annotation.value.clone();
                    }
                }
                DataBlock::Aggregation(agg) => {
                    for field in &agg.fields {
                        aggregation.push(IRDataAggregation {
                            name: field.name.clone(),
                            value: field.value.clone(),
                        });
                    }
                }
                DataBlock::Year(year) => {
                    let mut metrics = Vec::new();
                    let mut year_comment = None;

                    for year_block in &year.blocks {
                        match year_block {
                            YearBlock::Metrics(metrics_block) => {
                                for metric in &metrics_block.fields {
                                    let mut value = 0.0;
                                    let mut unit = String::new();
                                    let mut source = String::new();
                                    let mut comment = None;

                                    for attr in &metric.attributes {
                                        match attr.name.as_str() {
                                            "value" => {
                                                if let Expression::Number(n) = &attr.value {
                                                    value = *n;
                                                }
                                            }
                                            "unit" => {
                                                if let Expression::String(s) = &attr.value {
                                                    unit = s.clone();
                                                }
                                            }
                                            "source" => {
                                                if let Expression::String(s) = &attr.value {
                                                    source = s.clone();
                                                }
                                            }
                                            "comment" => {
                                                if let Expression::String(s) = &attr.value {
                                                    comment = Some(s.clone());
                                                }
                                            }
                                            _ => {}
                                        }
                                    }

                                    metrics.push(IRDataMetric {
                                        name: metric.name.clone(),
                                        value,
                                        unit,
                                        source,
                                        comment,
                                    });
                                }
                            }
                            YearBlock::CommentAssignment { value, .. } => {
                                year_comment = Some(value.clone());
                            }
                            _ => {}
                        }
                    }

                    years.push(IRDataYear {
                        year: year.year as u32,
                        metrics,
                        comment: year_comment,
                    });
                }
                _ => {}
            }
        }

        Ok(IRDataBlock {
            outlet_id: data.target_id as u32,
            aggregation,
            years,
            maps_to,
        })
    }

    /// Transform a relationship declaration
    fn transform_relationship(&mut self, rel: &RelationshipDeclaration) -> Result<IRRelationship> {
        match rel {
            RelationshipDeclaration::Diachronic(diachronic) => {
                let mut predecessor = 0;
                let mut successor = 0;
                let mut event_start_date = None;
                let mut event_end_date = None;
                let mut relationship_type = String::new();
                let mut comment = None;
                let mut maps_to = None;

                for field in &diachronic.fields {
                    match field {
                        DiachronicField::Predecessor { value, .. } => {
                            predecessor = *value as u32;
                        }
                        DiachronicField::Successor { value, .. } => {
                            successor = *value as u32;
                        }
                        DiachronicField::EventDate { value, .. } => {
                            event_start_date = Some(match &value.from {
                                DateExpression::Literal(date) => date.clone(),
                                DateExpression::Current => "CURRENT".to_string(),
                            });
                            event_end_date = value.to.as_ref().map(|to| match to {
                                DateExpression::Literal(date) => date.clone(),
                                DateExpression::Current => "CURRENT".to_string(),
                            });
                        }
                        DiachronicField::RelationshipType { value, .. } => {
                            relationship_type = value.clone();
                        }
                        DiachronicField::Annotation(annotation) => {
                            if annotation.name == "maps_to" {
                                maps_to = annotation.value.clone();
                            } else if annotation.name == "comment" {
                                comment = annotation.value.clone();
                            }
                        }
                        _ => {}
                    }
                }

                Ok(IRRelationship::Diachronic(IRDiachronicLink {
                    name: diachronic.name.clone(),
                    predecessor,
                    successor,
                    event_start_date,
                    event_end_date,
                    relationship_type,
                    comment,
                    maps_to,
                }))
            }
            RelationshipDeclaration::Synchronous(sync) => {
                let mut outlet_1_id = 0;
                let mut outlet_1_role = String::new();
                let mut outlet_2_id = 0;
                let mut outlet_2_role = String::new();
                let mut relationship_type = String::new();
                let mut period_start = None;
                let mut period_end = None;
                let mut details = None;
                let mut maps_to = None;

                for field in &sync.fields {
                    match field {
                        SynchronousField::Outlet1 { spec, .. } => {
                            outlet_1_id = spec.id as u32;
                            outlet_1_role = spec.role.clone().unwrap_or_default();
                        }
                        SynchronousField::Outlet2 { spec, .. } => {
                            outlet_2_id = spec.id as u32;
                            outlet_2_role = spec.role.clone().unwrap_or_default();
                        }
                        SynchronousField::RelationshipType { value, .. } => {
                            relationship_type = value.clone();
                        }
                        SynchronousField::Period { value, .. } => {
                            period_start = Some(match &value.from {
                                DateExpression::Literal(date) => date.clone(),
                                DateExpression::Current => "CURRENT".to_string(),
                            });
                            period_end = value.to.as_ref().map(|to| match to {
                                DateExpression::Literal(date) => date.clone(),
                                DateExpression::Current => "CURRENT".to_string(),
                            });
                        }
                        SynchronousField::Details { value, .. } => {
                            details = Some(value.clone());
                        }
                        SynchronousField::Annotation(annotation) => {
                            if annotation.name == "maps_to" {
                                maps_to = annotation.value.clone();
                            }
                        }
                        _ => {}
                    }
                }

                Ok(IRRelationship::Synchronous(IRSynchronousLink {
                    name: sync.name.clone(),
                    outlet_1: IRSyncOutlet {
                        id: outlet_1_id,
                        role: outlet_1_role,
                    },
                    outlet_2: IRSyncOutlet {
                        id: outlet_2_id,
                        role: outlet_2_role,
                    },
                    relationship_type,
                    period_start,
                    period_end,
                    details,
                    maps_to,
                }))
            }
        }
    }

    /// Transform an expression
    fn transform_expression(&mut self, expr: &Expression) -> Result<IRExpression> {
        match expr {
            Expression::String(s) => Ok(IRExpression::String(s.clone())),
            Expression::Number(n) => Ok(IRExpression::Number(*n)),
            Expression::Boolean(b) => Ok(IRExpression::Boolean(*b)),
            Expression::Variable(name) => Ok(IRExpression::Variable(name.clone())),
            Expression::Object(obj) => {
                let mut ir_fields = Vec::new();
                for field in &obj.fields {
                    match field {
                        ObjectField::Assignment { name, value, .. } => {
                            ir_fields.push(IRObjectField {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                        _ => {
                            // Skip other field types for now
                        }
                    }
                }
                Ok(IRExpression::Object(ir_fields))
            }
        }
    }

    /// Transform an event declaration
    fn transform_event(&mut self, event: &EventDeclaration) -> Result<IREvent> {
        let mut event_type = String::new();
        let mut date = None;
        let mut entities = Vec::new();
        let mut impact = Vec::new();
        let mut metadata = Vec::new();
        let mut status = None;

        for field in &event.fields {
            match field {
                EventField::Type { value, .. } => {
                    event_type = value.clone();
                }
                EventField::Date { value, .. } => {
                    date = Some(match value {
                        DateExpression::Literal(date_str) => date_str.clone(),
                        DateExpression::Current => "CURRENT".to_string(),
                    });
                }
                EventField::Entities { entities: entity_list, .. } => {
                    for entity in entity_list {
                        let mut ir_entity = IREventEntity {
                            name: entity.name.clone(),
                            id: 0,
                            role: String::new(),
                            stake_before: None,
                            stake_after: None,
                        };

                        for role in &entity.roles {
                            match role {
                                EntityRole::Id { value, .. } => {
                                    ir_entity.id = *value as u32;
                                }
                                EntityRole::Role { value, .. } => {
                                    ir_entity.role = value.clone();
                                }
                                EntityRole::StakeBefore { value, .. } => {
                                    ir_entity.stake_before = Some(*value);
                                }
                                EntityRole::StakeAfter { value, .. } => {
                                    ir_entity.stake_after = Some(*value);
                                }
                            }
                        }

                        entities.push(ir_entity);
                    }
                }
                EventField::Impact { impact: impacts, .. } => {
                    for impact_field in impacts {
                        impact.push(IREventImpact {
                            name: impact_field.name.clone(),
                            value: self.transform_expression(&impact_field.value)?,
                        });
                    }
                }
                EventField::Metadata { metadata: meta_fields, .. } => {
                    for meta_field in meta_fields {
                        if let MetadataField::Assignment { name, value, .. } = meta_field {
                            metadata.push(IREventMetadata {
                                name: name.clone(),
                                value: self.transform_expression(value)?,
                            });
                        }
                    }
                }
                EventField::Status { value, .. } => {
                    status = Some(value.clone());
                }
                _ => {
                    // Skip other field types for now
                }
            }
        }

        Ok(IREvent {
            name: event.name.clone(),
            event_type,
            date,
            entities,
            impact,
            metadata,
            status,
        })
    }
}

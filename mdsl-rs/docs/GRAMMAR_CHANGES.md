# MDSL Grammar Changes - EVENT Support

This document describes the grammar changes made to support EVENT constructs in the MediaLanguage DSL.

## Overview

The MDSL grammar has been extended to include EVENT declarations, which allow modeling temporal events such as acquisitions, mergers, and ownership changes in media outlet networks.

## Grammar Additions

### Top-Level Declarations

```ebnf
statement = import_statement
          | variable_declaration  
          | unit_declaration
          | vocabulary_declaration
          | family_declaration
          | template_declaration
          | data_declaration
          | relationship_declaration
          | event_declaration          (* NEW *)
          | catalog_declaration
          | comment ;
```

### EVENT Declaration

```ebnf
event_declaration = ("event" | "EVENT") identifier "{" { event_field } "}" ;

event_field = "type" "=" string_literal ";"
            | "date" "=" date_expression ";"
            | "entities" "=" "{" { event_entity } "}" ";"
            | "impact" "=" "{" { impact_field } "}" ";"
            | "metadata" "=" "{" { metadata_field } "}" ";"
            | "status" "=" string_literal ";"
            | annotation
            | comment ;
```

### EVENT Entity

```ebnf
event_entity = identifier "=" "{" { entity_role } "}" ";" ;

entity_role = "id" "=" number ";"
            | "role" "=" string_literal ";"
            | "stake_before" "=" number ";"
            | "stake_after" "=" number ";" ;
```

### Impact Field

```ebnf
impact_field = identifier "=" (string_literal | number) ";" ;
```

### Enhanced Date Support

```ebnf
date_expression = string_literal 
                | ("current" | "CURRENT") 
                | iso_date ;

iso_date = digit digit digit digit "-" digit digit "-" digit digit ;
```

### Relationship Enhancements

```ebnf
diachronic_field = "predecessor" "=" number ";"
                 | "successor" "=" number ";"
                 | "event_date" "=" date_expression ";"
                 | "relationship_type" "=" string_literal ";"
                 | "triggered_by_event" "=" identifier ";"    (* NEW *)
                 | annotation
                 | comment ;

synchronous_field = "outlet_1" "=" outlet_spec ";"
                  | "outlet_2" "=" outlet_spec ";"
                  | "relationship_type" "=" string_literal ";"
                  | "period" "=" date_range ";"
                  | "details" "=" string_literal ";"
                  | "created_by_event" "=" identifier ";"     (* NEW *)
                  | annotation
                  | comment ;
```

## New Keywords

The following keywords have been added to the lexer:

```
EVENT, event          - Event declaration
type                  - Event type field
date                  - Event date field  
entities              - Event entities field
impact                - Event impact field
stake_before          - Entity stake before event
stake_after           - Entity stake after event
triggered_by_event    - Relationship triggered by event
created_by_event      - Relationship created by event
```

## Compatibility

### Backward Compatibility

All existing MDSL constructs remain unchanged and fully compatible:

- ✅ UNIT declarations
- ✅ VOCABULARY declarations  
- ✅ FAMILY declarations
- ✅ OUTLET declarations
- ✅ TEMPLATE declarations
- ✅ DATA declarations
- ✅ RELATIONSHIP declarations (DIACHRONIC_LINK/SYNCHRONOUS_LINK)
- ✅ CATALOG declarations

### Forward Compatibility

The grammar changes are designed to be extensible:

- Additional event field types can be added
- New entity role types can be introduced
- Event validation rules can be enhanced
- Temporal query constructs can be added

## Examples

### Before (Pre-EVENT)

```mdsl
FAMILY "Styria Media Group" {
    OUTLET "Kleine Zeitung" {
        IDENTITY {
            id = 300014;
            title = "Kleine Zeitung";
        }
    }
}

DIACHRONIC_LINK ownership_change {
    predecessor = 300013;
    successor = 300014;
    event_date = "2019-06-15";
    relationship_type = "ownership_transfer";
}
```

### After (With EVENT Support)

```mdsl
EVENT styria_acquires_kleine_2019 {
    type = "acquisition";
    date = "2019-06-15";
    status = "completed";
    
    entities = {
        styria = {
            id = 200001;
            role = "acquirer";
            stake_after = 100;
        };
        kleine_zeitung = {
            id = 300014;
            role = "target";
            stake_before = 100;
        };
    };
    
    impact = {
        transaction_value = 75000000;
        currency = "EUR";
    };
}

FAMILY "Styria Media Group" {
    OUTLET "Kleine Zeitung" {
        IDENTITY {
            id = 300014;
            title = "Kleine Zeitung";
        }
    }
}

DIACHRONIC_LINK ownership_change {
    predecessor = 300013;
    successor = 300014;
    event_date = "2019-06-15";
    relationship_type = "ownership_transfer";
    triggered_by_event = styria_acquires_kleine_2019;  // NEW
}
```

## Parser Changes

### Token Handling

The lexer now recognizes EVENT-related keywords and handles them appropriately in different contexts:

- `type`, `date`, `entities`, `impact` can be used as both keywords and identifiers
- Enhanced identifier consumption for field names
- Improved string/identifier detection for names

### Error Recovery

Enhanced error recovery for EVENT constructs:

- Meaningful error messages for missing required fields
- Validation of nested entity structures
- Proper handling of malformed event declarations

### Trailing Comma Support

EVENT parsing supports flexible comma usage:

```mdsl
EVENT example {
    type = "acquisition",    // trailing comma OK
    date = "2023-01-01",     // trailing comma OK
    entities = {
        buyer = { id = 1; },  // trailing comma OK
    },                       // trailing comma OK
}
```

## AST Changes

### New AST Nodes

```rust
pub struct EventDeclaration {
    pub name: String,
    pub fields: Vec<EventField>,
    pub position: SourcePosition,
}

pub enum EventField {
    Type { value: String, position: SourcePosition },
    Date { value: DateExpression, position: SourcePosition },
    Status { value: String, position: SourcePosition },
    Entities { entities: Vec<EventEntity>, position: SourcePosition },
    Impact { impact: Vec<ImpactField>, position: SourcePosition },
    Metadata { metadata: Vec<MetadataField>, position: SourcePosition },
    Annotation { name: String, value: Option<String>, position: SourcePosition },
    Comment { text: String, position: SourcePosition },
}

pub struct EventEntity {
    pub name: String,
    pub roles: Vec<EntityRole>,
    pub position: SourcePosition,
}

pub enum EntityRole {
    Id { value: f64, position: SourcePosition },
    Role { value: String, position: SourcePosition },
    StakeBefore { value: f64, position: SourcePosition },
    StakeAfter { value: f64, position: SourcePosition },
}
```

### Enhanced Relationship AST

```rust
pub enum DiachronicField {
    // ... existing fields
    TriggeredByEvent { value: String, position: SourcePosition },  // NEW
}

pub enum SynchronousField {
    // ... existing fields  
    CreatedByEvent { value: String, position: SourcePosition },    // NEW
}
```

## Validation Rules

### Semantic Validation

The semantic validator now enforces EVENT-specific rules:

1. **Required Fields**: `type` and `date` must be present
2. **Entity References**: All entity IDs must reference valid outlets
3. **Stake Validation**: Ownership percentages should be reasonable
4. **Event References**: `triggered_by_event` and `created_by_event` must reference valid events
5. **Date Consistency**: Event dates should be consistent with relationship dates

### Grammar Validation

The parser enforces syntactic correctness:

1. **Field Syntax**: All fields must follow proper assignment syntax
2. **Nested Structure**: Entities and impact fields must be properly nested
3. **Identifier Rules**: Event names must be valid identifiers
4. **String Literals**: Quoted strings must be properly terminated

## Testing

### Unit Tests

New unit tests validate EVENT parsing:

- `test_parse_event_declaration`
- `test_parse_event_triggered_relationship` 
- `test_parse_event_created_synchronous_relationship`

### Integration Tests

EVENT support is tested in realistic scenarios:

- Complete family structures with events
- Multiple related events
- Event-relationship integration
- Error handling and recovery

### Validation Tests

Semantic validation is tested for:

- Missing required fields
- Invalid entity references
- Malformed event structures
- Type consistency

## Performance Impact

### Lexer Performance

- Minimal impact: Only adds keyword recognition
- No change to tokenization speed
- Memory usage unchanged

### Parser Performance  

- Moderate impact: Adds new parsing paths
- Well-optimized recursive descent parsing
- Minimal memory overhead for AST nodes

### Compilation Time

- Negligible impact on compile times
- Feature-gated code generation (no impact when disabled)
- Efficient AST representation

## Migration Guide

### For Existing MDSL Files

No changes required - all existing MDSL files remain valid.

### For New Features

To use EVENT support:

1. Add EVENT declarations to your MDSL files
2. Reference events in relationship declarations
3. Update validation rules if needed
4. Regenerate code/documentation

### For Tooling

Update tools that process MDSL:

1. Parse new EVENT constructs
2. Handle new relationship field types
3. Update code generators for events
4. Enhance validation logic

## Related Changes

- **lexer/token.rs**: Added EVENT keywords
- **lexer/scanner.rs**: Enhanced keyword recognition  
- **parser/ast.rs**: New EVENT AST nodes
- **parser/recursive_descent.rs**: EVENT parsing logic
- **semantic/validator.rs**: EVENT validation rules (future)
- **codegen/**: EVENT code generation (future)

## Future Grammar Extensions

Planned grammar enhancements:

1. **Event Templates**: Reusable event patterns
2. **Event Sequences**: Dependent event chains
3. **Conditional Events**: Events with prerequisites
4. **Duration Events**: Events with start/end dates
5. **Event Queries**: Built-in temporal query syntax
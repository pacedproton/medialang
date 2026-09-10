# EVENT Support in MDSL

This document describes the EVENT construct feature added to the MediaLanguage DSL (MDSL) for modeling temporal events such as acquisitions, mergers, ownership changes, and other meta-events in media outlet networks.

## Overview

The EVENT construct allows you to model significant temporal events that affect media outlets and their relationships. Events can represent acquisitions, mergers, ownership transfers, regulatory changes, or any other significant occurrence that impacts the media landscape.

## Syntax

### Basic EVENT Declaration

```mdsl
EVENT event_name {
    type = "event_type";
    date = "YYYY-MM-DD" | CURRENT;
    status = "status_value";
    
    entities = {
        entity_name = {
            id = outlet_id;
            role = "role_description";
            stake_before = percentage;
            stake_after = percentage;
        };
        // ... more entities
    };
    
    impact = {
        field_name = value;
        // ... more impact fields
    };
    
    metadata = {
        field_name = "value";
        // ... more metadata
    };
    
    @annotation_name = "annotation_value"
}
```

### Field Descriptions

#### Required Fields

- **`type`**: String describing the type of event (e.g., "acquisition", "merger", "divestiture")
- **`date`**: Event date as string literal or `CURRENT` keyword

#### Optional Fields

- **`status`**: String describing event status (e.g., "completed", "pending", "cancelled")
- **`entities`**: Map of participating entities with their roles and stake changes
- **`impact`**: Financial or structural impact data
- **`metadata`**: Additional descriptive information
- **Annotations**: Using `@annotation_name` syntax

### Entity Roles

Within the `entities` block, each entity can have:

- **`id`**: Numeric identifier of the outlet
- **`role`**: String describing the entity's role (e.g., "acquirer", "target", "subsidiary")
- **`stake_before`**: Ownership percentage before the event
- **`stake_after`**: Ownership percentage after the event

## Examples

### Simple Acquisition Event

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
    
    metadata = {
        regulatory_approval = "Austrian_Media_Authority";
        announcement_date = "2019-05-01";
    };
    
    @source = "Financial_Times_2019_06_16"
}
```

### Partial Acquisition

```mdsl
EVENT ringier_investment_2020 {
    type = "investment";
    date = "2020-03-10";
    status = "completed";
    
    entities = {
        ringier = {
            id = 400001;
            role = "investor";
            stake_after = 25;
        };
        target_outlet = {
            id = 500023;
            role = "recipient";
            stake_before = 100;
            stake_after = 75;
        };
    };
    
    impact = {
        investment_amount = 15000000;
        currency = "EUR";
        strategic_focus = "digital_expansion";
    };
}
```

### Merger Event

```mdsl
EVENT media_group_merger_2021 {
    type = "merger";
    date = "2021-09-01";
    status = "pending";
    
    entities = {
        company_a = {
            id = 600001;
            role = "merging_party";
            stake_before = 100;
            stake_after = 50;
        };
        company_b = {
            id = 600002;
            role = "merging_party";
            stake_before = 100;
            stake_after = 50;
        };
    };
    
    impact = {
        combined_circulation = 450000;
        expected_synergies = 12000000;
        currency = "EUR";
    };
    
    metadata = {
        regulatory_review = "pending";
        expected_completion = "2022-Q1";
    };
}
```

## Event-Relationship Integration

Events can be linked to relationships using special fields:

### Diachronic Links with Events

```mdsl
DIACHRONIC_LINK ownership_transfer_2019 {
    predecessor = 300014;
    successor = 300014;
    event_date = "2019-06-15";
    relationship_type = "ownership_change";
    triggered_by_event = styria_acquires_kleine_2019;
}
```

### Synchronous Links with Events

```mdsl
SYNCHRONOUS_LINK parent_subsidiary_2019 {
    outlet_1 = { id = 200001; role = "parent"; };
    outlet_2 = { id = 300014; role = "subsidiary"; };
    relationship_type = "ownership";
    created_by_event = styria_acquires_kleine_2019;
}
```

## Use Cases

### 1. Ownership Change Tracking

Track when media outlets change ownership, including partial acquisitions and full takeovers.

### 2. Corporate Restructuring

Model mergers, spin-offs, and other corporate restructuring events that affect media outlet structures.

### 3. Investment Tracking

Record investment rounds, strategic partnerships, and joint ventures.

### 4. Regulatory Events

Document regulatory decisions, license transfers, and compliance-related changes.

### 5. Market Analysis

Analyze temporal patterns in media ownership consolidation and market dynamics.

## Graph Representation

In graph databases (Neo4j), events are represented as:

- **Nodes**: Events become vertices/nodes in the graph
- **Relationships**: Events connect to participating outlets via relationships
- **Properties**: All event fields become node properties
- **Temporal Queries**: Enable time-based analysis of ownership networks

### Example Cypher Query

```cypher
// Find all acquisitions involving Styria Media Group
MATCH (event:Event {type: "acquisition"})-[:INVOLVES]->(outlet:Outlet)
WHERE outlet.name CONTAINS "Styria" OR event.entities CONTAINS "styria"
RETURN event, outlet
```

## Validation Rules

The MDSL semantic validator enforces:

1. **Required Fields**: `type` and `date` must be present
2. **Date Format**: Dates must be valid ISO format strings or `CURRENT`
3. **Entity IDs**: All entity IDs must reference valid outlets
4. **Stake Validation**: Ownership percentages should be between 0-100
5. **Type Consistency**: Event types should be from known vocabularies

## Best Practices

### 1. Consistent Naming

Use descriptive, consistent naming conventions:
- `company_acquires_target_year`
- `merger_company1_company2_year`
- `investment_round_target_year`

### 2. Complete Entity Information

Always include:
- Clear role descriptions
- Relevant stake information
- Proper outlet ID references

### 3. Meaningful Impact Data

Include quantitative impact data when available:
- Transaction values
- Market share changes
- Financial metrics

### 4. Source Documentation

Use annotations to track data sources:
```mdsl
@source = "Financial_Times_2019_06_16"
@confidence = "high"
@verification_date = "2023-11-15"
```

### 5. Status Tracking

Maintain clear status information:
- `"completed"` - Event has occurred
- `"pending"` - Event announced but not completed
- `"cancelled"` - Event was cancelled
- `"rumored"` - Unconfirmed reports

## Implementation Notes

### Parser Features

- Full EVENT declaration parsing
- Nested entity and impact field support
- Integration with existing relationship parsing
- Comprehensive error handling and validation

### AST Representation

Events are represented in the AST as `EventDeclaration` nodes with:
- Structured field types (`EventField` enum)
- Entity role modeling (`EntityRole` enum)
- Impact and metadata support
- Source position tracking for error reporting

### Code Generation

Events are supported in:
- **SQL Generation**: Creates event tables with proper foreign key relationships
- **Cypher Generation**: Generates Neo4j CREATE statements for event nodes
- **Documentation**: Automatic schema documentation generation

## Future Enhancements

Potential future additions to EVENT support:

1. **Event Hierarchies**: Support for event sequences and dependencies
2. **Duration Events**: Events with start and end dates
3. **Event Templates**: Reusable event patterns
4. **Conditional Events**: Events that depend on other events
5. **Event Validation**: Enhanced validation rules for specific event types
6. **Temporal Queries**: Built-in support for temporal relationship queries

## Related Documentation

- [MDSL Grammar Reference](./GRAMMAR.md)
- [Relationship Modeling](./RELATIONSHIPS.md)
- [Code Generation](./CODE_GENERATION.md)
- [Validation Rules](./VALIDATION.md)
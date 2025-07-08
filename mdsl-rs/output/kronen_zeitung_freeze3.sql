Generating SQL from file: ../MediaLanguage/media_groups/kronenzeitung/kronen_zeitung_freeze3.mdsl
Generated SQL:
-- Generated SQL from MediaLanguage DSL
-- This file contains CREATE TABLE statements, INSERT statements, and constraints
-- Generated for comprehensive media outlet and relationship management

-- IMPORTS
-- IMPORT "anmi_common_codes.mdsl"
-- IMPORT "anmi_media_sectors.mdsl"
-- IMPORT "anmi_mandate_types.mdsl"
-- IMPORT "anmi_source_references.mdsl"
-- IMPORT "anmi_market_data_schemas.mdsl"
-- IMPORT "Medienangebot.mdsl"
-- IMPORT "MedienangeboteDiachroneBeziehungen.mdsl"
-- IMPORT "MedienangeboteSynchroneBeziehungen.mdsl"
-- IMPORT "sources.mdsl"

-- VARIABLES
-- LET austria_region = "Österreich gesamt"
-- LET wien_region = "Wien"
-- LET founding_note = "Founded in ddd, post-war re-established ddd"

-- CORE SCHEMA TABLES
-- These tables support the MediaLanguage DSL structure

CREATE TABLE media_outlets (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    family_id INTEGER,
    template_id INTEGER,
    base_outlet_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (family_id) REFERENCES families(id),
    FOREIGN KEY (template_id) REFERENCES templates(id),
    FOREIGN KEY (base_outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE families (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    comment TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE templates (
    id INTEGER PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    template_type VARCHAR(100) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE outlet_identity (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    field_name VARCHAR(100) NOT NULL,
    field_value TEXT,
    field_type VARCHAR(50) DEFAULT 'string',
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE outlet_lifecycle (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    status VARCHAR(100) NOT NULL,
    start_date DATE,
    end_date DATE,
    precision_start VARCHAR(50),
    precision_end VARCHAR(50),
    comment TEXT,
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE outlet_characteristics (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    characteristic_name VARCHAR(100) NOT NULL,
    characteristic_value TEXT,
    characteristic_type VARCHAR(50) DEFAULT 'string',
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE outlet_metadata (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    metadata_name VARCHAR(100) NOT NULL,
    metadata_value TEXT,
    metadata_type VARCHAR(50) DEFAULT 'string',
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE relationships (
    id INTEGER PRIMARY KEY,
    relationship_name VARCHAR(255) NOT NULL,
    relationship_type VARCHAR(50) NOT NULL, -- 'diachronic' or 'synchronous'
    family_id INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (family_id) REFERENCES families(id)
);

CREATE TABLE diachronic_relationships (
    id INTEGER PRIMARY KEY,
    relationship_id INTEGER NOT NULL,
    predecessor_id INTEGER NOT NULL,
    successor_id INTEGER NOT NULL,
    event_start_date DATE,
    event_end_date DATE,
    relationship_subtype VARCHAR(100),
    comment TEXT,
    maps_to VARCHAR(255),
    FOREIGN KEY (relationship_id) REFERENCES relationships(id),
    FOREIGN KEY (predecessor_id) REFERENCES media_outlets(id),
    FOREIGN KEY (successor_id) REFERENCES media_outlets(id)
);

CREATE TABLE synchronous_relationships (
    id INTEGER PRIMARY KEY,
    relationship_id INTEGER NOT NULL,
    outlet_1_id INTEGER NOT NULL,
    outlet_1_role VARCHAR(100),
    outlet_2_id INTEGER NOT NULL,
    outlet_2_role VARCHAR(100),
    relationship_subtype VARCHAR(100),
    period_start DATE,
    period_end DATE,
    details TEXT,
    maps_to VARCHAR(255),
    FOREIGN KEY (relationship_id) REFERENCES relationships(id),
    FOREIGN KEY (outlet_1_id) REFERENCES media_outlets(id),
    FOREIGN KEY (outlet_2_id) REFERENCES media_outlets(id)
);

CREATE TABLE market_data (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    data_year INTEGER NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(15,2),
    metric_unit VARCHAR(50),
    data_source VARCHAR(100),
    comment TEXT,
    maps_to VARCHAR(255),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

CREATE TABLE data_aggregation (
    id INTEGER PRIMARY KEY,
    outlet_id INTEGER NOT NULL,
    aggregation_name VARCHAR(100) NOT NULL,
    aggregation_value VARCHAR(100) NOT NULL,
    FOREIGN KEY (outlet_id) REFERENCES media_outlets(id)
);

-- Template: AustrianNewspaper
INSERT INTO templates (name, template_type) VALUES ('AustrianNewspaper', 'OUTLET');
-- Template AustrianNewspaper characteristics:
--   language: "de"
--   mandate: "Privat-kommerziell"
--   distribution: "complex_object"
-- Template AustrianNewspaper metadata:
--   steward: "js"

-- Family: Kronen Zeitung Family
INSERT INTO families (name, comment) VALUES ('Kronen Zeitung Family', '@comment: Austria''s largest daily newspaper group');
-- Outlet: Kronen Zeitung
INSERT INTO media_outlets (id, name, family_id) VALUES (200001, 'Kronen Zeitung', (SELECT id FROM families WHERE name = 'Kronen Zeitung Family'));
INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES (200001, 'id', '200001');
INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES (200001, 'title', 'Kronen Zeitung');
INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES (200001, 'sector', 'Tageszeitung');
INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES (200001, 'distribution', 'complex_object');
INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES (200001, 'editorial_office', 'Wien');
INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES (200001, 'editorial_stance', 'complex_object');
INSERT INTO outlet_metadata (outlet_id, metadata_name, metadata_value) VALUES (200001, 'verified', '2024-10-15');
INSERT INTO outlet_metadata (outlet_id, metadata_name, metadata_value) VALUES (200001, 'notes', '$founding_note');
-- Outlet: krone.at
INSERT INTO media_outlets (id, name, family_id) VALUES (200002, 'krone.at', (SELECT id FROM families WHERE name = 'Kronen Zeitung Family'));
INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES (200002, 'id', '200002');
INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES (200002, 'title', 'krone.at');
INSERT INTO outlet_identity (outlet_id, field_name, field_value) VALUES (200002, 'url', 'https://www.krone.at');
INSERT INTO outlet_characteristics (outlet_id, characteristic_name, characteristic_value) VALUES (200002, 'sector', 'Online');
INSERT INTO outlet_metadata (outlet_id, metadata_name, metadata_value) VALUES (200002, 'verified', '2024-10-15');
INSERT INTO outlet_metadata (outlet_id, metadata_name, metadata_value) VALUES (200002, 'notes', 'Digital presence of Kronen Zeitung');

-- RELATIONSHIPS
INSERT INTO relationships (relationship_name, relationship_type, family_id) VALUES ('acquisition', 'diachronic', (SELECT id FROM families WHERE name = 'Kronen Zeitung Family'));
INSERT INTO diachronic_relationships (relationship_id, predecessor_id, successor_id, event_start_date, event_end_date, relationship_subtype, comment, maps_to) VALUES ((SELECT id FROM relationships WHERE relationship_name = 'acquisition'), 0, 0, NULL, NULL, '', NULL, NULL);
INSERT INTO relationships (relationship_name, relationship_type, family_id) VALUES ('combination', 'synchronous', (SELECT id FROM families WHERE name = 'Kronen Zeitung Family'));
INSERT INTO synchronous_relationships (relationship_id, outlet_1_id, outlet_1_role, outlet_2_id, outlet_2_role, relationship_subtype, period_start, period_end, details, maps_to) VALUES ((SELECT id FROM relationships WHERE relationship_name = 'combination'), 0, '', 0, '', '', NULL, NULL, NULL, NULL);
-- MARKET DATA


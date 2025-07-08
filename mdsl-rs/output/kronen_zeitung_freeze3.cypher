Generating Cypher from file: ../MediaLanguage/media_groups/kronenzeitung/kronen_zeitung_freeze3.mdsl
Generated Cypher:
// Generated Cypher from MediaLanguage DSL
// This file contains CREATE statements for Neo4j graph database
// Represents media outlets, families, and relationships as a graph

// IMPORTS
// IMPORT "anmi_common_codes.mdsl"
// IMPORT "anmi_media_sectors.mdsl"
// IMPORT "anmi_mandate_types.mdsl"
// IMPORT "anmi_source_references.mdsl"
// IMPORT "anmi_market_data_schemas.mdsl"
// IMPORT "Medienangebot.mdsl"
// IMPORT "MedienangeboteDiachroneBeziehungen.mdsl"
// IMPORT "MedienangeboteSynchroneBeziehungen.mdsl"
// IMPORT "sources.mdsl"

// VARIABLES
// LET austria_region = "Österreich gesamt"
// LET wien_region = "Wien"
// LET founding_note = "Founded in ddd, post-war re-established ddd"

// CONSTRAINTS AND INDEXES
// Create constraints for unique identifiers

CREATE CONSTRAINT outlet_id_unique IF NOT EXISTS FOR (o:Outlet) REQUIRE o.id IS UNIQUE;
CREATE CONSTRAINT family_name_unique IF NOT EXISTS FOR (f:Family) REQUIRE f.name IS UNIQUE;
CREATE CONSTRAINT template_name_unique IF NOT EXISTS FOR (t:Template) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT vocab_name_unique IF NOT EXISTS FOR (v:Vocabulary) REQUIRE v.name IS UNIQUE;

CREATE INDEX outlet_name_index IF NOT EXISTS FOR (o:Outlet) ON (o.name);
CREATE INDEX family_name_index IF NOT EXISTS FOR (f:Family) ON (f.name);
CREATE INDEX data_year_index IF NOT EXISTS FOR (d:MarketData) ON (d.year);
CREATE INDEX metric_name_index IF NOT EXISTS FOR (m:Metric) ON (m.name);

// Template: AustrianNewspaper
CREATE (t:Template {name: 'AustrianNewspaper', type: 'OUTLET', created_at: datetime()});
CREATE (c:Characteristic {name: 'language', value: 'de', template_name: 'AustrianNewspaper'});
MATCH (t:Template {name: 'AustrianNewspaper'}), (c:Characteristic {name: 'language', template_name: 'AustrianNewspaper'}) CREATE (t)-[:HAS_CHARACTERISTIC]->(c);
CREATE (c:Characteristic {name: 'mandate', value: 'Privat-kommerziell', template_name: 'AustrianNewspaper'});
MATCH (t:Template {name: 'AustrianNewspaper'}), (c:Characteristic {name: 'mandate', template_name: 'AustrianNewspaper'}) CREATE (t)-[:HAS_CHARACTERISTIC]->(c);
CREATE (c:Characteristic {name: 'distribution', value: 'complex_object', template_name: 'AustrianNewspaper'});
MATCH (t:Template {name: 'AustrianNewspaper'}), (c:Characteristic {name: 'distribution', template_name: 'AustrianNewspaper'}) CREATE (t)-[:HAS_CHARACTERISTIC]->(c);
CREATE (m:Metadata {name: 'steward', value: 'js', template_name: 'AustrianNewspaper'});
MATCH (t:Template {name: 'AustrianNewspaper'}), (m:Metadata {name: 'steward', template_name: 'AustrianNewspaper'}) CREATE (t)-[:HAS_METADATA]->(m);

// Family: Kronen Zeitung Family
CREATE (f:Family {name: 'Kronen Zeitung Family', comment: '@comment: Austria\'s largest daily newspaper group', created_at: datetime()});
// Outlet: Kronen Zeitung
CREATE (o:Outlet {id: 200001, name: 'Kronen Zeitung', family_name: 'Kronen Zeitung Family', created_at: datetime()});
MATCH (f:Family {name: 'Kronen Zeitung Family'}), (o:Outlet {id: 200001}) CREATE (f)-[:HAS_OUTLET]->(o);
MATCH (t:Template {name: 'AustrianNewspaper'}), (o:Outlet {id: 200001}) CREATE (o)-[:EXTENDS_TEMPLATE]->(t);
CREATE (i:Identity {name: 'id', value: '200001', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (i:Identity {name: 'id', outlet_id: 200001}) CREATE (o)-[:HAS_IDENTITY]->(i);
CREATE (i:Identity {name: 'title', value: 'Kronen Zeitung', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (i:Identity {name: 'title', outlet_id: 200001}) CREATE (o)-[:HAS_IDENTITY]->(i);
CREATE (c:Characteristic {name: 'sector', value: 'Tageszeitung', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (c:Characteristic {name: 'sector', outlet_id: 200001}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);
CREATE (c:Characteristic {name: 'distribution', value: 'complex_object', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (c:Characteristic {name: 'distribution', outlet_id: 200001}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);
CREATE (c:Characteristic {name: 'editorial_office', value: 'Wien', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (c:Characteristic {name: 'editorial_office', outlet_id: 200001}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);
CREATE (c:Characteristic {name: 'editorial_stance', value: 'complex_object', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (c:Characteristic {name: 'editorial_stance', outlet_id: 200001}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);
CREATE (m:Metadata {name: 'verified', value: '2024-10-15', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (m:Metadata {name: 'verified', outlet_id: 200001}) CREATE (o)-[:HAS_METADATA]->(m);
CREATE (m:Metadata {name: 'notes', value: '$founding_note', outlet_id: 200001});
MATCH (o:Outlet {id: 200001}), (m:Metadata {name: 'notes', outlet_id: 200001}) CREATE (o)-[:HAS_METADATA]->(m);
// Outlet: krone.at
CREATE (o:Outlet {id: 200002, name: 'krone.at', family_name: 'Kronen Zeitung Family', created_at: datetime()});
MATCH (f:Family {name: 'Kronen Zeitung Family'}), (o:Outlet {id: 200002}) CREATE (f)-[:HAS_OUTLET]->(o);
MATCH (base:Outlet {id: 200001}), (o:Outlet {id: 200002}) CREATE (o)-[:BASED_ON]->(base);
CREATE (i:Identity {name: 'id', value: '200002', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (i:Identity {name: 'id', outlet_id: 200002}) CREATE (o)-[:HAS_IDENTITY]->(i);
CREATE (i:Identity {name: 'title', value: 'krone.at', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (i:Identity {name: 'title', outlet_id: 200002}) CREATE (o)-[:HAS_IDENTITY]->(i);
CREATE (i:Identity {name: 'url', value: 'https://www.krone.at', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (i:Identity {name: 'url', outlet_id: 200002}) CREATE (o)-[:HAS_IDENTITY]->(i);
CREATE (c:Characteristic {name: 'sector', value: 'Online', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (c:Characteristic {name: 'sector', outlet_id: 200002}) CREATE (o)-[:HAS_CHARACTERISTIC]->(c);
CREATE (m:Metadata {name: 'verified', value: '2024-10-15', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (m:Metadata {name: 'verified', outlet_id: 200002}) CREATE (o)-[:HAS_METADATA]->(m);
CREATE (m:Metadata {name: 'notes', value: 'Digital presence of Kronen Zeitung', outlet_id: 200002});
MATCH (o:Outlet {id: 200002}), (m:Metadata {name: 'notes', outlet_id: 200002}) CREATE (o)-[:HAS_METADATA]->(m);

// RELATIONSHIPS
// Diachronic relationship: acquisition
MATCH (pred:Outlet {id: 0}), (succ:Outlet {id: 0}) CREATE (pred)-[:DIACHRONIC_LINK {name: 'acquisition', relationship_type: '', event_start_date: null, event_end_date: null, comment: null, maps_to: null}]->(succ);
// Synchronous relationship: combination
MATCH (o1:Outlet {id: 0}), (o2:Outlet {id: 0}) CREATE (o1)-[:SYNCHRONOUS_LINK {name: 'combination', relationship_type: '', outlet_1_role: '', outlet_2_role: '', period_start: null, period_end: null, details: null, maps_to: null}]->(o2);
// MARKET DATA


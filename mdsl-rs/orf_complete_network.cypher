Generating Cypher from file: orf_complete_network.mdsl
Generated Cypher:
// Generated Cypher from MediaLanguage DSL
// This file contains CREATE statements for Neo4j graph database
// Represents media outlets, families, and relationships as a graph

// IMPORTS
// IMPORT "anmi_common_codes.mdsl"
// IMPORT "anmi_media_sectors.mdsl"
// IMPORT "anmi_mandate_types.mdsl"
// IMPORT "anmi_source_references.mdsl"

// CONSTRAINTS AND INDEXES
// Create constraints for unique identifiers

CREATE CONSTRAINT mdsl_media_outlet_id_unique IF NOT EXISTS FOR (o:mdsl_media_outlet) REQUIRE o.id_mo IS UNIQUE;
CREATE CONSTRAINT mdsl_family_name_unique IF NOT EXISTS FOR (f:mdsl_Family) REQUIRE f.name IS UNIQUE;
CREATE CONSTRAINT mdsl_template_name_unique IF NOT EXISTS FOR (t:mdsl_Template) REQUIRE t.name IS UNIQUE;
CREATE CONSTRAINT mdsl_vocab_name_unique IF NOT EXISTS FOR (v:mdsl_Vocabulary) REQUIRE v.name IS UNIQUE;

CREATE INDEX mdsl_media_outlet_title_index IF NOT EXISTS FOR (o:mdsl_media_outlet) ON (o.mo_title);
CREATE INDEX mdsl_family_name_index IF NOT EXISTS FOR (f:mdsl_Family) ON (f.name);
CREATE INDEX mdsl_data_year_index IF NOT EXISTS FOR (d:mdsl_MarketData) ON (d.year);
CREATE INDEX mdsl_metric_name_index IF NOT EXISTS FOR (m:mdsl_Metric) ON (m.name);

// Vocabulary: DataSources
CREATE (v:Vocabulary {name: 'DataSources', body_name: 'TYPES', created_at: datetime()});
CREATE (e:VocabularyEntry {key: '1', value: 'In_Angabe_zu__Dach___Kombi__oder__Hauptausgabe_inkludiert_im_Jahr_2021', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '1', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '2', value: 'In_Angabe_zu__Dach___Kombi__oder__Hauptausgabe_inkludiert_im_Jahr_2022', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '2', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '3', value: 'In_Angabe_zu__Dach___Kombi__oder__Hauptausgabe_inkludiert_im_Jahr_2023', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '3', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '4', value: 'Massenmedien_in_Oesterreich___Medienbericht_Bd__4__1993', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '4', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '5', value: 'Media_Analyse_1993', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '5', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '6', value: 'Media_Analyse_1994', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '6', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '7', value: 'Media_Analyse_1995', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '7', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '8', value: 'Media_Analyse_1996', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '8', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '9', value: 'Media_Analyse_1997', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '9', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '10', value: 'Media_Analyse_1998', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '10', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '11', value: 'Media_Analyse_1999', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '11', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '12', value: 'Media_Analyse_2000', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '12', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '13', value: 'Media_Analyse_2001', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '13', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '14', value: 'Media_Analyse_2002', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '14', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '15', value: 'Media_Analyse_2003', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '15', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '16', value: 'Media_Analyse_2004_zitiert_nach_derstandart_at_23_03_2005', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '16', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '17', value: 'Media_Analyse_2005_zitiert_nach_derstandart_at_29_03_2006', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '17', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '18', value: 'Media_Analyse_2006_zitiert_nach_derstandart_at_27_03_2007', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '18', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '19', value: 'Media_Analyse_2007_zitiert_nach_derstandart_at_28_03_2008', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '19', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '20', value: 'Media_Analyse_2008_zitiert_nach_derstandart_at_01_04_2009', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '20', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '21', value: 'ORF_Bericht_2012', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '21', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '22', value: 'ORF_Bericht_2022', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '22', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '23', value: 'ORF_Bericht_2023', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '23', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '24', value: 'Oesterreichische_Auflagenkontrolle_Jahresbericht_2021', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '24', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '25', value: 'Oesterreichische_Auflagenkontrolle_Jahresbericht_2022', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '25', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '26', value: 'Oesterreichische_Auflagenkontrolle_Jahresbericht_2023', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '26', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '27', value: 'OeWA_4__Quartal_2008', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '27', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '28', value: 'OeWA_4__Quartal_2009', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '28', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '29', value: 'OeWA_4__Quartal_2010', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '29', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '30', value: 'OeWA_4__Quartal_2011', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '30', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '31', value: 'OeWA_4__Quartal_2012', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '31', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '32', value: 'OeWA_4__Quartal_2013', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '32', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '33', value: 'OeWA_4__Quartal_2014', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '33', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '34', value: 'OeWA_4__Quartal_2015', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '34', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '35', value: 'OeWA_4__Quartal_2016', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '35', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '36', value: 'OeWA_4__Quartal_2017', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '36', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '37', value: 'OeWA_4__Quartal_2018', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '37', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '38', value: 'OeWA_4__Quartal_2019', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '38', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '39', value: 'OeWA_Jahresbericht_2020', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '39', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '40', value: 'OeWA_Jahresbericht_2021', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '40', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '41', value: 'OeWA_Jahresbericht_2022', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '41', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '42', value: 'Statistik_Austria__Kulturstatistik__BJ_2003', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '42', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '43', value: 'Statistik_Austria__Kulturstatistik__BJ_2004', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '43', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '44', value: 'Statistik_Austria__Kulturstatistik__BJ_2006', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '44', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '45', value: 'Statistik_Austria__Kulturstatistik__BJ_2008_2009', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '45', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '46', value: 'Statistik_Austria__Kulturstatistik__BJ_2010', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '46', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '47', value: 'Statistik_Austria__Kulturstatistik__BJ_2011', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '47', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '48', value: 'Statistik_Austria__Kulturstatistik__BJ_2016', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '48', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '49', value: 'Statistik_Austria__Kulturstatistik__BJ_2018', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '49', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '50', value: 'Statistik_Austria__Kulturstatistik__BJ_2019', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '50', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '51', value: 'Statistik_Austria__Kulturstatistik__BJ_2021', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '51', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '52', value: 'TBA_2021', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '52', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '53', value: 'TBA_2022', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '53', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);
CREATE (e:VocabularyEntry {key: '54', value: 'TBA_2023', vocab_name: 'DataSources'});
MATCH (v:Vocabulary {name: 'DataSources'}), (e:VocabularyEntry {key: '54', vocab_name: 'DataSources'}) CREATE (v)-[:HAS_ENTRY]->(e);

// Family: ORF Media Group
CREATE (f:mdsl_Family {name: 'ORF Media Group', comment: '@comment: Österreichischer Rundfunk and related outlets', created_at: datetime()});
// Outlet: Dorf tv
CREATE (o:mdsl_media_outlet {id_mo: 300110, mo_title: 'Dorf tv', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300110}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.comments = COALESCE(o.comments, '') + ' id: 300110';
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.mo_title = 'Dorf tv';
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.start_date = datetime('2010-06-22');
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.mandate = 3;
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 300110}) SET o.comments = 'KA';
// Outlet: Ö1 Campus - https://oe1.orf.at/campus
CREATE (o:mdsl_media_outlet {id_mo: 200082, mo_title: 'Ö1 Campus - https://oe1.orf.at/campus', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 200082}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.comments = COALESCE(o.comments, '') + ' id: 200082';
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.mo_title = 'Ö1 Campus - https://oe1.orf.at/campus';
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.start_date = datetime('2009-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.id_sector = 40;
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200082}) SET o.comments = 'Internetradio';
// Outlet: Ö1 Inforadio - https://oe1.orf.at/inforadio
CREATE (o:mdsl_media_outlet {id_mo: 200090, mo_title: 'Ö1 Inforadio - https://oe1.orf.at/inforadio', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 200090}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.comments = COALESCE(o.comments, '') + ' id: 200090';
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.mo_title = 'Ö1 Inforadio - https://oe1.orf.at/inforadio';
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.start_date = datetime('2003-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.end_date = datetime('2011-04-30');
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.id_sector = 40;
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200090}) SET o.comments = 'Internetradio';
// Outlet: ORF 1
CREATE (o:mdsl_media_outlet {id_mo: 300013, mo_title: 'ORF 1', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300013}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.comments = COALESCE(o.comments, '') + ' id: 300013';
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.mo_title = 'ORF 1';
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.start_date = datetime('1992-10-26');
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300013}) SET o.comments = '01.2011 - 04.2018 ORF eins';
// Outlet: ORF 2
CREATE (o:mdsl_media_outlet {id_mo: 300022, mo_title: 'ORF 2', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300022}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.comments = COALESCE(o.comments, '') + ' id: 300022';
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.mo_title = 'ORF 2';
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.start_date = datetime('1992-10-26');
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300022}) SET o.comments = 'KA';
// Outlet: ORF 2 Europe
CREATE (o:mdsl_media_outlet {id_mo: 300080, mo_title: 'ORF 2 Europe', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300080}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.comments = COALESCE(o.comments, '') + ' id: 300080';
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.mo_title = 'ORF 2 Europe';
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.start_date = datetime('2004-07-05');
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300080}) SET o.comments = 'Europaweit ausgestrahlte Sendungen von ORF2';
// Outlet: orf.at
CREATE (o:mdsl_media_outlet {id_mo: 400013, mo_title: 'orf.at', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 400013}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.comments = COALESCE(o.comments, '') + ' id: 400013';
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.mo_title = 'orf.at';
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.start_date = datetime('1997-07-24');
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.id_sector = 40;
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 400013}) SET o.comments = 'KA';
// Outlet: ORF III
CREATE (o:mdsl_media_outlet {id_mo: 300040, mo_title: 'ORF III', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300040}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.comments = COALESCE(o.comments, '') + ' id: 300040';
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.mo_title = 'ORF III';
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.start_date = datetime('2011-10-26');
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300040}) SET o.comments = 'KA';
// Outlet: ORF Radio [[Dach]]
CREATE (o:mdsl_media_outlet {id_mo: 200018, mo_title: 'ORF Radio [[Dach]]', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 200018}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.comments = COALESCE(o.comments, '') + ' id: 200018';
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.mo_title = 'ORF Radio [[Dach]]';
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.start_date = datetime('1967-10-01');
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200018}) SET o.comments = 'KA';
// Outlet: ORF Sat
CREATE (o:mdsl_media_outlet {id_mo: 300070, mo_title: 'ORF Sat', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300070}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.comments = COALESCE(o.comments, '') + ' id: 300070';
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.mo_title = 'ORF Sat';
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.start_date = datetime('1997-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.end_date = datetime('2000-12-31');
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300070}) SET o.comments = 'Europaweit ausgestrahlte Sendungen von ORF 2 und TW1';
// Outlet: ORF Sport+
CREATE (o:mdsl_media_outlet {id_mo: 300050, mo_title: 'ORF Sport+', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300050}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.comments = COALESCE(o.comments, '') + ' id: 300050';
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.mo_title = 'ORF Sport+';
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.start_date = datetime('2011-10-26');
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.mandate = 8;
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300050}) SET o.comments = '26.10.2011 SPORT+ zu einem 24-Stunden-Spartenkanal ausgebaut';
// Outlet: ORF Sport Plus
CREATE (o:mdsl_media_outlet {id_mo: 300049, mo_title: 'ORF Sport Plus', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300049}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.comments = COALESCE(o.comments, '') + ' id: 300049';
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.mo_title = 'ORF Sport Plus';
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.start_date = datetime('2006-05-01');
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.end_date = datetime('2011-10-25');
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.mandate = 8;
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300049}) SET o.comments = 'Seit Mai 2000 gab es eine eigene Sportschiene im Programm von TW1 / Bis 25.10.2011 teilte sich der Sender, der kein Vollprogramm ausstrahlte, die Frequenz mit TW1';
// Outlet: Österreichischer Rundfunk - ORF
CREATE (o:mdsl_media_outlet {id_mo: 923400, mo_title: 'Österreichischer Rundfunk - ORF', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 923400}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.comments = COALESCE(o.comments, '') + ' id: 923400';
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.mo_title = 'Österreichischer Rundfunk - ORF';
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.start_date = datetime('1955-07-27');
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.id_sector = 90;
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 923400}) SET o.comments = '19.01.1952 die öffentliche Verwaltung des Rundfunks bekommt den Namenszusatz „Österreichischer Rundfunk“, 27.07.1955  alle Radiosender offiziell zu einem österreichischen Rundfunk zusammengefasst,  11.12.1957 Gründung Österreichische Rundfunk Ges.m.b.H., die am 01.01.1958 den Hörfunk- und Fernsehbetrieb übernahm, 01.01.1967 Gründung des ORF im Wesentlichen in seiner heutigen Form (https://science.orf.at/stories/3200700/) ';
// Outlet: Österreichischer Rundfunk - ORF Fernsehen [[Dach]]
CREATE (o:mdsl_media_outlet {id_mo: 300010, mo_title: 'Österreichischer Rundfunk - ORF Fernsehen [[Dach]]', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'ORF Media Group'}), (o:mdsl_media_outlet {id_mo: 300010}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.comments = COALESCE(o.comments, '') + ' id: 300010';
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.mo_title = 'Österreichischer Rundfunk - ORF Fernsehen [[Dach]]';
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.start_date = datetime('1961-09-11');
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300010}) SET o.comments = 'KA';

// Family: Other Media Outlets
CREATE (f:mdsl_Family {name: 'Other Media Outlets', comment: '@comment: Sample of other media outlets from database', created_at: datetime()});
// Outlet: 3sat
CREATE (o:mdsl_media_outlet {id_mo: 300060, mo_title: '3sat', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300060}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.comments = COALESCE(o.comments, '') + ' id: 300060';
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.mo_title = '3sat';
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.start_date = datetime('1984-12-01');
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300060}) SET o.comments = 'Von ARD, ORF, SRG und ZDF gemeinsam betrieben';
// Outlet: ARD alpha
CREATE (o:mdsl_media_outlet {id_mo: 300501, mo_title: 'ARD alpha', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300501}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.comments = COALESCE(o.comments, '') + ' id: 300501';
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.mo_title = 'ARD alpha';
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.start_date = datetime('2014-06-29');
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300501}) SET o.comments = 'KA';
// Outlet: arte
CREATE (o:mdsl_media_outlet {id_mo: 300490, mo_title: 'arte', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300490}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.comments = COALESCE(o.comments, '') + ' id: 300490';
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.mo_title = 'arte';
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.start_date = datetime('1998-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.mandate = 2;
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300490}) SET o.comments = 'ARD (Vertretung durch den SWR), ZDF und France Télévisions, Französischer Staat, Radio France, Institut national de l’audiovisuel';
// Outlet: Blue Danube Radio
CREATE (o:mdsl_media_outlet {id_mo: 200060, mo_title: 'Blue Danube Radio', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200060}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.comments = COALESCE(o.comments, '') + ' id: 200060';
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.mo_title = 'Blue Danube Radio';
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.start_date = datetime('1979-08-23');
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.end_date = datetime('2000-01-31');
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200060}) SET o.comments = 'KA';
// Outlet: BR alpha
CREATE (o:mdsl_media_outlet {id_mo: 300601, mo_title: 'BR alpha', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300601}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.comments = COALESCE(o.comments, '') + ' id: 300601';
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.mo_title = 'BR alpha';
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.start_date = datetime('2000-06-01');
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.end_date = datetime('2014-06-28');
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300601}) SET o.comments = 'Gegründet 07.01.1998, doch alpha Österreich wird erst seit 01.06.2000 ausgestrahlt';
// Outlet: FM4
CREATE (o:mdsl_media_outlet {id_mo: 200070, mo_title: 'FM4', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200070}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.comments = COALESCE(o.comments, '') + ' id: 200070';
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.mo_title = 'FM4';
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.start_date = datetime('1995-01-16');
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200070}) SET o.comments = 'KA';
// Outlet: FS 1
CREATE (o:mdsl_media_outlet {id_mo: 300012, mo_title: 'FS 1', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300012}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.comments = COALESCE(o.comments, '') + ' id: 300012';
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.mo_title = 'FS 1';
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.start_date = datetime('1961-09-11');
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.end_date = datetime('1992-10-25');
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300012}) SET o.comments = 'KA';
// Outlet: FS 2
CREATE (o:mdsl_media_outlet {id_mo: 300021, mo_title: 'FS 2', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300021}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.comments = COALESCE(o.comments, '') + ' id: 300021';
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.mo_title = 'FS 2';
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.start_date = datetime('1961-09-11');
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.end_date = datetime('1992-10-25');
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300021}) SET o.comments = '3 mal wöchentlich, ab ???? 5 mal wöchentlich, ab 01.09.1970 täglich, ab 02.04.1989 mit Informationsangebot in Kroatisch und Serbisch und der mehrsprachigen Sendung Heimat, fremde Heimat';
// Outlet: Kurzwellendienst des Österreichischen Rundfunks
CREATE (o:mdsl_media_outlet {id_mo: 200051, mo_title: 'Kurzwellendienst des Österreichischen Rundfunks', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200051}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.comments = COALESCE(o.comments, '') + ' id: 200051';
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.mo_title = 'Kurzwellendienst des Österreichischen Rundfunks';
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.start_date = datetime('1955-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.end_date = datetime('1985-12-31');
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200051}) SET o.comments = 'KA';
// Outlet: Ö3 - Hitradio Ö3
CREATE (o:mdsl_media_outlet {id_mo: 200041, mo_title: 'Ö3 - Hitradio Ö3', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200041}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.comments = COALESCE(o.comments, '') + ' id: 200041';
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.mo_title = 'Ö3 - Hitradio Ö3';
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.start_date = datetime('1967-10-01');
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200041}) SET o.comments = 'KA';
// Outlet: Österreich 1 (Ö1)
CREATE (o:mdsl_media_outlet {id_mo: 200020, mo_title: 'Österreich 1 (Ö1)', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200020}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.comments = COALESCE(o.comments, '') + ' id: 200020';
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.mo_title = 'Österreich 1 (Ö1)';
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.start_date = datetime('1967-10-01');
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200020}) SET o.comments = 'Nachrichten in deutscher und englischer Sprache';
// Outlet: Österreich 2 (Ö2)  [[Dach]]
CREATE (o:mdsl_media_outlet {id_mo: 200036, mo_title: 'Österreich 2 (Ö2)  [[Dach]]', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200036}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.comments = COALESCE(o.comments, '') + ' id: 200036';
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.mo_title = 'Österreich 2 (Ö2)  [[Dach]]';
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.start_date = datetime('1990-05-02');
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.end_date = datetime('9999-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200036}) SET o.comments = 'Bis Mitte der 1990er Jahre bestand noch ein gemeinsames Rahmenprogramm, das nur stundenweise durch Regionalfenster unterbrochen wurde.';
// Outlet: Österreichisches Fernsehen
CREATE (o:mdsl_media_outlet {id_mo: 300011, mo_title: 'Österreichisches Fernsehen', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300011}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.comments = COALESCE(o.comments, '') + ' id: 300011';
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.mo_title = 'Österreichisches Fernsehen';
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.start_date = datetime('1955-08-01');
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.end_date = datetime('1961-09-10');
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300011}) SET o.comments = '3 mal wöchentlich, ab 01.01.1957 6 mal wöchentlich, ab 10.1959 täglich (https://der.orf.at/unternehmen/chronik/index.html)';
// Outlet: Österreich Regional
CREATE (o:mdsl_media_outlet {id_mo: 200035, mo_title: 'Österreich Regional', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200035}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.comments = COALESCE(o.comments, '') + ' id: 200035';
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.mo_title = 'Österreich Regional';
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.start_date = datetime('1967-10-01');
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.end_date = datetime('1990-05-01');
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 200035}) SET o.comments = 'Gemeinsames Rahmenprogramm der ORF-Regionalradios, das nur stundenweise durch Regionalfenster unterbrochen wurde';
// Outlet: Radio 1476
CREATE (o:mdsl_media_outlet {id_mo: 200081, mo_title: 'Radio 1476', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200081}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.comments = COALESCE(o.comments, '') + ' id: 200081';
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.mo_title = 'Radio 1476';
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.start_date = datetime('1997-03-21');
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.end_date = datetime('2008-12-31');
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200081}) SET o.comments = 'KA';
// Outlet: Radio Österreich International 
CREATE (o:mdsl_media_outlet {id_mo: 200052, mo_title: 'Radio Österreich International ', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 200052}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.comments = COALESCE(o.comments, '') + ' id: 200052';
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.mo_title = 'Radio Österreich International ';
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.start_date = datetime('1985-01-01');
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.end_date = datetime('2003-06-30');
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.id_sector = 20;
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.language = 'mehrsprachig';
MATCH (o:mdsl_media_outlet {id_mo: 200052}) SET o.comments = 'KA';
// Outlet: TW1
CREATE (o:mdsl_media_outlet {id_mo: 300030, mo_title: 'TW1', id_sector: 2, mandate: 1, location: 'Wien', primary_distr_area: 1, local: 0, language: 'deutsch', start_date: datetime('1955-01-01'), end_date: datetime('9999-01-01'), editorial_line_s: 'Öffentlich-rechtlich', comments: 'Generated from MDSL'});
MATCH (f:mdsl_Family {name: 'Other Media Outlets'}), (o:mdsl_media_outlet {id_mo: 300030}) CREATE (f)-[:mdsl_HAS_OUTLET]->(o);
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.comments = COALESCE(o.comments, '') + ' id: 300030';
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.mo_title = 'TW1';
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.start_date = datetime('1997-10-01');
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.end_date = datetime('2011-10-25');
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.id_sector = 30;
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.mandate = 1;
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.comments = COALESCE(o.comments, '') + ' distribution: complex_object';
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.language = 'deutsch';
MATCH (o:mdsl_media_outlet {id_mo: 300030}) SET o.comments = 'TW1 war zwar seit Oktober 2005 zu 100 % im Besitz des ORF (zuvor 50%) und der rechtliche Rahmen war nach § 9 des ORF-Gesetzes vorgegeben, für die Finanzierung des Senders durften jedoch keine Mittel aus den Rundfunkgebühren verwendet werden. TW1 galt daher nicht als öffentlich-rechtlicher Sender. / Die inhaltliche Nachfolge wird eher durch Sport+ erfüllt, die Frequenz bekam jedoch ORF III';

// RELATIONSHIPS
// Synchronous relationship: link_200090_main_media_outlet
MATCH (o1:mdsl_media_outlet {id_mo: 200090}), (o2:mdsl_media_outlet {id_mo: 200020}) MERGE (o1)-[r:mdsl_main_media_outlet]->(o2) SET r.start_rel = datetime('2011-04-30'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200082_main_media_outlet
MATCH (o1:mdsl_media_outlet {id_mo: 200082}), (o2:mdsl_media_outlet {id_mo: 200020}) MERGE (o1)-[r:mdsl_main_media_outlet]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300070_main_media_outlet
MATCH (o1:mdsl_media_outlet {id_mo: 300070}), (o2:mdsl_media_outlet {id_mo: 300030}) MERGE (o1)-[r:mdsl_main_media_outlet]->(o2) SET r.start_rel = datetime('2000-12-31'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300080_main_media_outlet
MATCH (o1:mdsl_media_outlet {id_mo: 300080}), (o2:mdsl_media_outlet {id_mo: 300022}) MERGE (o1)-[r:mdsl_main_media_outlet]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300070_main_media_outlet
MATCH (o1:mdsl_media_outlet {id_mo: 300070}), (o2:mdsl_media_outlet {id_mo: 300022}) MERGE (o1)-[r:mdsl_main_media_outlet]->(o2) SET r.start_rel = datetime('2008-06-07'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200081_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200081}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('2008-12-31'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200070_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200070}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200060_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200060}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('2000-01-31'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200052_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200052}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('2003-06-30'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200051_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200051}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('1985-12-31'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200041_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200041}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200036_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200036}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200035_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200035}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('1990-05-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200020_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200020}), (o2:mdsl_media_outlet {id_mo: 200018}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300050_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300050}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300049_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300049}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('2011-10-25'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300040_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300040}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300030_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300030}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('2011-10-25'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300022_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300022}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300021_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300021}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('1992-10-25'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300013_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300013}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300012_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300012}), (o2:mdsl_media_outlet {id_mo: 300010}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('1992-10-25'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200018_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 200018}), (o2:mdsl_media_outlet {id_mo: 923400}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_400013_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 400013}), (o2:mdsl_media_outlet {id_mo: 923400}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300011_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300011}), (o2:mdsl_media_outlet {id_mo: 923400}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('1961-09-10'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300010_umbrella
MATCH (o1:mdsl_media_outlet {id_mo: 300010}), (o2:mdsl_media_outlet {id_mo: 923400}) MERGE (o1)-[r:mdsl_umbrella]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300010_collaboration
MATCH (o1:mdsl_media_outlet {id_mo: 300010}), (o2:mdsl_media_outlet {id_mo: 300490}) MERGE (o1)-[r:mdsl_collaboration]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300010_collaboration
MATCH (o1:mdsl_media_outlet {id_mo: 300010}), (o2:mdsl_media_outlet {id_mo: 300601}) MERGE (o1)-[r:mdsl_collaboration]->(o2) SET r.start_rel = datetime('2014-06-28'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200052_collaboration
MATCH (o1:mdsl_media_outlet {id_mo: 200052}), (o2:mdsl_media_outlet {id_mo: 200081}) MERGE (o1)-[r:mdsl_collaboration]->(o2) SET r.start_rel = datetime('2002-12-31'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_200020_collaboration
MATCH (o1:mdsl_media_outlet {id_mo: 200020}), (o2:mdsl_media_outlet {id_mo: 200081}) MERGE (o1)-[r:mdsl_collaboration]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Synchronous relationship: link_300010_collaboration
MATCH (o1:mdsl_media_outlet {id_mo: 300010}), (o2:mdsl_media_outlet {id_mo: 300501}) MERGE (o1)-[r:mdsl_collaboration]->(o2) SET r.start_rel = datetime('9999-01-01'), r.end_rel = datetime('9999-01-01');
// Diachronic relationship: evolution_300011_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300011}), (succ:mdsl_media_outlet {id_mo: 300012}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300012_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300012}), (succ:mdsl_media_outlet {id_mo: 300013}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300021_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300021}), (succ:mdsl_media_outlet {id_mo: 300022}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300030_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300030}), (succ:mdsl_media_outlet {id_mo: 300040}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300030_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300030}), (succ:mdsl_media_outlet {id_mo: 300050}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300049_succession
MATCH (pred:mdsl_media_outlet {id_mo: 300049}), (succ:mdsl_media_outlet {id_mo: 300050}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_200035_succession
MATCH (pred:mdsl_media_outlet {id_mo: 200035}), (succ:mdsl_media_outlet {id_mo: 200036}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_200051_succession
MATCH (pred:mdsl_media_outlet {id_mo: 200051}), (succ:mdsl_media_outlet {id_mo: 200052}) MERGE (pred)-[r:mdsl_succession]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_200081_new_sector
MATCH (pred:mdsl_media_outlet {id_mo: 200081}), (succ:mdsl_media_outlet {id_mo: 200082}) MERGE (pred)-[r:mdsl_new_sector]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300070_interruption
MATCH (pred:mdsl_media_outlet {id_mo: 300070}), (succ:mdsl_media_outlet {id_mo: 300080}) MERGE (pred)-[r:mdsl_interruption]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_300010_offshoot
MATCH (pred:mdsl_media_outlet {id_mo: 300010}), (succ:mdsl_media_outlet {id_mo: 300060}) MERGE (pred)-[r:mdsl_offshoot]->(succ) SET r.event_rel = datetime('1900-01-01');
// Diachronic relationship: evolution_200060_merger
MATCH (pred:mdsl_media_outlet {id_mo: 200060}), (succ:mdsl_media_outlet {id_mo: 200070}) MERGE (pred)-[r:mdsl_merger]->(succ) SET r.event_rel = datetime('1900-01-01');
// MARKET DATA


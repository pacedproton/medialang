
// ===== CONSTRAINT CREATION =====

// Ensure unique IDs for media outlets
CREATE CONSTRAINT outlet_id_unique IF NOT EXISTS
FOR (mo:MediaOutlet) REQUIRE mo.id IS UNIQUE;

// Ensure unique relationship IDs
CREATE CONSTRAINT relationship_id_unique IF NOT EXISTS
FOR (r:Relationship) REQUIRE r.id IS UNIQUE;

// ===== NODE CREATION =====

// Create Kronen Zeitung media outlet node
CREATE (kz:MediaOutlet:Newspaper {
    id: 200001,
    title: "Kronen Zeitung",
    sector: "Tageszeitung",
    mandate: "Privat-kommerziell",
    location: "Wien",
    primary_distribution_area: "Österreich gesamt",
    local_offering: false,
    language: "de",
    start_date: date("1959-01-01"),
    start_precision: "known",
    current_status: "active",
    editorial_stance_self: "Popular journalism",
    editorial_stance_external: "Populist-leaning",
    editorial_stance_attribution: "Media Analysis, 2020",
    steward: "js",
    verified: date("2024-10-15"),
    notes: "Founded in 1900, re-established in 1959",
    created_at: datetime(),
    group: "Kronen Zeitung Family"
});

// Create Express media outlet node
CREATE (ex:MediaOutlet:Newspaper {
    id: 300001,
    title: "Express",
    sector: "Tageszeitung",
    mandate: "Privat-kommerziell",
    location: "Wien",
    primary_distribution_area: "Österreich gesamt",
    local_offering: false,
    language: "de",
    start_date: date("1950-01-01"),
    start_precision: "known",
    end_date: date("1980-12-31"),
    end_precision: "known",
    current_status: "integrated_ceased",
    editorial_stance_self: "Independent daily news",
    editorial_stance_external: "Aligned with Kronen Zeitung",
    editorial_stance_attribution: "Media Analysis, 1972",
    steward: "js",
    verified: date("2024-10-15"),
    notes: "Integrated into Kronen Zeitung after 1971 acquisition",
    created_at: datetime(),
    group: "Express Family"
});

// Create media group nodes
CREATE (kzGroup:MediaGroup {
    name: "Kronen Zeitung Family",
    description: "Includes Express post-1971 acquisition",
    created_at: datetime()
});

CREATE (exGroup:MediaGroup {
    name: "Express Family", 
    description: "Acquired by Kronen Zeitung in 1971",
    created_at: datetime()
});

// ===== RELATIONSHIP CREATION =====

// Connect outlets to their initial groups
MATCH (kz:MediaOutlet {id: 200001}), (kzGroup:MediaGroup {name: "Kronen Zeitung Family"})
CREATE (kz)-[:BELONGS_TO {
    from_date: date("1959-01-01"),
    relationship_type: "founding_member"
}]->(kzGroup);

MATCH (ex:MediaOutlet {id: 300001}), (exGroup:MediaGroup {name: "Express Family"})
CREATE (ex)-[:BELONGS_TO {
    from_date: date("1950-01-01"),
    to_date: date("1970-12-31"),
    relationship_type: "founding_member"
}]->(exGroup);

// Create acquisition relationship (diachronic)
MATCH (ex:MediaOutlet {id: 300001}), (kz:MediaOutlet {id: 200001})
CREATE (ex)-[:ACQUIRED_BY {
    event_date: date("1971-01-01"),
    event_year: 1971,
    relationship_id: 1,
    relationship_type: "Akquisition",
    details: "Express acquired by Kronen Zeitung in 1971",
    created_at: datetime()
}]->(kz);

// Create post-acquisition group membership
MATCH (ex:MediaOutlet {id: 300001}), (kzGroup:MediaGroup {name: "Kronen Zeitung Family"})
CREATE (ex)-[:BELONGS_TO {
    from_date: date("1971-01-01"),
    to_date: date("1980-12-31"),
    relationship_type: "acquired_member"
}]->(kzGroup);

// Create combination relationship (synchronous)
MATCH (kz:MediaOutlet {id: 200001}), (ex:MediaOutlet {id: 300001})
CREATE (kz)-[:COMBINED_WITH {
    relationship_id: 1,
    relationship_type: "Kombination",
    start_date: date("1972-01-01"),
    end_date: date("1980-12-31"),
    details: "Express as part of Kronen Zeitung's advertising unit",
    created_at: datetime()
}]->(ex);

// ===== MARKET DATA NODES =====

// Create market data nodes for Kronen Zeitung 2021
CREATE (kzData2021:MarketData {
    outlet_id: 200001,
    year: 2021,
    aggregation_type: "national",
    circulation: 700000,
    circulation_unit: "copies",
    circulation_source: "oeak",
    circulation_comment: "Verified",
    unique_users: 99,
    unique_users_unit: "individuals", 
    unique_users_source: "owa",
    unique_users_comment: "N/A",
    reach_national: 25.0,
    reach_national_unit: "percent",
    reach_national_source: "media_analyse",
    reach_national_comment: "Verified",
    market_share_national: 30.0,
    market_share_national_unit: "percent",
    market_share_national_source: "media_analyse",
    market_share_national_comment: "Verified",
    overall_comment: "Circulation data verified",
    created_at: datetime()
});

// Create market data nodes for Express 1972
CREATE (exData1972:MarketData {
    outlet_id: 300001,
    year: 1972,
    aggregation_type: "national",
    circulation: 45000,
    circulation_unit: "copies",
    circulation_source: "oeak",
    circulation_comment: "Post-acquisition",
    unique_users: 99,
    unique_users_unit: "individuals",
    unique_users_source: "owa", 
    unique_users_comment: "N/A",
    reach_national: 1.8,
    reach_national_unit: "percent",
    reach_national_source: "media_analyse",
    reach_national_comment: "Estimated",
    market_share_national: 1.2,
    market_share_national_unit: "percent",
    market_share_national_source: "media_analyse",
    market_share_national_comment: "Estimated",
    overall_comment: "Post-acquisition data under Kronen Zeitung",
    created_at: datetime()
});

// Connect market data to outlets
MATCH (kz:MediaOutlet {id: 200001}), (kzData:MarketData {outlet_id: 200001, year: 2021})
CREATE (kz)-[:HAS_MARKET_DATA]->(kzData);

MATCH (ex:MediaOutlet {id: 300001}), (exData:MarketData {outlet_id: 300001, year: 1972})
CREATE (ex)-[:HAS_MARKET_DATA]->(exData);

// ===== VOCABULARY AND REFERENCE NODES =====

// Create sector vocabulary
CREATE (sectorNewspaper:Sector {
    id: 1,
    name: "Tageszeitung",
    description: "Daily newspaper",
    created_at: datetime()
});

// Create mandate vocabulary  
CREATE (mandatePrivateCommercial:Mandate {
    id: 2,
    name: "Privat-kommerziell", 
    description: "Private commercial",
    created_at: datetime()
});

// Create region vocabulary
CREATE (regionAustria:Region {
    id: 10,
    name: "Österreich gesamt",
    description: "Austria total",
    created_at: datetime()
});

// Connect outlets to vocabulary
MATCH (kz:MediaOutlet {id: 200001}), (sector:Sector {id: 1})
CREATE (kz)-[:HAS_SECTOR]->(sector);

MATCH (ex:MediaOutlet {id: 300001}), (sector:Sector {id: 1})
CREATE (ex)-[:HAS_SECTOR]->(sector);

MATCH (kz:MediaOutlet {id: 200001}), (mandate:Mandate {id: 2})
CREATE (kz)-[:HAS_MANDATE]->(mandate);

MATCH (ex:MediaOutlet {id: 300001}), (mandate:Mandate {id: 2})
CREATE (ex)-[:HAS_MANDATE]->(mandate);

MATCH (kz:MediaOutlet {id: 200001}), (region:Region {id: 10})
CREATE (kz)-[:DISTRIBUTED_IN]->(region);

MATCH (ex:MediaOutlet {id: 300001}), (region:Region {id: 10})
CREATE (ex)-[:DISTRIBUTED_IN]->(region);

// ===== ANALYTICAL QUERIES =====

// Query: Find acquisition relationships
MATCH (acquired:MediaOutlet)-[r:ACQUIRED_BY]->(acquirer:MediaOutlet)
RETURN acquired.title AS acquired_outlet,
       acquirer.title AS acquiring_outlet,
       r.event_year AS acquisition_year,
       r.details AS details;

// Query: Find all outlets in Kronen Zeitung family over time
MATCH (outlet:MediaOutlet)-[r:BELONGS_TO]->(group:MediaGroup {name: "Kronen Zeitung Family"})
RETURN outlet.title AS outlet_name,
       r.from_date AS membership_start,
       r.to_date AS membership_end,
       r.relationship_type AS membership_type
ORDER BY r.from_date;

// Query: Market evolution analysis
MATCH (outlet:MediaOutlet)-[:HAS_MARKET_DATA]->(data:MarketData)
WHERE outlet.id IN [200001, 300001]
RETURN outlet.title AS outlet_name,
       data.year AS year,
       data.circulation AS circulation,
       data.reach_national AS reach_national,
       data.market_share_national AS market_share
ORDER BY data.year, outlet.title;

// Query: Find combination relationships with temporal context
MATCH (outlet1:MediaOutlet)-[r:COMBINED_WITH]->(outlet2:MediaOutlet)
RETURN outlet1.title AS primary_outlet,
       outlet2.title AS combined_outlet,
       r.start_date AS combination_start,
       r.end_date AS combination_end,
       r.details AS combination_details;

// Query: Full ownership and relationship timeline
MATCH path = (outlet:MediaOutlet)-[*1..3]-(related)
WHERE outlet.id = 200001
RETURN path; 
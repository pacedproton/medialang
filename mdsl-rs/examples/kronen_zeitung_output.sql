-- Media Data Specification Language Output
-- ===== SCHEMA CREATION =====
-- Main media outlets table
CREATE TABLE Medienangebot (
  id_mo INTEGER PRIMARY KEY,
  mo_title VARCHAR(120) NOT NULL,
  id_sector INTEGER,
  mandate VARCHAR(50),
  location VARCHAR(25),
  primary_distr_area INTEGER,
  local_offering BOOLEAN,
  language VARCHAR(50),
  start_day INTEGER,
  start_month INTEGER,
  start_year INTEGER,
  start_fake_date_type VARCHAR(50),
  end_day INTEGER,
  end_month INTEGER,
  end_year INTEGER,
  end_fake_date_type VARCHAR(50),
  current_status_text VARCHAR(50),
  editorial_line_self_descr TEXT,
  editorial_line_external_attr TEXT,
  comments TEXT,
  steward VARCHAR(10),
  verified DATE,
  notes TEXT
);
-- Diachronic relationships (acquisitions, successions)
CREATE TABLE MedienangeboteDiachroneBeziehungen (
  id_relationship INTEGER PRIMARY KEY,
  id_mo_predecessor INTEGER,
  id_mo_successor INTEGER,
  relationship_type VARCHAR(50),
  event_day INTEGER,
  event_month INTEGER,
  event_year INTEGER,
  comments TEXT,
  FOREIGN KEY (id_mo_predecessor) REFERENCES Medienangebot(id_mo),
  FOREIGN KEY (id_mo_successor) REFERENCES Medienangebot(id_mo)
);
-- Synchronous relationships (combinations, cooperations)
CREATE TABLE MedienangeboteSynchroneBeziehungen (
  id_relationship INTEGER PRIMARY KEY,
  id_mo_1 INTEGER,
  id_mo_2 INTEGER,
  relationship_type VARCHAR(50),
  start_year INTEGER,
  end_year INTEGER,
  comments TEXT,
  FOREIGN KEY (id_mo_1) REFERENCES Medienangebot(id_mo),
  FOREIGN KEY (id_mo_2) REFERENCES Medienangebot(id_mo)
);
-- Market data table
CREATE TABLE MedienangebotMarktdaten (
  id_mo INTEGER,
  year INTEGER,
  calc_additive VARCHAR(100),
  circulation INTEGER,
  circulation_source_id INTEGER,
  unique_users INTEGER,
  unique_users_source_id INTEGER,
  reach_national DECIMAL(5, 2),
  reach_national_source_id INTEGER,
  market_share_national DECIMAL(5, 2),
  market_share_national_source_id INTEGER,
  comments TEXT,
  PRIMARY KEY (id_mo, year),
  FOREIGN KEY (id_mo) REFERENCES Medienangebot(id_mo)
);
-- ===== DATA INSERTION =====
-- Insert Kronen Zeitung
INSERT INTO Medienangebot (
    id_mo,
    mo_title,
    id_sector,
    mandate,
    location,
    primary_distr_area,
    local_offering,
    language,
    start_day,
    start_month,
    start_year,
    start_fake_date_type,
    current_status_text,
    editorial_line_self_descr,
    editorial_line_external_attr,
    steward,
    verified,
    notes
  )
VALUES (
    200001,
    'Kronen Zeitung',
    1,
    'Privat-kommerziell',
    'Wien',
    10,
    false,
    'de',
    1,
    1,
    1959,
    'n.a.',
    'active',
    'Popular journalism',
    'Populist-leaning (Media Analysis, 2020)',
    'js',
    '2024-10-15',
    'Founded in 1900, re-established in 1959'
  );
-- Insert Express
INSERT INTO Medienangebot (
    id_mo,
    mo_title,
    id_sector,
    mandate,
    location,
    primary_distr_area,
    local_offering,
    language,
    start_day,
    start_month,
    start_year,
    start_fake_date_type,
    end_day,
    end_month,
    end_year,
    end_fake_date_type,
    current_status_text,
    editorial_line_self_descr,
    editorial_line_external_attr,
    steward,
    verified,
    notes
  )
VALUES (
    300001,
    'Express',
    1,
    'Privat-kommerziell',
    'Wien',
    10,
    false,
    'de',
    1,
    1,
    1950,
    'n.a.',
    31,
    12,
    1980,
    'n.a.',
    'integrated_ceased',
    'Independent daily news',
    'Aligned with Kronen Zeitung (Media Analysis, 1972)',
    'js',
    '2024-10-15',
    'Integrated into Kronen Zeitung after 1971 acquisition'
  );
-- Insert acquisition relationship
INSERT INTO MedienangeboteDiachroneBeziehungen (
    id_relationship,
    id_mo_predecessor,
    id_mo_successor,
    relationship_type,
    event_day,
    event_month,
    event_year,
    comments
  )
VALUES (
    1,
    300001,
    200001,
    'Akquisition',
    1,
    1,
    1971,
    'Express acquired by Kronen Zeitung in 1971'
  );
-- Insert combination relationship
INSERT INTO MedienangeboteSynchroneBeziehungen (
    id_relationship,
    id_mo_1,
    id_mo_2,
    relationship_type,
    start_year,
    end_year,
    comments
  )
VALUES (
    1,
    200001,
    300001,
    'Kombination',
    1972,
    1980,
    'Express as part of Kronen Zeitung''s advertising unit'
  );
-- Insert market data for Kronen Zeitung 2021
INSERT INTO MedienangebotMarktdaten (
    id_mo,
    year,
    calc_additive,
    circulation,
    circulation_source_id,
    unique_users,
    unique_users_source_id,
    reach_national,
    reach_national_source_id,
    market_share_national,
    market_share_national_source_id,
    comments
  )
VALUES (
    200001,
    2021,
    'national',
    700000,
    5,
    99,
    3,
    25.0,
    2,
    30.0,
    2,
    'Circulation data verified'
  );
-- Insert market data for Express 1972 (post-acquisition)
INSERT INTO MedienangebotMarktdaten (
    id_mo,
    year,
    calc_additive,
    circulation,
    circulation_source_id,
    unique_users,
    unique_users_source_id,
    reach_national,
    reach_national_source_id,
    market_share_national,
    market_share_national_source_id,
    comments
  )
VALUES (
    300001,
    1972,
    'national',
    45000,
    5,
    99,
    3,
    1.8,
    2,
    1.2,
    2,
    'Post-acquisition data under Kronen Zeitung'
  );
-- ===== ANALYTICAL QUERIES =====
-- Query: Find all media outlets acquired by Kronen Zeitung
SELECT mo_pred.mo_title AS acquired_outlet,
  mo_succ.mo_title AS acquiring_outlet,
  rel.event_year AS acquisition_year,
  rel.comments
FROM MedienangeboteDiachroneBeziehungen rel
  JOIN Medienangebot mo_pred ON rel.id_mo_predecessor = mo_pred.id_mo
  JOIN Medienangebot mo_succ ON rel.id_mo_successor = mo_succ.id_mo
WHERE rel.relationship_type = 'Akquisition'
  AND mo_succ.mo_title = 'Kronen Zeitung';
-- Query: Market share evolution over time
SELECT mo.mo_title,
  md.year,
  md.circulation,
  md.reach_national,
  md.market_share_national
FROM MedienangebotMarktdaten md
  JOIN Medienangebot mo ON md.id_mo = mo.id_mo
WHERE mo.id_mo IN (200001, 300001)
ORDER BY md.year,
  mo.mo_title;
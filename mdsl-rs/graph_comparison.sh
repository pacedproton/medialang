#!/bin/bash

# Graph comparison script for Original vs MDSL-generated Neo4j data
# This script queries both graphs and identifies differences

NEO4J_URL="http://100.77.115.86:7475/db/neo4j/tx/commit"

echo "=== Neo4j Graph Comparison: Original vs MDSL-generated ==="
echo

# Function to execute cypher query
execute_query() {
    local query="$1"
    local description="$2"
    
    echo "--- $description ---"
    curl -s -X POST "$NEO4J_URL" \
        -H "Content-Type: application/json" \
        -d "{\"statements\": [{\"statement\": \"$query\"}]}" | \
        python3 -m json.tool | grep -E '"row"|"columns"' || echo "Query failed"
    echo
}

# 1. Overall node counts
execute_query "MATCH (n:media_outlet) WITH count(n) as original_count MATCH (m:mdsl_media_outlet) RETURN original_count, count(m) as mdsl_count" \
    "Total node counts"

# 2. ORF-specific node counts
execute_query "MATCH (n:media_outlet) WHERE n.mo_title =~ '(?i).*orf.*' WITH count(n) as original_orf MATCH (m:mdsl_media_outlet) WHERE m.mo_title =~ '(?i).*orf.*' RETURN original_orf, count(m) as mdsl_orf" \
    "ORF-related node counts"

# 3. Node comparison by title
execute_query "MATCH (o:media_outlet) WHERE o.mo_title =~ '(?i).*orf.*' WITH collect(o.mo_title) as original_titles MATCH (m:mdsl_media_outlet) WHERE m.mo_title =~ '(?i).*orf.*' WITH original_titles, collect(m.mo_title) as mdsl_titles RETURN size(original_titles) as original_count, size(mdsl_titles) as mdsl_count, [t IN original_titles WHERE NOT t IN mdsl_titles] as only_in_original, [t IN mdsl_titles WHERE NOT t IN original_titles] as only_in_mdsl" \
    "Title comparison for ORF outlets"

# 4. Relationship type counts - Original
execute_query "MATCH (n:media_outlet)-[r]-(m:media_outlet) WHERE n.mo_title =~ '(?i).*orf.*' OR m.mo_title =~ '(?i).*orf.*' RETURN type(r) as rel_type, count(r) as count ORDER BY count DESC" \
    "Original relationship counts (ORF-related)"

# 5. Relationship type counts - MDSL
execute_query "MATCH (n:mdsl_media_outlet)-[r]-(m:mdsl_media_outlet) WHERE n.mo_title =~ '(?i).*orf.*' OR m.mo_title =~ '(?i).*orf.*' RETURN type(r) as rel_type, count(r) as count ORDER BY count DESC" \
    "MDSL relationship counts (ORF-related)"

# 6. Direct ID mapping check
execute_query "MATCH (o:media_outlet) WHERE o.mo_title =~ '(?i).*orf.*' AND o.id_mo IS NOT NULL WITH o OPTIONAL MATCH (m:mdsl_media_outlet {id_mo: o.id_mo}) RETURN o.id_mo as id, o.mo_title as original_title, m.mo_title as mdsl_title, CASE WHEN m IS NULL THEN 'MISSING' ELSE 'FOUND' END as status ORDER BY status DESC, id LIMIT 20" \
    "ID mapping check (first 20)"

# 7. Property comparison for matching nodes
execute_query "MATCH (o:media_outlet) WHERE o.mo_title = 'ORF' WITH o OPTIONAL MATCH (m:mdsl_media_outlet) WHERE m.mo_title = 'ORF' RETURN o.id_mo as original_id, m.id_mo as mdsl_id, o.mo_title as title, keys(o) as original_props, keys(m) as mdsl_props" \
    "Property comparison for 'ORF' node"

# 8. Relationship comparison for specific node
execute_query "MATCH (o:media_outlet {mo_title: 'ORF'})-[r]-(other:media_outlet) WITH type(r) as original_rel_type, count(r) as original_count OPTIONAL MATCH (m:mdsl_media_outlet {mo_title: 'ORF'})-[r2]-(other2:mdsl_media_outlet) WHERE type(r2) = 'mdsl_' + original_rel_type RETURN original_rel_type, original_count, count(r2) as mdsl_count" \
    "Relationship comparison for 'ORF' node"

# 9. Data property check
execute_query "MATCH (o:media_outlet) WHERE o.mo_title =~ '(?i).*orf.*' AND (o.auflage_2019 IS NOT NULL OR o.auflage_2020 IS NOT NULL) RETURN o.id_mo, o.mo_title, o.auflage_2019, o.auflage_2020 ORDER BY o.id_mo LIMIT 10" \
    "Original nodes with market data"

# 10. MDSL data property check
execute_query "MATCH (m:mdsl_media_outlet) WHERE m.mo_title =~ '(?i).*orf.*' AND (m.auflage_2019 IS NOT NULL OR m.auflage_2020 IS NOT NULL) RETURN m.id_mo, m.mo_title, m.auflage_2019, m.auflage_2020 ORDER BY m.id_mo LIMIT 10" \
    "MDSL nodes with market data"
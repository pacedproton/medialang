#!/bin/bash

# Graph comparison script with cleaner output
NEO4J_URL="http://100.77.115.86:7475/db/neo4j/tx/commit"

echo "=== Neo4j Graph Comparison: Original vs MDSL-generated ==="
echo

# 1. Total node counts
echo "1. Total node counts:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:media_outlet) WITH count(n) as original_count MATCH (m:mdsl_media_outlet) RETURN original_count, count(m) as mdsl_count"}]}' | \
    jq -r '.results[0].data[0].row as $row | "   Original: \($row[0]), MDSL: \($row[1])"'
echo

# 2. ORF-specific node counts
echo "2. ORF-related node counts:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:media_outlet) WHERE n.mo_title =~ \"(?i).*orf.*\" WITH count(n) as original_orf MATCH (m:mdsl_media_outlet) WHERE m.mo_title =~ \"(?i).*orf.*\" RETURN original_orf, count(m) as mdsl_orf"}]}' | \
    jq -r '.results[0].data[0].row as $row | "   Original ORF: \($row[0]), MDSL ORF: \($row[1])"'
echo

# 3. Original relationship types for ORF
echo "3. Original relationship types (ORF-related):"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:media_outlet)-[r]-(m:media_outlet) WHERE n.mo_title =~ \"(?i).*orf.*\" OR m.mo_title =~ \"(?i).*orf.*\" RETURN type(r) as rel_type, count(r) as count ORDER BY count DESC LIMIT 10"}]}' | \
    jq -r '.results[0].data[] | "   \(.row[0]): \(.row[1])"'
echo

# 4. MDSL relationship types for ORF
echo "4. MDSL relationship types (ORF-related):"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:mdsl_media_outlet)-[r]-(m:mdsl_media_outlet) WHERE n.mo_title =~ \"(?i).*orf.*\" OR m.mo_title =~ \"(?i).*orf.*\" RETURN type(r) as rel_type, count(r) as count ORDER BY count DESC LIMIT 10"}]}' | \
    jq -r '.results[0].data[] | "   \(.row[0]): \(.row[1])"'
echo

# 5. Check for specific ORF node
echo "5. Checking for main 'ORF' node:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (o:media_outlet) WHERE o.mo_title = \"ORF\" RETURN o.id_mo, o.mo_title UNION MATCH (m:mdsl_media_outlet) WHERE m.mo_title = \"ORF\" RETURN m.id_mo, m.mo_title"}]}' | \
    jq -r '.results[0].data[] | "   ID: \(.row[0]), Title: \(.row[1])"'
echo

# 6. Sample of ORF-related titles
echo "6. Sample ORF-related titles (first 5 from each):"
echo "   Original:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:media_outlet) WHERE n.mo_title =~ \"(?i).*orf.*\" RETURN n.mo_title ORDER BY n.mo_title LIMIT 5"}]}' | \
    jq -r '.results[0].data[] | "     \(.row[0])"'
echo "   MDSL:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:mdsl_media_outlet) WHERE n.mo_title =~ \"(?i).*orf.*\" RETURN n.mo_title ORDER BY n.mo_title LIMIT 5"}]}' | \
    jq -r '.results[0].data[] | "     \(.row[0])"'
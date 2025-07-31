#!/bin/bash

# Load Cypher file to Neo4j
NEO4J_URL="http://100.77.115.86:7475/db/neo4j/tx/commit"
CYPHER_FILE="orf_complete_network_new.cypher"

echo "Loading MDSL-generated Cypher to Neo4j..."

# Read the entire Cypher file
CYPHER_CONTENT=$(cat "$CYPHER_FILE")

# Create JSON payload
cat > neo4j_payload.json <<EOF
{
  "statements": [
    {
      "statement": "$CYPHER_CONTENT"
    }
  ]
}
EOF

# Send to Neo4j
curl -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d @neo4j_payload.json \
    -o neo4j_response.json

# Check response
echo "Neo4j response:"
jq . neo4j_response.json

# Clean up
rm -f neo4j_payload.json neo4j_response.json

echo "Done loading Cypher to Neo4j"
#!/bin/bash

NEO4J_URL="http://100.77.115.86:7475/db/neo4j/tx/commit"
CYPHER_FILE="orf_complete_network_new.cypher"

echo "Loading Cypher statements line by line..."

# Process each non-empty line
line_count=0
while IFS= read -r line; do
    # Skip comments and empty lines
    if [[ -n "$line" && "$line" =~ ^(CREATE|MATCH|MERGE|SET) ]]; then
        line_count=$((line_count + 1))
        echo "Loading statement $line_count: ${line:0:50}..."
        
        # Escape the line for JSON
        escaped_line=$(echo "$line" | sed 's/"/\\"/g' | sed "s/'/\\'/g")
        
        # Send to Neo4j
        curl -s -X POST "$NEO4J_URL" \
            -H "Content-Type: application/json" \
            -d "{\"statements\": [{\"statement\": \"$escaped_line\"}]}" \
            -o /tmp/neo4j_response.json
        
        # Check for errors
        if grep -q '"errors":\[{' /tmp/neo4j_response.json; then
            echo "ERROR in statement $line_count:"
            jq '.errors' /tmp/neo4j_response.json
            echo "Statement: $line"
            break
        fi
    fi
done < "$CYPHER_FILE"

echo "Loaded $line_count statements to Neo4j"

# Verify nodes were created
echo "Verifying nodes created:"
curl -s -X POST "$NEO4J_URL" \
    -H "Content-Type: application/json" \
    -d '{"statements": [{"statement": "MATCH (n:mdsl_media_outlet) RETURN count(n) as node_count"}]}' | \
    jq -r '.results[0].data[0].row[0]'
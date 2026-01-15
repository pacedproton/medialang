import React, { useEffect, useMemo, useRef, useState } from "react";
import cytoscape from "cytoscape";
import mermaid from "mermaid";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Info, Wrench } from "lucide-react";

/**
 * ANMI Ownership & Control — Comprehensive Hybrid Model
 * This version expands the canvas to cover ~80–90% of a production-ready schema:
 * - Property-graph core with legal arrangements, board control, joint control, regions/sectors/mandates
 * - Relational marts for EMFA Art.6 (ownership transparency) & Art.22 (plurality metrics)
 * - Meta-model dictionaries (entity_type, relation_type, role_type, policy_rule)
 * - Publication package & provenance/quality
 */

// ------------------------------
// 1) MERMAID — CORE ER (OPERATIONS + MARTS)
// ------------------------------
const erCore = `erDiagram
  PARTY {
    bigint party_id PK
    smallint party_type "1=LegalEntity 2=NaturalPerson"
    text display_name
    char2 country_code
    smallint founded_year
    smallint dissolved_year
  }
  LEGAL_ENTITY {
    bigint party_id PK
    smallint entity_kind "10=Media 20=Non-media"
    text company_number
    text lei
    text nace_code
    text owner_public_class
  }
  NATURAL_PERSON {
    bigint party_id PK
    smallint dob_year
    boolean public_figure_flag
  }
  MEDIA_OUTLET {
    bigint outlet_id PK
    text title
    smallint sector
    smallint mandate
    smallint primary_distr_area
  }
  PUBLIC_BODY {
    bigint body_id PK
    text name
    char2 country_code
    text gov_level
    boolean is_third_country
  }
  SOURCES {
    int source_id PK
    text title
    text citation
    smallint quality "1=self-declared 2=registry 3=audited"
    date accessed_on
  }
  DATE_PRECISION {
    smallint code PK
    text label "day month year open"
  }
  SHARE_CLASS {
    bigint company_id FK
    text class_id PK
    numeric votes_per_share
    text dividend_rights
    boolean convertible_flag
    text seniority
  }
  OUTLET_PARTY_LINK {
    bigint outlet_id FK
    bigint party_id FK
    smallint function_code "41=Provider 42=Other"
    date valid_from
    date valid_to
    smallint date_precision_start FK
    smallint date_precision_end FK
    int source_id FK
    smallint confidence
  }
  OWNERSHIP_INTEREST {
    bigint owner_id FK
    bigint owned_id FK
    text class_id FK
    smallint instrument_type
    decimal6_3 share_pct
    decimal6_3 votes_pct
    boolean direct_flag
    boolean security_interest_flag
    date valid_from
    date valid_to
    smallint date_precision_start FK
    smallint date_precision_end FK
    int source_id FK
    smallint verification
  }
  CONTROL_RELATION {
    bigint controller_id FK
    bigint controlled_id FK
    smallint control_kind "21=board_majority 22=appointment_right 23=veto_golden_share 24=shareholder_agreement 25=debt_covenant 26=variable_interest_contract 27=rights_plan 28=call_option_pref_shares 29=other"
    text basis
    boolean contingent_flag
    decimal6_3 trigger_threshold_pct
    smallint trigger_window_days
    bigint triggered_event_id FK
    date valid_from
    date valid_to
    smallint date_precision_start FK
    smallint date_precision_end FK
    int source_id FK
    smallint verification
  }
  LEGAL_ARRANGEMENT {
    bigint arrangement_id PK
    smallint la_type "1=Trust 2=Foundation 3=Other"
    text name
    date valid_from
    date valid_to
    int source_id FK
  }
  LA_ROLE {
    bigint arrangement_id FK
    bigint party_id FK
    smallint role_code "settlor trustee protector beneficiary founder council auditor"
    date valid_from
    date valid_to
    int source_id FK
  }
  BOARD_BODY {
    bigint board_id PK
    bigint company_id FK
    smallint level "1=board 2=supervisory 3=foundation_council"
    text name
  }
  BOARD_SEAT {
    bigint board_id FK
    bigint person_id FK
    smallint seat_role "member chair"
    bigint appointing_party_id FK
    date valid_from
    date valid_to
    int source_id FK
  }
  CORPORATE_EVENT {
    bigint event_id PK
    bigint from_party_id FK
    bigint to_party_id FK
    smallint event_type "61=Succession 63=Merger 64=Acquisition 65=RightsPlanTrigger 66=PrefIssue"
    date event_date
    int source_id FK
  }
  CORPORATE_ROLE {
    bigint child_id FK
    bigint parent_id FK
    smallint role_code "51..59"
    decimal6_3 stake_pct
    date valid_from
    date valid_to
    smallint date_precision_start FK
    smallint date_precision_end FK
    int source_id FK
  }
  CONTROL_GROUP {
    bigint group_id PK
    text label
    text concert_basis
    boolean mandatory_bid_implication
    date valid_from
    date valid_to
    int source_id FK
  }
  CONTROL_GROUP_MEMBER {
    bigint group_id FK
    bigint party_id FK
    smallint role "member lead"
    date valid_from
    date valid_to
  }
  CONTROL_GROUP_RELATION {
    bigint group_id FK
    bigint controlled_id FK
    smallint control_kind
    date valid_from
    date valid_to
    int source_id FK
  }
  REGION {
    smallint region_id PK
    text name
  }
  SECTOR {
    smallint sector_id PK
    text name
  }
  MANDATE {
    smallint mandate_id PK
    text name
  }
  MARKET_SHARE {
    bigint outlet_id FK
    smallint region_id FK
    smallint sector_id FK
    smallint mandate_id FK
    smallint year
    numeric9_5 share
    boolean dedup_exclusion_flag
  }
  BENEFICIAL_OWNER_SNAPSHOT {
    bigint owned_id FK
    bigint bo_person_id FK
    date as_of_date
    decimal6_3 bo_cashflow_pct
    decimal6_3 bo_votes_pct
    smallint bo_basis "1=Equity 2=Control 3=Both"
    int path_count
    text calc_version
    text fallback_reason
    text explanation
    boolean group_control_flag
  }
  BENEFICIAL_OWNER_GROUP_SNAPSHOT {
    bigint owned_id FK
    bigint control_group_id FK
    date as_of_date
    decimal6_3 eff_votes_pct
    text basis
  }
  ULTIMATE_ENTITY_SNAPSHOT {
    bigint owned_id FK
    bigint uce_entity_id FK
    date as_of_date
    smallint control_basis
    int path_count
    text calc_version
  }
  PUBLIC_AD_FUNDING {
    bigint outlet_id FK
    bigint body_id FK
    int year
    numeric14_2 amount
    int source_id FK
  }
  PUBLICATION_STATUS {
    bigint subject_id PK "party or outlet"
    smallint subject_kind "1=Party 2=Outlet"
    date last_verified_on
    date next_review_due
    smallint publication_basis "1=self-declared 2=registry-match 3=audited"
    boolean non_compliance_flag
    text notice
  }

  PARTY ||--|{ LEGAL_ENTITY : "is-a"
  PARTY ||--|{ NATURAL_PERSON : "is-a"
  LEGAL_ENTITY ||--o{ SHARE_CLASS : has
  SHARE_CLASS ||--o{ OWNERSHIP_INTEREST : share_class
  PARTY ||--o{ OWNERSHIP_INTEREST : owner
  PARTY ||--o{ OWNERSHIP_INTEREST : owned
  PARTY ||--o{ CONTROL_RELATION : controller
  PARTY ||--o{ CONTROL_RELATION : controlled
  PARTY ||--o{ CORPORATE_EVENT : "from"
  PARTY ||--o{ CORPORATE_EVENT : "to"
  PARTY ||--o{ CORPORATE_ROLE : parent_child
  MEDIA_OUTLET ||--o{ OUTLET_PARTY_LINK : links
  PUBLIC_BODY ||--o{ PUBLIC_AD_FUNDING : funds
  MEDIA_OUTLET ||--o{ PUBLIC_AD_FUNDING : receives
  SOURCES ||--o{ OUTLET_PARTY_LINK : cites
  SOURCES ||--o{ OWNERSHIP_INTEREST : cites
  SOURCES ||--o{ CONTROL_RELATION : cites
  SOURCES ||--o{ CORPORATE_EVENT : cites
  LEGAL_ARRANGEMENT ||--o{ LA_ROLE : has
  LEGAL_ENTITY ||--o{ BOARD_BODY : has
  BOARD_BODY ||--o{ BOARD_SEAT : holds
  REGION ||--o{ MARKET_SHARE : dims
  SECTOR ||--o{ MARKET_SHARE : dims
  MANDATE ||--o{ MARKET_SHARE : dims
  BENEFICIAL_OWNER_SNAPSHOT }o--|| PARTY : bo_person
  BENEFICIAL_OWNER_SNAPSHOT }o--|| PARTY : company
  BENEFICIAL_OWNER_GROUP_SNAPSHOT }o--|| CONTROL_GROUP : group
  ULTIMATE_ENTITY_SNAPSHOT }o--|| PARTY : ultimate
`;

// ------------------------------
// 2) MERMAID — META-MODEL (DICTIONARIES & POLICIES)
// ------------------------------
const erMeta = `classDiagram
  %% SOLID, STATEMENT-CENTRIC META-MODEL (BODS-aligned)
  class STATEMENT {
    +id: uuid
    +subject_id: uuid
    +predicate: text
    +object_id: uuid
    +object_value: text
    +qualifiers_json: text
    +valid_from: date
    +valid_to: date
    +source_id: int
    +asserted_by: text
    +asserted_on: date
    +confidence: smallint
    +filing_id: uuid
  }
  class FILING {
    +id: uuid
    +filer_id: uuid
    +jurisdiction: char2
    +registry: text
    +submitted_on: date
    +covers_from: date
    +covers_to: date
    +filing_type: text
  }
  class SOURCE {
    +source_id: int
    +title: text
    +citation: text
    +url: text
    +quality: smallint
    +accessed_on: date
  }
  class DATA_QUALITY {
    +quality: smallint
    +label: text
    +definition: text
  }
  class POLICY_RULE {
    +code: text
    +jurisdiction: char2
    +bo_threshold_pct: numeric
    +alt_tests_json: text
    +applies_to: text
    +effective_from: date
    +effective_to: date
  }
  class COMPUTATION_RUN {
    +run_id: uuid
    +policy_set: text
    +code_version: text
    +executed_at: datetime
  }
  class IDENTIFIER {
    +party_id: uuid
    +scheme: text
    +id_value: text
    +status: text
    +issued_by: text
  }
  class NAME {
    +party_id: uuid
    +name: text
    +type: text
    +lang: text
    +from: date
    +to: date
  }
  class CODELIST {
    +list_name: text
    +code: text
    +label: text
    +description: text
  }
  class OWNERSHIP_INTEREST_MART {
    <<derived>>
    +owner_id: uuid
    +owned_id: uuid
    +class_id: text
    +share_pct: decimal
    +votes_pct: decimal
    +valid_from: date
    +valid_to: date
    +source_id: int
  }
  class CONTROL_RELATION_MART {
    <<derived>>
    +controller_id: uuid
    +controlled_id: uuid
    +control_kind: text
    +basis: text
    +valid_from: date
    +valid_to: date
    +source_id: int
  }
  class SNAPSHOT_BO {
    <<derived>>
    +owned_id: uuid
    +bo_person_id: uuid
    +as_of_date: date
    +bo_votes_pct: decimal
    +bo_cashflow_pct: decimal
    +bo_basis: smallint
    +path_count: int
    +run_id: uuid
  }
  class SNAPSHOT_GROUP {
    <<derived>>
    +owned_id: uuid
    +control_group_id: uuid
    +as_of_date: date
    +eff_votes_pct: decimal
    +run_id: uuid
  }

  %% RELATIONSHIPS
  STATEMENT --> FILING : "part of"
  STATEMENT --> SOURCE : cites
  SOURCE --> DATA_QUALITY : has_tier
  STATEMENT ..> CODELIST : uses_codes
  POLICY_RULE ..> CODELIST : uses_codes
  COMPUTATION_RUN --> POLICY_RULE : uses
  SNAPSHOT_BO --> COMPUTATION_RUN : produced_by
  SNAPSHOT_GROUP --> COMPUTATION_RUN : produced_by
  STATEMENT --> OWNERSHIP_INTEREST_MART : feeds
  STATEMENT --> CONTROL_RELATION_MART : feeds
  IDENTIFIER --> NAME : supports
`;

// ------------------------------
// 3) CYTOSCAPE — PROPERTY GRAPH META VIEW
// ------------------------------
function buildGraphElements() {
  const nodes = [
    { data: { id: "Party", label: "🏷️ Party", type: "party" }, classes: "entity" },
    { data: { id: "LegalEntity", label: "🏢 LegalEntity\n(subtype Party)", type: "legal" }, classes: "subtype" },
    { data: { id: "NaturalPerson", label: "👤 NaturalPerson\n(subtype Party)", type: "person" }, classes: "subtype" },
    { data: { id: "MediaOutlet", label: "📰 MediaOutlet", type: "outlet" }, classes: "entity" },
    { data: { id: "PublicBody", label: "🏛️ PublicBody", type: "public" }, classes: "entity" },
    { data: { id: "LegalArrangement", label: "📜 LegalArrangement\n(Trust/Foundation)", type: "arrangement" }, classes: "entity" },
    { data: { id: "BoardBody", label: "👥 BoardBody", type: "board" }, classes: "entity" },
    { data: { id: "BoardSeat", label: "💺 BoardSeat", type: "board" }, classes: "entity" },
    { data: { id: "CorporateEvent", label: "⏲️ CorporateEvent", type: "event" }, classes: "event" },
    { data: { id: "ControlGroup", label: "🧩 ControlGroup", type: "group" }, classes: "group" },
    { data: { id: "Region", label: "🗺️ Region", type: "dim" }, classes: "dim" },
    { data: { id: "Sector", label: "📈 Sector", type: "dim" }, classes: "dim" },
    { data: { id: "Mandate", label: "🎛️ Mandate", type: "dim" }, classes: "dim" },
    { data: { id: "PolicyRule", label: "⚖️ PolicyRule", type: "govern" }, classes: "govern" },
  ];

  const edges = [
    { data: { id: "sub1", source: "LegalEntity", target: "Party", label: "is-a" }, classes: "isa" },
    { data: { id: "sub2", source: "NaturalPerson", target: "Party", label: "is-a" }, classes: "isa" },

    { data: { id: "e1", source: "Party", target: "Party", label: "OWNS\nshare_pct, votes_pct, valid_*" }, classes: "owns" },
    { data: { id: "e2", source: "Party", target: "Party", label: "CONTROLS\nboard/veto/agreement, valid_*" }, classes: "controls" },
    { data: { id: "e3", source: "Party", target: "MediaOutlet", label: "PROVIDES 41/42\nvalid_*" }, classes: "provides" },
    { data: { id: "e4", source: "PublicBody", target: "MediaOutlet", label: "FUNDED_BY_PUBLIC_ADS\nyear, amount" }, classes: "funds" },
    { data: { id: "e5", source: "CorporateEvent", target: "Party", label: "from" }, classes: "eventEdge" },
    { data: { id: "e6", source: "CorporateEvent", target: "Party", label: "to" }, classes: "eventEdge" },
    { data: { id: "e7", source: "ControlGroup", target: "Party", label: "member" }, classes: "groupEdge" },
    { data: { id: "e8", source: "ControlGroup", target: "Party", label: "JOINT_CONTROL" }, classes: "controls" },

    { data: { id: "e9", source: "LegalArrangement", target: "Party", label: "ROLE: settlor/trustee/beneficiary…\nvalid_*" }, classes: "arr" },
    { data: { id: "e10", source: "BoardBody", target: "LegalEntity", label: "of company" }, classes: "board" },
    { data: { id: "e11", source: "BoardSeat", target: "BoardBody", label: "seat in" }, classes: "board" },
    { data: { id: "e12", source: "Party", target: "BoardSeat", label: "APPOINTS" }, classes: "controls" },

    { data: { id: "e13", source: "Region", target: "MediaOutlet", label: "classifies" }, classes: "dimEdge" },
    { data: { id: "e14", source: "Sector", target: "MediaOutlet", label: "classifies" }, classes: "dimEdge" },
    { data: { id: "e15", source: "Mandate", target: "MediaOutlet", label: "classifies" }, classes: "dimEdge" },

    { data: { id: "e16", source: "PolicyRule", target: "Party", label: "applies to" }, classes: "governEdge" },
    { data: { id: "e17", source: "PolicyRule", target: "MediaOutlet", label: "applies to" }, classes: "governEdge" },
  ];

  return [...nodes, ...edges];
}

// ------------------------------
// Legend cards
// ------------------------------
const Legend = () => (
  <div className="grid grid-cols-1 xl:grid-cols-3 gap-4">
    <Card>
      <CardContent className="p-4 text-sm">
        <div className="font-semibold mb-2">Node types</div>
        <ul className="list-disc ml-5">
          <li><span className="font-mono">Party</span> supertype; subtypes: <span className="font-mono">LegalEntity</span>, <span className="font-mono">NaturalPerson</span></li>
          <li><span className="font-mono">MediaOutlet</span>, <span className="font-mono">PublicBody</span></li>
          <li><span className="font-mono">LegalArrangement</span> (trust/foundation)</li>
          <li><span className="font-mono">BoardBody</span> & <span className="font-mono">BoardSeat</span> (appointment control)</li>
          <li><span className="font-mono">ControlGroup</span> (joint control)</li>
          <li>Dims: <span className="font-mono">Region</span>, <span className="font-mono">Sector</span>, <span className="font-mono">Mandate</span></li>
          <li><span className="font-mono">PolicyRule</span> (jurisdictional thresholds & fallbacks)</li>
        </ul>
        <div className="font-semibold mt-4 mb-2">Edge types</div>
        <ul className="list-disc ml-5">
          <li><span className="font-mono">OWNS</span> — economic ownership (cash‑flow & voting %)</li>
          <li><span className="font-mono">CONTROLS</span> — non‑equity control (appointment, veto, agreements)</li>
          <li><span className="font-mono">ROLE</span> — trust/foundation roles over time</li>
          <li><span className="font-mono">PROVIDES</span> — party↔outlet functional link (41/42)</li>
          <li><span className="font-mono">FUNDED_BY_PUBLIC_ADS</span> — annual public advertising</li>
          <li><span className="font-mono">EVENT</span> — merger/acquisition/succession</li>
          <li><span className="font-mono">JOINT_CONTROL</span> via <span className="font-mono">ControlGroup</span></li>
          <li><span className="font-mono">classifies</span> — dims attached to outlets for HHI/C4</li>
          <li><span className="font-mono">applies to</span> — policy rules coverage</li>
        </ul>
        <div className="flex items-start gap-2 mt-4 opacity-80">
          <Info className="w-4 h-4 mt-1" />
          <p>Edges carry <span className="font-mono">valid_from</span>, <span className="font-mono">valid_to</span>, precision, <span className="font-mono">source_id</span>, and confidence for full provenance.</p>
        </div>
      </CardContent>
    </Card>

    <Card>
      <CardContent className="p-4 text-sm">
        <div className="font-semibold mb-2">Relational marts (EMFA Art.6 & 22)</div>
        <ul className="list-disc ml-5">
          <li><span className="font-mono">beneficial_owner_snapshot</span> — BO persons (+ fallback & explanation)</li>
          <li><span className="font-mono">ultimate_entity_snapshot</span> — ultimate controlling entity</li>
          <li><span className="font-mono">market_share</span> — region/sector/mandate/year (+ <span className="font-mono">dedup_exclusion_flag</span>)</li>
          <li><span className="font-mono">public_ad_funding</span> — annual totals (incl. third‑country bodies)</li>
          <li><span className="font-mono">publication_status</span> — last‑verified, next‑review, notices</li>
        </ul>
      </CardContent>
    </Card>

    <Card>
      <CardContent className="p-4 text-sm">
        <div className="font-semibold mb-2">Meta‑model dictionaries</div>
        <ul className="list-disc ml-5">
          <li><span className="font-mono">entity_type</span>, <span className="font-mono">relation_type</span>, <span className="font-mono">role_type</span></li>
          <li><span className="font-mono">policy_rule</span> — thresholds (25% default), SMO fallback, effective dates</li>
          <li><span className="font-mono">quality_tier</span> — evidence strength</li>
        </ul>
        <div className="flex items-center gap-2 mt-3 text-xs text-gray-600"><Wrench className="w-4 h-4"/>Use the meta‑model to add new relation/role types without DB churn.</div>
      </CardContent>
    </Card>
  </div>
);

// ------------------------------
// Cytoscape panes (enhanced rendering & controls)
// ------------------------------
function CytoscapePane({ elements, layoutName, edgeOpacity, onSelect }) {
  const containerRef = useRef(null);
  const cyRef = useRef(null);

  // init
  useEffect(() => {
    if (!containerRef.current) return;
    const cy = cytoscape({
      container: containerRef.current,
      elements,
      layout: { name: layoutName || "cose", animate: false, padding: 30 },
      style: [
        { selector: "node", style: { "background-color": "#ffffff", label: "data(label)", "text-wrap": "wrap", "text-max-width": 200, "font-size": 12, "text-valign": "center", "text-halign": "center", width: 200, height: 72, shape: "round-rectangle", "border-width": 2, "border-color": "#cbd5e1", "shadow-blur": 8, "shadow-color": "#e5e7eb", "shadow-opacity": 1, "shadow-offset-x": 0, "shadow-offset-y": 1 } },
        { selector: "node.subtype", style: { "background-color": "#ecfeff", "border-color": "#06b6d4" } },
        { selector: "node.event", style: { shape: "diamond", width: 66, height: 66, "background-color": "#fff7ed", "border-color": "#f59e0b" } },
        { selector: "node.group", style: { shape: "ellipse", width: 170, height: 86, "background-color": "#eef2ff", "border-color": "#6366f1" } },
        { selector: "node.dim", style: { "background-color": "#fefce8", "border-color": "#a3a3a3" } },
        { selector: "node.govern", style: { "background-color": "#f0fdf4", "border-color": "#16a34a" } },
        { selector: "edge", style: { width: 2.5, opacity: edgeOpacity ?? 1, "curve-style": "bezier", "line-color": "#94a3b8", "target-arrow-color": "#94a3b8", "target-arrow-shape": "triangle", label: "data(label)", "font-size": 11, "text-background-color": "#ffffff", "text-background-opacity": 0.85, "text-background-padding": 2, "arrow-scale": 1 } },
        { selector: "edge.isa", style: { "line-style": "dashed", "line-color": "#a8a29e" } },
        { selector: "edge.owns", style: { "line-color": "#10b981", "target-arrow-color": "#10b981", width: 3 } },
        { selector: "edge.controls", style: { "line-color": "#ef4444", "target-arrow-color": "#ef4444", width: 3 } },
        { selector: "edge.provides", style: { "line-color": "#3b82f6", "target-arrow-color": "#3b82f6" } },
        { selector: "edge.funds", style: { "line-color": "#a855f7", "target-arrow-color": "#a855f7" } },
        { selector: "edge.arr", style: { "line-color": "#f59e0b", "target-arrow-color": "#f59e0b" } },
        { selector: "edge.board", style: { "line-color": "#8b5cf6", "target-arrow-color": "#8b5cf6" } },
        { selector: "edge.dimEdge", style: { "line-color": "#a3a3a3", "line-style": "dotted", "target-arrow-color": "#a3a3a3" } },
        { selector: "edge.governEdge", style: { "line-color": "#16a34a", "target-arrow-color": "#16a34a", "line-style": "dashed" } },
        { selector: ".faded", style: { opacity: 0.08 } },
        { selector: ".highlight", style: { "border-color": "#111827", "border-width": 3 } },
      ],
    });

    // selection & neighborhood highlight
    const clearHighlights = () => {
      cy.elements().removeClass("faded highlight");
    };
    cy.on("tap", (evt) => {
      if (evt.target === cy) {
        clearHighlights();
        onSelect?.(null);
      }
    });
    cy.on("tap", "node", (evt) => {
      const node = evt.target;
      const neighborhood = node.closedNeighborhood();
      cy.elements().addClass("faded");
      neighborhood.removeClass("faded");
      node.addClass("highlight");
      onSelect?.({ id: node.id(), label: node.data("label"), type: node.data("type") });
    });

    cyRef.current = cy;
    return () => { cy.destroy(); };
  }, [elements, layoutName]);

  // react to edgeOpacity changes
  useEffect(() => {
    if (!cyRef.current) return;
    cyRef.current.style().selector('edge').style('opacity', edgeOpacity ?? 1).update();
  }, [edgeOpacity]);

  // re-run layout when layoutName changes
  useEffect(() => {
    if (!cyRef.current) return;
    cyRef.current.layout({ name: layoutName || 'cose', animate: true, padding: 30 }).run();
  }, [layoutName]);

  // export helpers
  const exportPng = () => {
    if (!cyRef.current) return;
    const png = cyRef.current.png({ bg: '#ffffff', full: true, scale: 2 });
    const a = document.createElement('a');
    a.href = png;
    a.download = 'anmi-ownership-graph.png';
    a.click();
  };

  return (
    <div className="space-y-3">
      <div ref={containerRef} className="w-full h-[600px] rounded-2xl border border-gray-200" />
      <div className="flex flex-wrap gap-2">
        <Button variant="outline" onClick={() => cyRef.current && cyRef.current.fit()}>Fit to view</Button>
        <Button variant="outline" onClick={() => cyRef.current && cyRef.current.reset()}>Reset pan/zoom</Button>
        <Button variant="outline" onClick={exportPng}>Export PNG</Button>
      </div>
    </div>
  );
}

function MermaidPane({ text, caption }) {
  const ref = useRef(null);
  const [error, setError] = useState(null);

  useEffect(() => {
    mermaid.initialize({ startOnLoad: false, securityLevel: "loose", theme: "default" });
    const el = ref.current;
    if (!el) return;
    const render = async () => {
      try {
        const { svg } = await mermaid.render(`er_${Date.now()}`, text);
        el.innerHTML = svg;
      } catch (e) {
        setError(String(e));
      }
    };
    render();
  }, [text]);

  return (
    <div className="space-y-3">
      {error && <div className="text-red-600 text-sm">Mermaid render error: {error}</div>}
      <div ref={ref} className="w-full h-[560px] overflow-auto rounded-2xl border border-gray-200 p-4 bg-white" />
      <div className="text-xs text-gray-500">{caption}</div>
    </div>
  );
}

export default function OwnershipModelVisualizer() {
  const elements = useMemo(() => buildGraphElements(), []);
  const [layoutName, setLayoutName] = useState("cose");
  const [edgeOpacity, setEdgeOpacity] = useState(0.9);
  const [selected, setSelected] = useState(null);

  return (
    <div className="p-6 space-y-6">
      <div>
        <h1 className="text-2xl font-semibold">ANMI Ownership & Control — Comprehensive Hybrid Model</h1>
        <p className="text-gray-600 mt-1">Property‑graph core + ER marts + meta‑model dictionaries, including trusts/foundations, board/appointment control, joint control, and EMFA reporting dimensions.</p>
      </div>

      <Tabs defaultValue="graph" className="w-full">
        <TabsList>
          <TabsTrigger value="graph">Graph core</TabsTrigger>
          <TabsTrigger value="er-core">ER — core & marts</TabsTrigger>
          <TabsTrigger value="er-meta">ER — meta‑model</TabsTrigger>
          <TabsTrigger value="legend">Legend</TabsTrigger>
        </TabsList>

        <TabsContent value="graph" className="mt-4">
          <div className="grid grid-cols-1 xl:grid-cols-4 gap-4">
            <div className="xl:col-span-3 space-y-4">
              <Card>
                <CardContent className="p-4 flex flex-wrap items-center gap-3 text-sm">
                  <div className="flex items-center gap-2">
                    <span className="text-gray-600">Layout</span>
                    <select className="border rounded-md px-2 py-1" value={layoutName} onChange={(e) => setLayoutName(e.target.value)}>
                      <option value="cose">CoSE (force)</option>
                      <option value="breadthfirst">Breadthfirst</option>
                      <option value="concentric">Concentric</option>
                      <option value="grid">Grid</option>
                    </select>
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-gray-600">Edge opacity</span>
                    <input type="range" min={0.1} max={1} step={0.05} value={edgeOpacity} onChange={(e)=>setEdgeOpacity(parseFloat(e.target.value))} />
                    <span className="tabular-nums w-10 text-right">{edgeOpacity.toFixed(2)}</span>
                  </div>
                </CardContent>
              </Card>
              <CytoscapePane elements={elements} layoutName={layoutName} edgeOpacity={edgeOpacity} onSelect={setSelected} />
            </div>
            <div>
              <Card>
                <CardContent className="p-4 space-y-3">
                  <div className="font-semibold">Details</div>
                  {selected ? (
                    <div className="text-sm">
                      <div className="font-mono text-xs text-gray-500 mb-1">id: {selected.id}</div>
                      <div className="text-base">{selected.label}</div>
                      <div className="text-gray-600 mt-2">type: <span className="font-mono">{selected.type || 'n/a'}</span></div>
                      <div className="text-gray-500 text-xs mt-3">Neighborhood is highlighted; click on canvas background to clear.</div>
                    </div>
                  ) : (
                    <div className="text-sm text-gray-600">Tap a node to inspect it. The node’s neighborhood is highlighted and everything else fades, to help you reason about control paths.</div>
                  )}
                </CardContent>
              </Card>
            </div>
          </div>
        </TabsContent>

        <TabsContent value="er-core" className="mt-4"><MermaidPane text={erCore} caption="Core operational tables and reporting marts for EMFA Art.6/22." /></TabsContent>
        <TabsContent value="er-meta" className="mt-4"><MermaidPane text={erMeta} caption="Schema‑as‑data: entity/relation/role dictionaries and policy rules." /></TabsContent>
        <TabsContent value="legend" className="mt-4"><Legend /></TabsContent>
      </Tabs>

      <div className="text-xs text-gray-500 leading-relaxed">
        Tip: Use Breadthfirst or Concentric layout to emphasise ownership levels; CoSE to see communities. Export a PNG from the Graph tab.
      </div>
    </div>
  );
}

% ANMI-ML: A Reproducible Digital Memory Infrastructure for Modeling Austrian Media Landscapes
% Michael Alexander, Postdoctoral Researcher, Austrian Academy of Sciences (OeAW), Institute for Comparative Media and Communication Studies
% 2026-01-12

## Abstract

Research on Austrian memory culture and memory politics increasingly depends on technical infrastructures that determine what can be documented, queried, verified, and publicly communicated. This paper presents **ANMI-ML**, a reproducible digital memory infrastructure that combines (i) a domain-specific language (DSL) and compiler toolchain for representing media outlets and their temporal relations, and (ii) a hybrid data model that integrates a property-graph core with relational reporting marts and a statement-centric meta-model for provenance and policy. We argue that treating data modeling and compilation as infrastructural work helps bridge a persistent gap between interpretive memory studies and the operational requirements of auditable, publishable knowledge about media institutions. We describe ANMI-ML’s compilation pipeline (lexing, parsing, semantic analysis, intermediate representation, and code generation to SQL/Cypher), and detail a schema that encodes ownership, control, legal arrangements, governance, and evidence quality with explicit temporal validity. We evaluate ANMI-ML via a contract-oriented validation workflow that makes system behavior testable and reproducible across components. We conclude with implications for memory studies: how infrastructure choices shape epistemic visibility, and how provenance-rich representations enable critique without collapsing into purely technical accounts.

**Keywords:** memory infrastructure; provenance; temporal modeling; media landscape; domain-specific languages; graphs; reproducibility

## 1. Introduction

In memory studies, the “archive” is not merely a container of traces but a set of practices, institutions, and material infrastructures that shape what can be remembered, contested, and legitimized. As Austrian memory culture and memory politics are increasingly mediated through data-intensive systems (registries, institutional databases, public portals, journalistic datasets), the technical architectures used to represent actors and relations become consequential. They can enable certain kinds of inquiry (longitudinal change, accountability claims, cross-institution comparison) while obscuring others.

This paper presents **ANMI-ML**, a digital memory infrastructure project that treats **data modeling and compilation** as first-class scholarly objects. ANMI-ML was designed to represent media outlets, organizations, and temporal relationships in a form that is (a) human-readable and versionable, (b) machine-validated, and (c) compilable into multiple backends (relational and graph). The project’s immediate motivation is to operationalize an end-to-end pipeline from a structured source database into a language-based representation suitable for verification and downstream publication and analysis.

We make three contributions:

1. **A language-based representation for media landscapes**: a DSL with constructs for entities, relationships, and temporal events that can be validated and curated over time.
2. **A reproducible compiler toolchain**: a modular pipeline (lexer → parser → semantic analysis → IR → code generation) that produces database artifacts (SQL and Cypher) from a single source representation.
3. **A hybrid model emphasizing temporality and provenance**: a schema that encodes ownership and control, legal arrangements, evidence quality tiers, temporal validity, and publication/verification status, designed to support auditable outputs for research and public accountability.

The remainder of the paper situates ANMI-ML in related work on memory infrastructures (Section 2), describes the system architecture (Section 3), details the data model (Section 4) and implementation choices (Section 5), and evaluates the system through contract-based reproducibility criteria and a short vignette (Section 6). Section 7 discusses limitations and ethics; Section 8 concludes.

## 2. Background and related work

### 2.1 Memory studies and infrastructure

Memory scholarship has long emphasized that memory is made through institutions and material supports, not only through individual recollection. Cultural-memory research highlights the role of media, archives, and institutionalized practices in stabilizing and transmitting memory across generations [@assmann2011].

Contemporary work in critical archival studies, platform studies, and critical data studies extends this insight: infrastructures, standards, and metadata regimes shape what can be said, verified, and circulated about the past. Rather than treating databases and information systems as neutral carriers, infrastructural approaches study their categories, omissions, and governance as part of memory politics.

### 2.2 Provenance, evidence, and auditability

A recurring challenge in public-facing memory infrastructures is **credibility**: what counts as evidence, how competing claims are represented, and how uncertainties are expressed without collapsing into “anything goes.” Provenance modeling and quality tiers (e.g., self-declared vs registry vs audited sources) provide a structured way to represent evidential status while retaining the possibility of critique and revision.

### 2.3 Graph and relational representations for cultural data

Graph representations (knowledge graphs, property graphs) are widely used to model heterogeneous entities and relationships, especially when lineage, temporality, and multiple relation types matter. Relational schemas remain central for reporting and aggregation, particularly where standardized indicators are required. Hybrid approaches that combine graph modeling with relational marts are increasingly common in data-intensive domains, but remain under-theorized in memory studies as infrastructures with epistemic effects [@angles2008].

### 2.4 DSLs and reproducible data infrastructures

DSLs are widely used to encode domain knowledge in a form that is both human-meaningful and machine-checkable. In research settings, DSLs can function as “infrastructure in text,” enabling version control, review, and reproducible compilation to multiple outputs. ANMI-ML builds on standard compiler design patterns to support validation and backend generation, extending practices of reproducible research to the level of data-model semantics.

## 3. System overview: ANMI-ML as a digital memory infrastructure

ANMI-ML is organized around three layers:

1. **Source import and normalization**: a database import contract specifies how structured source records are transformed into a stable, human-readable language representation (MDSL).
2. **Language toolchain**: MDSL files are processed by a Rust compiler pipeline that validates syntax and semantics and transforms content into a normalized intermediate representation.
3. **Backend generation and publication views**: code generation emits graph and relational artifacts (Cypher and SQL), supporting both exploratory network analysis and standardized reporting.

Figure 1 sketches the pipeline.

**Figure 1.** ANMI-ML pipeline: source database import, language-based representation, validation, and compilation to backends.

```text
SourceDB
  -> sql_import (extract/normalize)
  -> MDSL (versioned text representation)
  -> mdsl validate (syntax/semantic checks)
  -> codegen: Cypher (graph) and SQL (relational)
  -> analysis/publication views (queries, marts, exports)
```

This pipeline is designed to make infrastructural decisions explicit and testable: what is considered an entity, what counts as a relationship, how temporal validity is represented, and how evidence is attached. In this sense, ANMI-ML is not only an engineering artifact but an object for memory-infrastructure critique: it encodes commitments about categories, provenance, and publishability.

## 4. Data model: hybrid graph core, relational marts, and a provenance meta-model

ANMI-ML’s modeling choices are illustrated in the project’s hybrid schema artifact, which combines an ER view with a property-graph view. The model is designed to represent entities (persons, organizations, outlets, public bodies), relationships (ownership, control, governance roles), temporal validity, and evidence quality.

### 4.1 Core entities and identification

At the core is a `PARTY` supertype with `LEGAL_ENTITY` and `NATURAL_PERSON` subtypes, supporting identity management across organizational and personal actors. Media outlets are represented as `MEDIA_OUTLET`, with dimensions such as sector and mandate.

This approach supports both interpretive and operational needs: it makes category choices explicit (e.g., what qualifies as an outlet vs an organization), while also enabling consistent joins and queries across representations.

### 4.2 Ownership and control as distinct relations

ANMI-ML distinguishes **ownership interests** (economic and voting percentages; share classes; direct/indirect flags) from **control relations** (board appointment rights, veto rights, shareholder agreements, debt covenants, options, etc.). Separating these relations is important for memory-infrastructure work: public narratives about media influence can conflate ownership with control; infrastructure should allow researchers to analyze those differences.

### 4.3 Legal arrangements, governance, and joint control

The model includes `LEGAL_ARRANGEMENT` (e.g., trust/foundation) and time-bounded roles (`LA_ROLE`) to represent governance structures that can be salient in public accountability debates. It also includes `BOARD_BODY` and `BOARD_SEAT` to model appointment control, and `CONTROL_GROUP` structures for joint control.

### 4.4 Temporality, precision, and provenance

To support longitudinal research and historically sensitive claims, relationships and roles are time-bounded (`valid_from`, `valid_to`) and associated with **date precision** (day/month/year/open). Each assertion is tied to a `SOURCE` with a quality tier and to confidence/verification fields.

This is a key memory-infrastructure point: temporal ambiguity and evidential uncertainty are not merely “data quality problems” but epistemic conditions of public history. A model that makes them explicit allows both critique and careful publication.

### 4.5 Reporting marts and publication status

The hybrid model includes relational marts that support standardized reporting views: snapshots of beneficial ownership, ultimate controlling entities, market share dimensions, public advertising funding, and a `PUBLICATION_STATUS` view that encodes verification and review cycles. These views serve the infrastructural goal of publishability: moving from raw, heterogeneous assertions to curated, reviewable outputs.

**Figure 2.** Hybrid model concept: graph core (entities/relations), relational marts (snapshots and metrics), and meta-model (statements, sources, policy rules).

```text
GraphCore(entities, relations, temporal_validity, provenance_refs)
  -> RelationalMarts(snapshots, metrics, publication_status)
  -> MetaModel(statements, filings, sources, policy_rules, computation_runs)
```

### 4.6 Summary table: components and research uses

**Table 1.** How ANMI-ML components support memory-infrastructure research practices.

| Component                     | Encodes                                   | Enables                                                       |
| ----------------------------- | ----------------------------------------- | ------------------------------------------------------------- |
| Graph core (property graph)   | heterogeneous entities and relation types | network reasoning, control-path exploration, change over time |
| Relational marts              | standardized snapshots/metrics            | reporting, comparative indicators, publication workflows      |
| Statement/meta-model          | assertions + provenance + confidence      | auditability, competing claims, evidence critique             |
| Temporal validity + precision | bounded intervals + uncertainty           | historically sensitive querying and interpretation            |

## 5. Implementation: compiler toolchain and validation contracts

### 5.1 Compiler pipeline

ANMI-ML’s toolchain follows a standard compiler architecture implemented in Rust: lexical analysis, parsing to an AST, semantic analysis (symbol tables, type checking, validation), transformation to an intermediate representation, and code generation. This pipeline supports modular extension: adding new constructs or backends can be done without rewriting the entire system.

### 5.2 Code generation targets

From the IR, ANMI-ML generates:

- **SQL**: relational schema artifacts and inserts where appropriate.
- **Cypher**: graph artifacts for Neo4j-style property graphs.

The explicit goal is to keep the **language representation** as the single source of truth, and treat backends as derived outputs.

### 5.3 Contract-oriented reproducibility

A notable design feature is an explicit interface contract between import, validation, and downstream components. The contract specifies required inputs, expected outputs, and validation behavior (e.g., the import tool produces syntactically valid outlet declarations, validation must pass with zero errors, and generated graph artifacts must create expected node types). This contract supports reproducibility across teams and components: it externalizes “what correct means” into testable requirements.

### 5.4 Language constructs for temporality and events

Beyond static entity definitions, ANMI-ML’s DSL supports temporal modeling patterns required for longitudinal inquiry. In particular, an `EVENT` construct can represent acquisitions, mergers, ownership transfers, or other occurrences that reconfigure relationships. An event-centered representation is useful for memory-infrastructure work because it supports historically situated narratives (what changed, when, and with what documented basis), rather than only end-state snapshots.

```text
EVENT example_event {
  type = "acquisition";
  date = "YYYY-MM-DD" | CURRENT;
  entities = { /* participants and roles */ };
  impact = { /* optional structured effects */ };
  metadata = { /* optional descriptive fields */ };
}
```

## 6. Evaluation: reproducibility criteria and a small vignette

Because ANMI-ML is an infrastructure project, evaluation focuses on **reproducibility and auditability** rather than predictive accuracy.

### 6.1 Reproducibility criteria

We evaluate ANMI-ML against four criteria:

1. **Syntactic correctness**: language artifacts validate with zero errors.
2. **Semantic integrity**: references resolve; temporal fields are well-formed; required identifiers exist.
3. **Deterministic compilation**: identical inputs yield identical derived outputs under the same toolchain version.
4. **Contract compliance across components**: import, validation, and code generation satisfy interface requirements.

### 6.2 Contract invariants as evaluation evidence

Table 2 summarizes the type of invariants used to evaluate ANMI-ML as infrastructure. These are not domain-truth claims (e.g., that a specific ownership percentage is correct), but system-behavior guarantees that make downstream scholarship more auditable.

**Table 2.** Examples of contract-style invariants used to evaluate ANMI-ML as infrastructure.

| Invariant                                   | Purpose for reproducibility and auditability                                                   |
| ------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| Import produces valid outlet declarations   | Ensures the system yields a reviewable, versionable representation rather than ad hoc exports. |
| Validation is explicit and non-silent       | Prevents hidden fallback behavior; failure is detectable and actionable.                       |
| Code generation creates expected node types | Enables stable downstream queries and visualization assumptions (e.g., outlet nodes exist).    |
| Sources and confidence fields are carried   | Supports evidential critique and selective publication.                                        |

### 6.2 Vignette: reasoning about control paths

To illustrate the scholarly use of the hybrid model, consider the task of tracing how governance and control arrangements may affect the visibility of media institutions in public memory debates. Ownership percentages alone may not explain influence; appointment rights, veto mechanisms, and legal arrangements can be decisive. ANMI-ML’s separation of ownership from control, plus time-bounded roles and provenance, allows researchers to ask historically sensitive questions such as:

- What changes in control relations coincide with shifts in outlet status or restructuring events?
- Where do legal arrangements introduce governance roles that are not visible in equity ownership tables?
- Which claims rely on self-declared sources versus registry/audited sources, and how does that affect interpretability?

This vignette emphasizes infrastructure as method: the system enables inquiry by making categories, temporality, and evidence explicit.

## 7. Discussion: limitations, ethics, and implications for memory studies

### 7.1 Limitations

ANMI-ML does not solve all problems of memory-infrastructure work. Data access may be restricted, source records may be incomplete or inconsistent, and representational choices can introduce biases (e.g., what counts as an outlet, which relationship types are modeled). Moreover, derived backends can diverge in their expressive affordances: some queries are natural in graph form, others in relational marts.

### 7.2 Ethics and responsible publication

Publishing structured claims about persons and organizations raises ethical questions: privacy, defamation risk, and the reification of categories. ANMI-ML’s provenance and confidence modeling is not a substitute for ethical review; it is an infrastructural support that makes evidence and uncertainty explicit. Any public release should include governance: review cycles, notices, and mechanisms for correction.

### 7.3 Implications

For memory studies, ANMI-ML suggests a practical synthesis: infrastructures can be both **objects of critique** and **instruments for critique**, provided they carry provenance, temporality, and auditable workflows. Treating compilation pipelines as research infrastructure helps operationalize transparency without surrendering interpretive nuance.

### 7.4 Threats to validity and scope conditions

The paper’s argument is methodological and infrastructural. It does not claim that any particular outlet relationship is true by virtue of being representable. Instead, ANMI-ML provides a framework for expressing and validating representations of claims. As such, there are at least three scope conditions:

1. The system’s epistemic value depends on data access and governance (what sources are available and permissible).
2. The adequacy of categories (e.g., outlet, party, control relation) is historically and politically situated and must remain open to revision.
3. Reproducibility of compilation does not imply reproducibility of interpretation, which remains contingent on scholarly context and contestation.

## 8. Conclusion

ANMI-ML demonstrates how a language-based, contract-validated toolchain and a hybrid model (graph + relational marts + provenance meta-model) can function as a reproducible digital memory infrastructure for representing Austrian media landscapes. By making categories, temporality, and evidence explicit, ANMI-ML supports both operational publication workflows and critical inquiry into how infrastructures shape what becomes knowable about institutions central to memory culture and politics.

## Acknowledgements

This paper describes an infrastructure artifact developed within an iterative human–LLM collaboration workflow; any remaining errors are the author’s.

## References

This Markdown manuscript uses citation keys intended for `paper/references.bib`.

## Appendix A. Claims-to-evidence matrix (repository traceability)

The paper makes conservative claims anchored in the repository’s artifacts. This appendix maps claims to evidence locations to support auditability and future revision.

| Claim (paper)                                                                                                | Evidence type              | Repository anchor                                                                       |
| ------------------------------------------------------------------------------------------------------------ | -------------------------- | --------------------------------------------------------------------------------------- |
| ANMI-ML includes a compiler pipeline (lexer → parser → semantic → IR → codegen)                              | Architecture documentation | `README.md`, `mdsl-rs/docu/project_structure.md`, `mdsl-rs/README-LLM-COMPREHENSIVE.md` |
| Contract-based interface specifies import/validation/codegen expectations                                    | Interface contract         | `anmi-mdsl-interface.md`                                                                |
| Hybrid model includes ownership, control, legal arrangements, temporality, provenance, and publication views | Schema/model artifact      | `anmi_ownership_model_graph_er.jsx`                                                     |
| Evaluation emphasizes reproducibility (validation, deterministic compilation, contract compliance)           | Methodological choice      | `anmi-mdsl-interface.md` (validation expectations)                                      |

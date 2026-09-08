# Activity Catalog (raw research) — stages, activities, artifacts

Date: 2026-09-07. Purpose: raw, sourced catalog of software-development stages and activities/techniques to seed the default redkiln-rs process catalog. Evidence discipline: every row is tagged `[documented]` (a source page was fetched in this session and the activity is named/described there) or `[inferred]` (well-known practice attributed to its usual origin, not fetched here). Lineage URLs for inferred rows are the canonical home of the practice and should be spot-checked before shipping.

## 0. Conventions

- Work-item kinds: `init` (initiative), `proj` (project), `story`, `bug`, `inc` (incident), `spike`, `chore`, `debt` (tech-debt), `vuln` (security-vuln), `docs`. `*` = all kinds.
- Cost tier: S (minutes–an hour of agent time, one artifact), M (hours, several artifacts or tool runs), L (day+, multi-party, or human-heavy).
- Execution: `A` = agent-executable from artifacts alone; `T` = agent-executable but needs tooling/environment access (test runner, scanner, prod telemetry, deploy pipeline); `H` = needs a human (decision authority, external people, physical/legal accountability). Combinations like `A+H` mean agent drafts, human gates.
- Column order in every table: id | purpose | inputs | outputs | lineage | kinds | cost | exec | evidence.
- "Ordered options" = alternatives at increasing rigor; the orchestrator picks ONE per situation. "Unordered" = pick several as needed.

Sources fetched this session (documented base): SWEBOK v4 topic list (computer.org), DA process-goal overview (pmi.org), ISO/IEC/IEEE 12207 + 29119 overviews (wikipedia), ISTQB CTFL v4.0 syllabus contents (istqb.org), NIST SSDF project page (csrc.nist.gov), OWASP Threat Modeling Process / ASVS / SAMM (owasp.org, owaspsamm.org), Google SRE book ch.12 Effective Troubleshooting, ch.15 Postmortem Culture, ch.27 Reliable Product Launches, ch.32 PRR; SRE Workbook ch.2 Implementing SLOs + Postmortem Culture (sre.google), PagerDuty response docs "During an Incident" + "Postmortem Process" (response.pagerduty.com), DORA capability catalog (dora.dev), Principles of Chaos (principlesofchaos.org), WCAG-EM 1.0 (w3.org), Shape Up ch.3 Set Boundaries (basecamp.com), Opportunity Solution Trees (producttalk.org), Fowler refactoring catalog / Parallel Change / Canary Release / Blue-Green / Feature Toggles / TDD (refactoring.com, martinfowler.com), ADR org (adr.github.io), C4 model (c4model.com), Team Topologies key concepts (teamtopologies.com), EventStorming (eventstorming.com), Google eng-practices code review (google.github.io), CycloneDX SBOM guides (cyclonedx.org), Diátaxis (diataxis.fr), Atlassian blameless postmortem (atlassian.com). Not reachable this session: PMI per-goal DA pages (timeouts), Axelos ITIL 4 practice guides (cookie wall), IEEE 29148 (wiki 404) — those rows are `[inferred]`.

## 1. Stage model

The proposed 12 stages survive contact with the sources, with three adjustments the sources argue for:

1. **Triage** should include *intake* (DA "Intake Work" goal; ITIL "service desk"/categorization) and, for incidents, the *declare/severity* step (PagerDuty severity levels) — it is not only "which bucket".
2. **Analysis/Investigation** and **Discovery** are distinct: discovery is outward-facing (users, market, opportunities: Torres, Cagan, Shape Up "raw idea → appetite"), investigation is inward-facing (repro, root cause: SRE ch.12, Agans). Bugs/incidents skip discovery; initiatives/projects usually skip investigation.
3. **Operate/Observe** must contain the *incident-response* loop (restore service first: PagerDuty IC/deputy/scribe; ITIL incident management) and hand off to Closeout for the postmortem. Incident work items therefore *start* at triage → operate (mitigate) → analysis (root cause) → implement (fix) → closeout (postmortem), i.e. stages are not strictly linear per kind.

| # | Stage | Goal | Entry condition | Exit artifact(s) | Typical kinds |
|---|---|---|---|---|---|
| 1 | triage | Decide whether/when/where the item is worked and with what appetite | An intake record exists (raw idea, ticket, alert, CVE, request) | TriageDecision (kind, severity/priority, owner, appetite, next stage) | * |
| 2 | discovery | Understand the problem/opportunity space and the users before committing to a solution | TriageDecision routes to discovery; problem is not yet framed | ProblemStatement, OpportunityMap, DiscoveryReport | init, proj, spike |
| 3 | analysis | Establish facts: reproduce, localize, explain, size | A report/symptom or a framed problem needs evidence | ReproCase, RootCauseHypothesis/RootCause, AnalysisReport, ImpactAssessment | bug, inc, vuln, spike, debt, proj |
| 4 | specification | State what will be built and how we will know it is done | Problem framed (discovery) or cause known (analysis) | Spec (stories+AC), Requirements, Contracts, DefinitionOfDone | init, proj, story, bug |
| 5 | design | Decide how it will be built and record the decisions | A spec exists with agreed scope | DesignDoc, ADR set, ThreatModel, DataMigrationPlan, UXDesign | init, proj, story (light), debt |
| 6 | planning | Decompose into shippable slices, order them, size, and de-risk | Design decisions sufficient to slice | Plan (slice DAG), Estimate/Appetite, RiskRegister, TestStrategy, RolloutPlan | init, proj, spike |
| 7 | implement | Produce working, tested, documented change | A planned slice with AC and a DoD | Changeset (code+tests+docs), ImplementationReport | story, bug, inc(fix), debt, vuln, chore, docs |
| 8 | verify | Demonstrate the change meets AC, NFRs and does not regress | Changeset exists | TestReport, CoverageReport, ScanReport, VerificationVerdict | story, bug, proj, vuln |
| 9 | review | Independent judgement on correctness, quality, safety, readiness | Verified changeset or artifact awaiting approval | ReviewVerdict (+ findings), ApprovalRecord | * |
| 10 | release | Get the change into production safely and reversibly | Approved changeset + RolloutPlan | ReleaseRecord, DeploymentLog, ReleaseNotes, ChangeRecord | proj, story, bug, vuln, inc |
| 11 | operate | Keep the service within SLO; detect and mitigate incidents; learn from telemetry | Change is live, or an alert fired | IncidentRecord, MitigationRecord, SLOReport, Runbook | inc, proj, init, chore |
| 12 | closeout | Learn, harvest knowledge, tidy, and formally close | Outcome delivered, incident resolved, or spike time-box ended | Postmortem/Retrospective, ActionItems, ClosureRecord, KB atoms | * |

Cross-cutting activities (risk, security, docs, metrics) appear inside the stage where they are executed, but DA and SSDF both treat them as "ongoing goals"; the orchestrator should allow them to be re-entered from any stage.

---

## 2. Stage catalogs

### 2.1 Triage

Goal: decide *whether*, *when*, *how big* and *by whom*. Entry: intake record. Exit: TriageDecision.

Ordered options (pick one): `quick-triage` < `structured-triage` < `triage-committee`. Unordered (pick several): the rest.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| intake-classify | Classify an incoming item into a work-item kind and domain area | IntakeRecord | TriageDecision(kind) | DA "Intake Work" process goal, https://www.pmi.org/disciplined-agile/process/introduction-to-dad/process-goals ; ITIL 4 incident mgmt categorization | * | S | A | documented (DA goal) / inferred (ITIL) |
| quick-triage | One-pass accept/reject/defer with a severity and next stage | IntakeRecord | TriageDecision | Lean/Kanban intake (Anderson), https://kanban.university | * | S | A+H | inferred |
| structured-triage | Score against explicit criteria (impact, urgency, reach, risk) | IntakeRecord, TriageRubric | TriageDecision, TriageScore | ITIL 4 impact×urgency priority matrix; RICE (Intercom) | * | S | A+H | inferred |
| triage-committee | Periodic human review of contested or large items | TriageQueue | TriageDecision set | Bug-triage meeting practice (Mozilla/Chromium); DA "Govern Team" | init, proj, vuln | M | H | inferred |
| duplicate-detect | Find prior/duplicate items and link them | IntakeRecord, Backlog | DuplicateLink | Issue-tracker practice (Bugzilla "duplicate of") | bug, inc, vuln | S | A | inferred |
| reproducibility-check | Confirm a report reproduces before investing | BugReport | ReproStatus | Agans, *Debugging* rule "Make it fail", https://www.debuggingrules.com/ | bug, vuln | S | T | documented (site) / inferred (rules text) |
| severity-assess | Assign severity from user impact and blast radius | IncidentSignal or BugReport | SeverityRating | PagerDuty severity levels, https://response.pagerduty.com/before/severity_levels/ ; SRE postmortem triggers, https://sre.google/sre-book/postmortem-culture/ | bug, inc, vuln | S | A+H | documented |
| incident-declare | Declare a major incident, page IC, open comms channel | IncidentSignal, SeverityRating | IncidentRecord(open) | PagerDuty "During an Incident", https://response.pagerduty.com/during/during_an_incident/ | inc | S | T+H | documented |
| vuln-score | Score a vulnerability (CVSS/EPSS, reachability) and set a remediation SLA | VulnReport, SBOM | VulnAssessment | NIST SSDF RV.1/RV.2, https://csrc.nist.gov/Projects/ssdf ; FIRST CVSS/EPSS | vuln | S | A+T | documented (SSDF) / inferred (CVSS) |
| priority-rank | Order accepted items (WSJF, MoSCoW, cost of delay) | TriageDecision set, Backlog | PrioritizedBacklog | SAFe WSJF; DSDM MoSCoW; Reinertsen cost of delay | init, proj, story, debt | S | A+H | inferred |
| appetite-set | Fix a time budget ("small batch"/"big batch") before designing | RawIdea | Appetite | Shape Up ch.3 "Set Boundaries", https://basecamp.com/shapeup/1.2-chapter-03 | init, proj, spike | S | H (A drafts) | documented |
| ownership-assign | Route to the owning team/agent pool per team topology | TriageDecision, TeamMap | Assignment | Team Topologies stream-aligned/platform teams, https://teamtopologies.com/key-concepts | * | S | A | documented |
| readiness-check | Verify Definition of Ready before pulling into a stage | Item + stage DoR | ReadinessVerdict | Scrum "Definition of Ready" (community) ; DA "Intake Work" | story, bug | S | A | inferred |

### 2.2 Discovery

Goal: understand problem/opportunity and users; produce a framed problem, not a solution. Entry: routed item lacking a ProblemStatement. Exit: ProblemStatement + DiscoveryReport (+ OpportunityMap for initiatives).

Ordered options (increasing rigor, pick one as the spine): `problem-framing` < `assumption-mapping` < `opportunity-solution-tree` < `design-sprint`. Unordered: research techniques feeding the spine.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| problem-framing | Write the problem, who has it, why now, what success looks like | RawIdea, Appetite | ProblemStatement | Shape Up "narrow down the problem"; Cagan *Inspired* opportunity assessment | init, proj, spike | S | A+H | documented (Shape Up) |
| stakeholder-map | Identify stakeholders, concerns, decision rights | ProblemStatement | StakeholderMap | SWEBOK v4 ch.1 §2.1 Requirements Sources; ch.2 §1.2 Stakeholders and Concerns, https://www.computer.org/education/bodies-of-knowledge/software-engineering/topics | init, proj | S | A | documented |
| outcome-definition | Set the measurable business/product outcome at the root of discovery | ProblemStatement, Metrics | OutcomeStatement | Torres OST "outcome at the top", https://www.producttalk.org/opportunity-solution-tree/ | init, proj | S | A+H | documented |
| opportunity-solution-tree | Map outcome → opportunities → solutions → assumption tests | OutcomeStatement, InterviewNotes | OpportunityMap | Torres, https://www.producttalk.org/opportunity-solution-tree/ | init, proj | M | A+H | documented |
| user-interview | Elicit needs and stories directly from users | InterviewGuide, Persona | InterviewNotes | Torres continuous interviewing; SWEBOK ch.1 §2.2 elicitation techniques | init, proj | M | H (A drafts guide, synthesizes) | documented (SWEBOK) |
| jtbd-analysis | Frame demand as jobs, forces, and desired outcomes | InterviewNotes | JobStory set | Christensen/Ulwick JTBD (inferred) | init, proj | S | A | inferred |
| persona-definition | Describe archetypal users to anchor scope | InterviewNotes, Analytics | Persona | DA Explore Scope "Explore Usage: persona" (goal page partial), https://www.pmi.org/disciplined-agile/process/introduction-to-dad/process-goals | init, proj | S | A | documented |
| competitor-analysis | Compare alternatives/competitors on capability, UX, pricing | ProblemStatement | CompetitorAnalysis | UX research canon (Nielsen Norman competitive usability) | init, proj, spike | M | A (web) | inferred |
| market-landscape-scan | Survey standards, libraries, vendors, prior art | ProblemStatement | LandscapeReport | Spike practice (XP); SWEBOK ch.15 economics "make/buy/reuse" | init, spike | M | A (web) | inferred |
| analytics-review | Mine product analytics/support tickets for evidence | Telemetry, SupportTickets | EvidenceSummary | Torres "can you create opportunities from support tickets/analytics" | init, proj | S | T | documented |
| impact-mapping | Goal → actors → impacts → deliverables tree | OutcomeStatement | ImpactMap | Adzic *Impact Mapping*; DA Explore Scope "Impact map" | init, proj | S | A+H | documented (DA) |
| assumption-mapping | List and rank riskiest assumptions (desirable/viable/feasible/usable) | OpportunityMap or ProblemStatement | AssumptionList | Torres assumption testing; Bland (inferred) | init, proj, spike | S | A | documented (OST) |
| user-story-mapping | Arrange activities/steps/stories along the user journey | Persona, InterviewNotes | StoryMap | Patton *User Story Mapping*; DA Explore Scope "User story map" | init, proj | M | A+H | documented (DA) |
| big-picture-event-storming | Workshop the whole domain flow as events to find boundaries and hotspots | StakeholderMap | EventStormBoard, DomainGlossary | Brandolini EventStorming, https://www.eventstorming.com/ ; DA Explore the Domain "Event storming" | init, proj | L | H (A synthesizes) | documented |
| value-proposition-canvas | Fit customer jobs/pains/gains to product | Persona | ValuePropositionCanvas | Osterwalder; DA Explore Purpose "Value proposition canvas" | init | S | A | documented (DA) |
| breadboarding | Sketch places/affordances/connections without visual design | ProblemStatement, Appetite | Breadboard | Shape Up ch.4 (inferred, chapter not fetched) | proj | S | A | inferred |
| fat-marker-sketch | Low-fidelity visual to test a concept | ProblemStatement | ConceptSketch | Shape Up ch.4 (inferred) | proj | S | A | inferred |
| prototype-test | Build throwaway prototype to test desirability/usability | ConceptSketch | PrototypeFindings | SWEBOK ch.1 §5.3 Prototyping; Torres solution tests | init, proj, spike | M | A+H | documented (SWEBOK) |
| design-sprint | Five-day map/sketch/decide/prototype/test cycle | ProblemStatement | PrototypeFindings, DecisionRecord | GV Design Sprint (Knapp); DA "Design sprint" | init | L | H | documented (DA name) |
| rabbit-hole-scan | Enumerate unknowns/risks in the idea before pitching | Breadboard/ConceptSketch | RiskList (discovery) | Shape Up ch.5 "Risks and Rabbit Holes" (inferred) | proj | S | A | inferred |
| discovery-report | Synthesize evidence into a go/no-go recommendation | all discovery artifacts | DiscoveryReport | DA "Develop Common Vision" goal | init, proj, spike | S | A | documented (goal) |

### 2.3 Analysis / Investigation

Goal: establish facts (repro, cause, extent, feasibility). Entry: symptom or framed problem needing evidence. Exit: ReproCase + RootCause (bugs/incidents) or AnalysisReport/FeasibilityReport (others).

Ordered options for defect localization (pick one as spine): `hypothesis-troubleshooting` < `differential-debugging` < `bisect` < `instrumented-trace-analysis`. Ordered options for causal analysis: `five-whys` < `fishbone` < `fault-tree`. Unordered: everything else.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| minimal-repro | Reduce the failing case to the smallest deterministic reproduction | BugReport, Logs | ReproCase | Agans rule "Make it fail" + SSCCE practice; delta debugging (Zeller) | bug, vuln, inc | S–M | T | inferred |
| hypothesis-troubleshooting | Iterate hypothesize → test → refine using telemetry (hypothetico-deductive) | ReproCase, Telemetry, SystemKnowledge | RootCauseHypothesis | Google SRE ch.12 Effective Troubleshooting, https://sre.google/sre-book/effective-troubleshooting/ | bug, inc | M | T | documented |
| agans-rules-pass | Apply the 9 rules (understand the system, make it fail, quit thinking and look, divide and conquer, change one thing at a time, keep an audit trail, check the plug, get a fresh view, if you didn't fix it it ain't fixed) as a checklist | ReproCase | DebugLog, RootCauseHypothesis | Agans *Debugging*, https://www.debuggingrules.com/ | bug, inc | S | A | documented (site) / inferred (list) |
| differential-debugging | Compare working vs failing configuration/inputs/versions to isolate the delta | ReproCase, KnownGoodBaseline | DeltaFindings | Agans "divide and conquer"; SRE ch.12 "what changed" | bug, inc | M | T | inferred |
| bisect | Binary-search commit history to find the introducing change | ReproCase, VCS history | CulpritCommit | `git bisect` (git docs) | bug | M | T | inferred |
| instrumented-trace-analysis | Add/collect logs, traces, metrics; analyze to localize fault | ReproCase, Telemetry | TraceAnalysis | SRE ch.12 "Examine"; OpenTelemetry (inferred) | bug, inc | M | T | documented (SRE) |
| log-analysis | Mine logs for error patterns, timing, correlation | Logs | LogFindings | SRE ch.12 | bug, inc | S | T | documented |
| change-correlation | Correlate onset with deploys, config, infra, traffic changes | Telemetry, DeploymentLog | ChangeCorrelation | PagerDuty IC "identify obvious cause (recent deployment...)", https://response.pagerduty.com/during/during_an_incident/ ; SRE ch.12 | inc, bug | S | T | documented |
| profiling | Measure CPU/memory/IO hotspots | ReproCase or Workload | ProfileReport | Performance engineering canon (Gregg *Systems Performance*) | bug, debt, story | M | T | inferred |
| memory-analysis | Heap/leak/dump inspection | Dump, ProfileReport | MemoryFindings | PagerDuty repair actions "gather forensic data (heap dumps)" | bug, inc | M | T | documented |
| timeline-reconstruction | Build the incident/defect timeline with evidence links | ChatLog, Alerts, DeploymentLog | IncidentTimeline | PagerDuty postmortem "timeline should be the main focus", https://response.pagerduty.com/after/post_mortem_process/ | inc | M | A+T | documented |
| five-whys | Iteratively ask why to reach a systemic cause | IncidentTimeline or Symptom | CausalChain | Toyota/Ohno; Atlassian postmortem guide | bug, inc | S | A | inferred |
| fishbone-analysis | Categorize contributing causes (people/process/tools/env/data) | IncidentTimeline | CauseCategoryMap | Ishikawa diagram (inferred) | inc, debt | S | A | inferred |
| fault-tree-analysis | Top-down boolean tree of failure conditions | SystemModel, Symptom | FaultTree | Reliability engineering (FTA, NUREG-0492) | inc, init(safety) | M | A | inferred |
| root-cause-confirmation | Prove the cause by reproducing and then removing it ("if you didn't fix it it ain't fixed") | RootCauseHypothesis, ReproCase | RootCause | Agans rule 9; SRE ch.12 "Treat" | bug, inc | S | T | inferred/documented |
| impact-assessment | Quantify affected users, data, revenue, duration | RootCause, Telemetry | ImpactAssessment | SRE postmortem impact section; ITIL impact×urgency | bug, inc, vuln | S | A+T | documented (SRE) |
| exploitability-analysis | Determine reachability/exploitability of a vulnerability in context | VulnReport, SBOM, Code | VulnAssessment | SSDF RV.1 "identify and confirm vulnerabilities"; OWASP threat modeling ranking | vuln | M | A+T | documented |
| blast-radius-analysis | Determine which components/tenants/data a change or fault can touch | Changeset or RootCause, ArchitectureModel | BlastRadius | SRE ch.27 launch; Team Topologies flow (inferred pairing) | inc, story, proj | S | A | inferred |
| dependency-impact-analysis | Find callers/consumers affected by a proposed change | Code, API contracts | ImpactAssessment | SWEBOK ch.7 Software Maintenance "impact analysis" | story, debt, proj | S | A+T | documented (SWEBOK topic) |
| code-archaeology | Read history/blame/ADRs to learn why code is as it is | Code, VCS, ADRs | ContextNotes | SRE ch.12 "understanding the system"; Feathers legacy code (inferred) | bug, debt, story | S | A | inferred |
| domain-analysis | Build a conceptual/domain model of the problem area | DiscoveryReport, Glossary | DomainModel | SWEBOK ch.1 §3 Requirements Analysis; DA "Explore the Domain: domain/conceptual model" | init, proj | M | A | documented |
| requirements-conflict-analysis | Detect conflicting/ambiguous requirements and negotiate | Requirements | ConflictLog, ResolvedRequirements | SWEBOK ch.1 §3.4 Addressing Conflict | init, proj | S | A+H | documented |
| feasibility-study | Assess technical/economic feasibility of candidate approaches | ProblemStatement, LandscapeReport | FeasibilityReport | SWEBOK ch.15 Software Engineering Economics; RUP Inception | init, spike | M | A | documented (topic) / inferred |
| technical-spike | Time-boxed experiment to answer one question with code | SpikeQuestion, Appetite | SpikeReport | XP spike solution; DA "Prove Architecture Early" | spike | M | T | documented (goal) |
| data-quality-analysis | Profile data for shape, nulls, drift before migration/feature | Dataset | DataProfile | Data engineering practice (inferred) | proj, story(data) | M | T | inferred |
| static-analysis-investigation | Run analyzers to locate defect classes related to the symptom | Code | ScanFindings | SWEBOK ch.12 Quality "static analysis"; ISTQB static testing | bug, vuln, debt | S | T | documented (topics) |
| tech-debt-assessment | Quantify debt items, interest, and remediation options | Code, ScanFindings | DebtRegisterEntry | DA "Improve Quality"; SWEBOK ch.7 | debt | S | A | documented (goal) |
| analysis-report | Synthesize findings and recommend next stage | all analysis artifacts | AnalysisReport | (structural) | * | S | A | inferred |

### 2.4 Specification

Goal: state what will be built and the evidence of done. Entry: ProblemStatement or RootCause. Exit: Spec with AC (+ Requirements set, Contracts, DoD).

Ordered options for the primary spec form (pick one): `user-story-ac` < `structured-nl-requirements` < `bdd-scenarios` < `use-case-spec` < `model-based-spec`. Unordered: NFR/quality, security, privacy, accessibility, contracts, validation.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| user-story-ac | Write stories with acceptance-criteria-based requirements | ProblemStatement/StoryMap | Spec(Story, AC) | SWEBOK ch.1 §4.3 Acceptance Criteria-Based Specification; DA Explore Usage "User story" | proj, story, bug | S | A | documented |
| structured-nl-requirements | Requirements in a constrained template (EARS/shall statements) with attributes | ProblemStatement | RequirementsSet | SWEBOK ch.1 §4.2 Structured NL; IEEE 29148 (inferred) | init, proj | M | A | documented (SWEBOK) |
| bdd-scenarios | Given/When/Then executable examples as spec | Spec(Story) | GherkinScenarios | ISTQB CTFL v4 "Collaboration-based test approaches" (BDD/ATDD), https://www.istqb.org/certifications/certified-tester-foundation-level ; North/Adzic (inferred) | story, bug | S | A | documented (ISTQB) |
| use-case-spec | Actor/goal/flows/exceptions narrative | StakeholderMap | UseCaseSpec | DA Explore Usage "Use case", UML use case diagram; RUP | proj | M | A | documented (DA) |
| model-based-spec | State machines/sequence/data models as the spec | DomainModel | SpecModel | SWEBOK ch.1 §4.4 Model-Based Specification | proj, story(complex) | M | A | documented |
| acceptance-criteria-refinement | Make AC testable, enumerated, traceable (AC-###) | Spec draft | Spec(AC final) | SWEBOK ch.1 §4.5 additional attributes; §7.3 tracing | story, bug | S | A | documented |
| example-mapping | Rules/examples/questions workshop to sharpen a story | Spec draft | ExampleMap, OpenQuestions | Wynne example mapping (inferred) | story | S | A+H | inferred |
| nfr-quality-scenarios | Specify quality attributes as stimulus/response scenarios (latency, availability, etc.) | ProblemStatement | QualityAttributeScenarios | SWEBOK ch.1 §1.5/§1.7 QoS constraints; SEI ATAM scenarios (inferred) | init, proj | M | A | documented |
| slo-definition | Define SLIs/SLOs and error budget for the capability | QualityAttributeScenarios, Telemetry | SLOSpec | SRE Workbook ch.2 Implementing SLOs, https://sre.google/workbook/implementing-slos/ | init, proj | M | A+H | documented |
| glossary-ubiquitous-language | Fix domain terms | DomainModel | Glossary | DDD ubiquitous language (Evans); DA "Glossary" | init, proj | S | A | documented (DA) |
| api-contract-spec | Specify interfaces (OpenAPI/AsyncAPI/proto) before implementation | DesignDoc or Spec | APIContract | Consumer-driven contracts / OpenAPI (inferred) | proj, story | M | A | inferred |
| data-contract-spec | Specify schemas, invariants, retention, ownership for data | DomainModel | DataContract | Data-mesh data contracts (inferred) | proj, story(data) | M | A | inferred |
| security-requirements | Derive security requirements from ASVS level and threat model | ThreatModel or Spec | SecurityRequirements | OWASP ASVS 5.0, https://owasp.org/www-project-application-security-verification-standard/ ; SAMM "Security Requirements"; SSDF PW.1 | init, proj, story, vuln | M | A | documented |
| abuse-case-spec | Write misuse/abuser stories | Spec, ThreatModel | AbuseCases | OWASP threat modeling "abuser stories, misuse cases", https://owasp.org/www-community/Threat_Modeling_Process | proj, story | S | A | documented |
| privacy-requirements | Identify personal data, lawful basis, minimization; screen for DPIA | DataContract, Spec | PrivacyRequirements, DPIAScreening | GDPR art.35 DPIA (ICO guidance, inferred) | init, proj | M | A+H | inferred |
| accessibility-requirements | Set WCAG conformance target and component-level requirements | Spec, UXDesign | AccessibilityRequirements | WCAG 2.x (W3C); WCAG-EM scope step, https://www.w3.org/TR/WCAG-EM/ | proj, story(UI) | S | A | documented |
| requirements-prioritization | Rank requirements (MoSCoW/Kano/WSJF) and set cut lines | RequirementsSet, Appetite | PrioritizedRequirements | SWEBOK ch.1 §7.2 Requirements Prioritization | init, proj | S | A+H | documented |
| requirements-review | Inspect requirements for completeness, consistency, testability | RequirementsSet | ReviewFindings | SWEBOK ch.1 §5.1 Requirements Reviews; ISTQB static testing | init, proj | S | A | documented |
| requirements-tracing | Link requirements ↔ stories ↔ tests | RequirementsSet, Spec | TraceabilityMatrix | SWEBOK ch.1 §7.3 Requirements Tracing | init, proj | S | A | documented |
| requirements-scrubbing | Remove/merge redundant or unneeded requirements | RequirementsSet | RequirementsSet' | SWEBOK ch.1 §6.1 Requirements Scrubbing | init, proj | S | A | documented |
| pitch-writing | Shape Up pitch: problem, appetite, solution, rabbit holes, no-gos | Breadboard, Appetite, RiskList | Pitch | Shape Up ch.6 "Write the Pitch" (inferred chapter) | proj | S | A | inferred |
| definition-of-done | Tailor the DoD checklist for this item (tests, docs, review, flags, telemetry) | OrgDoD, Spec | DefinitionOfDone | Scrum guide DoD; DA "Produce a Potentially Consumable Solution" | proj, story | S | A | documented (goal) |
| spike-question-spec | State the question, hypothesis, time-box and decision criteria | ProblemStatement | SpikeCharter | XP spike; Shape Up appetite | spike | S | A | inferred |
| fix-spec | For a bug: expected behaviour, regression test intent, scope of fix | RootCause | Spec(Fix) | (structural; ISTQB defect management) | bug, vuln | S | A | inferred |

### 2.5 Design

Goal: decide how; record decisions. Entry: agreed spec. Exit: DesignDoc + ADRs (+ ThreatModel, DataMigrationPlan, UXDesign as applicable).

Ordered options for architecture description (pick one level): `c4-context` < `c4-container` < `c4-component` < `full-architecture-views`. Ordered options for evaluation: `design-review-lightweight` < `architecture-tradeoff-analysis`. Unordered: everything else.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| c4-context | System-context diagram: system, users, external systems | Spec, StakeholderMap | C4ContextDiagram | C4 model, https://c4model.com/ | init, proj | S | A | documented |
| c4-container | Container diagram: deployable units and their interactions | C4ContextDiagram | C4ContainerDiagram | C4 model | proj | S | A | documented |
| c4-component | Component diagram inside a container | C4ContainerDiagram | C4ComponentDiagram | C4 model | proj, story(complex) | S | A | documented |
| full-architecture-views | Multiple views/viewpoints (logical, process, deployment) per a framework | Spec | ArchitectureDescription | SWEBOK ch.2 §2.1 views/viewpoints; ISO 42010 (inferred) | init | L | A | documented (SWEBOK) |
| architecture-options | Enumerate 2–3 candidate architectures with trade-offs | Spec, QualityAttributeScenarios | OptionsAnalysis | SWEBOK ch.2 §3.2 architecture synthesis/evaluation; DA "Identify Architecture Strategy" | init, proj | M | A | documented |
| adr-record | Record a significant decision, context, options, consequences | OptionsAnalysis | ADR | Nygard ADR / adr.github.io, https://adr.github.io/ | init, proj, story, debt | S | A (H accepts) | documented |
| architecture-tradeoff-analysis | Scenario-based evaluation of the architecture against quality attributes | ArchitectureDescription, QualityAttributeScenarios | ArchitectureEvaluation | SEI ATAM (inferred); SWEBOK ch.2 §4 Architecture Evaluation | init | L | A+H | documented (SWEBOK) |
| context-mapping | Identify bounded contexts and their relationships (ACL, conformist, shared kernel...) | DomainModel, EventStormBoard | ContextMap | Evans DDD context map (inferred); DA "Explore the Domain" | init, proj | M | A | inferred |
| design-level-event-storming | Model commands/aggregates/policies for a slice | EventStormBoard, Spec | AggregateDesign | Brandolini design-level EventStorming, https://www.eventstorming.com/ | proj, story | M | A+H | documented |
| domain-model-design | Entities, value objects, aggregates, invariants | DomainModel, Glossary | DomainDesign | Evans DDD tactical patterns (inferred) | proj, story | M | A | inferred |
| api-design | Design resources/operations/errors/versioning; write contract | Spec, DomainDesign | APIContract, APIDesignNotes | Zimmermann et al. *Patterns for API Design* (cited on adr.github.io) | proj, story | M | A | documented (mention) / inferred |
| api-design-review | Review contract for consistency, compatibility, style | APIContract | ReviewFindings | Google API design guide / API review boards (inferred) | proj, story | S | A | inferred |
| data-model-design | Schema/index/partition design and ownership | DataContract, DomainDesign | DataModel | SWEBOK ch.3 §3.3 Data Persistence | proj, story | M | A | documented |
| data-migration-plan | Plan expand/migrate/contract phases, backfill, dual-write, verification | DataModel, current schema | DataMigrationPlan | Fowler/Sato Parallel Change, https://martinfowler.com/bliki/ParallelChange.html ; DORA "Database change management", https://dora.dev/capabilities/ | proj, story, debt | M | A | documented |
| threat-model | Scope (DFD, entry points, trust levels) → STRIDE threats → countermeasures → assess | C4ContainerDiagram, DataContract | ThreatModel | OWASP Threat Modeling Process, https://owasp.org/www-community/Threat_Modeling_Process ; SAMM "Threat Assessment"; SSDF PW.1 | init, proj, story(security-relevant) | M | A (H accepts risk) | documented |
| security-architecture-review | Check design against secure-architecture practices | ArchitectureDescription, ThreatModel | ReviewFindings | SAMM "Secure Architecture"/"Architecture Assessment", https://owaspsamm.org/model/ | init, proj | M | A | documented |
| privacy-by-design | Data flow mapping, minimization, retention, DPIA if screened in | PrivacyRequirements, DataModel | DPIA, PrivacyDesign | GDPR art.25/35 (inferred) | init, proj | M | A+H | inferred |
| ux-flow-design | User flows and wireframes | Spec, Persona | UXDesign(Flows, Wireframes) | SWEBOK ch.3 §5.5 User-Centered Design; DA "Explore Usage: usage scenario" | proj, story(UI) | M | A | documented |
| interaction-spec | Detailed states, errors, empty/loading, copy | UXDesign | InteractionSpec | UX practice (inferred) | story(UI) | S | A | inferred |
| accessibility-design-review | Check flows/wireframes against WCAG before build | UXDesign, AccessibilityRequirements | ReviewFindings | WCAG 2.x; WCAG-EM "useful during earlier design stages", https://www.w3.org/TR/WCAG-EM/ | proj, story(UI) | S | A | documented |
| performance-model | Estimate load, latency budget, capacity, cost; identify hot paths | QualityAttributeScenarios, Telemetry | CapacityPlan | SRE ch.27 launch capacity; performance engineering (inferred) | init, proj | M | A | documented (SRE) |
| failure-mode-analysis | FMEA-style table of failure modes, effects, detection, mitigation | ArchitectureDescription | FailureModeTable | FMEA (inferred); SRE "Addressing Cascading Failures" ch.22 | init, proj | M | A | inferred |
| resilience-pattern-selection | Choose timeouts, retries, circuit breakers, bulkheads, idempotency | FailureModeTable | DesignDoc(section) | Nygard *Release It!* (inferred); Principles of Chaos systemic weaknesses list, https://principlesofchaos.org/ | proj, story | S | A | documented (chaos) / inferred |
| observability-design | Decide logs/metrics/traces/SLIs to emit and dashboards | SLOSpec, DesignDoc | ObservabilityPlan | SRE Workbook ch.2; DORA "Monitoring and observability" | proj, story | S | A | documented |
| sequence-and-state-modeling | Sequence diagrams / state machines for tricky behaviour | Spec, DomainDesign | BehaviorModel | SWEBOK ch.3 §4.3 Behavioral Design Descriptions | story, proj | S | A | documented |
| design-doc | Write the design document tying the above together with rationale | all design artifacts | DesignDoc | SWEBOK ch.3 §4.6 Design Rationale; Google design docs (inferred) | init, proj, story(complex) | M | A | documented (SWEBOK) |
| design-review-lightweight | Peer review of the design doc | DesignDoc | ReviewVerdict | SWEBOK ch.3 §6.1 Design Reviews and Audits | proj, story | S | A+H | documented |
| walking-skeleton-plan | Choose the thinnest end-to-end path to prove the architecture early | ArchitectureDescription | SkeletonPlan | Cockburn walking skeleton (inferred); DA "Prove Architecture Early" | init, proj | S | A | documented (goal) |
| deprecation-design | Plan sunset of old API/feature: notices, timelines, telemetry | APIContract | DeprecationPlan | API lifecycle practice (inferred) | proj, debt | S | A | inferred |
| fix-design | For a bug: choose fix location, guard against recurrence, decide on workaround vs fix | RootCause, Spec(Fix) | FixDesign | (structural) | bug, vuln, inc | S | A | inferred |

### 2.6 Planning / Decomposition

Goal: slice, order, size, de-risk. Entry: enough design to slice. Exit: Plan (DAG of slices), Estimate/Appetite reconciliation, RiskRegister, TestStrategy, RolloutPlan.

Ordered options for sizing (pick one): `appetite-check` < `t-shirt-sizing` < `relative-estimation` < `reference-class-forecast`. Unordered: rest.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| vertical-slicing | Decompose into end-to-end shippable slices | Spec, DesignDoc, StoryMap | SliceSet (stories) | DA "Produce a Potentially Consumable Solution"; Patton story mapping "walking skeleton then flesh" | init, proj | M | A | documented (goal) |
| scope-cutlines | Define must-have vs nice-to-have scopes within the appetite | SliceSet, Appetite | ScopeMap | Shape Up "fixed time, variable scope", https://basecamp.com/shapeup/1.2-chapter-03 | proj | S | A+H | documented |
| dependency-dag | Order slices by dependency; detect cycles; merge order | SliceSet | DependencyDAG | Critical-path scheduling (PMBOK, inferred) | init, proj | S | A | inferred |
| appetite-check | Verify plan fits the appetite; if not, narrow problem | DependencyDAG, Appetite | AppetiteVerdict | Shape Up ch.3 | proj | S | A+H | documented |
| t-shirt-sizing | Coarse S/M/L sizing per slice | SliceSet | SizeEstimates | Agile estimation (Cohn, inferred) | proj, story | S | A | inferred |
| relative-estimation | Story points / planning poker | SliceSet | Estimates | Cohn *Agile Estimating and Planning* (inferred) | proj, story | S | A+H | inferred |
| reference-class-forecast | Estimate from historical distributions of similar items | SliceSet, History | ForecastRange | Flyvbjerg reference-class forecasting; Monte Carlo (Vacanti) (inferred) | init | M | A+T | inferred |
| risk-register | Enumerate risks with likelihood/impact/owner/response | DesignDoc, RiskList | RiskRegister | DA "Address Risk" goal; PMBOK risk; SWEBOK ch.9 | init, proj | S | A | documented (goal) |
| pre-mortem | Imagine failure, list causes, mitigate now | Plan | PreMortemFindings | Klein pre-mortem (HBR 2007, inferred) | init, proj | S | A+H | inferred |
| test-strategy | Decide test levels/types, environments, automation, entry/exit criteria | Spec, DesignDoc, RiskRegister | TestStrategy | ISO/IEC/IEEE 29119-2 test planning; ISTQB "Test Planning"; DA "Develop Test Strategy" | init, proj | M | A | documented |
| rollout-plan | Choose deployment pattern (flag/canary/blue-green), stages, gates, rollback | DesignDoc, RiskRegister | RolloutPlan | Humble & Farley *Continuous Delivery*; SRE ch.27 launch checklist, https://sre.google/sre-book/reliable-product-launches/ | proj, story(risky), vuln | S | A | documented (SRE) / inferred (CD book) |
| release-plan | Map slices to releases/milestones | DependencyDAG, Estimates | ReleasePlan | DA "Plan the Release" | init, proj | S | A+H | documented (goal) |
| spike-timebox-plan | Set time-box, checkpoints, kill criteria for a spike | SpikeCharter | SpikePlan | XP; Shape Up appetite | spike | S | A | inferred |
| capacity-allocation | Assign agents/humans and parallelism per slice | DependencyDAG | Assignment | DA "Coordinate Activities"; Team Topologies | init, proj | S | A+H | documented (goal) |
| definition-of-done-tailoring | Attach DoD/gates to each slice | SliceSet, DefinitionOfDone | SliceSet' | Scrum DoD | proj | S | A | inferred |
| hill-chart-setup | Define scopes to track uphill (unknowns) vs downhill (execution) | SliceSet | HillChart | Shape Up ch.13 (inferred) | proj | S | A | inferred |
| migration-sequencing | Order schema/data changes across releases (expand → migrate → contract) | DataMigrationPlan, DependencyDAG | MigrationSchedule | Parallel Change | proj, debt | S | A | documented |
| plan-review | Verify traceability (AC → slice → test), DAG acyclic, appetite fit | Plan | ReviewVerdict | (structural; SWEBOK ch.1 §7.3 tracing) | init, proj | S | A | documented (tracing) |

### 2.7 Implement

Goal: produce working, tested, documented change. Entry: planned slice with AC + DoD. Exit: Changeset + ImplementationReport.

Ordered options for the development loop (pick one): `test-after` < `tdd-red-green-refactor` < `bdd-outside-in`. Ordered options for large change safety: `direct-change` < `branch-by-abstraction` < `strangler-fig`. Unordered: rest.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| tdd-red-green-refactor | Write failing test, make it pass, refactor; from a test list | Spec(AC), FixDesign | Changeset (code+tests) | Beck TDD; Fowler, https://martinfowler.com/bliki/TestDrivenDevelopment.html | story, bug, vuln | M | T | documented |
| bdd-outside-in | Drive from failing acceptance scenario inward to units | GherkinScenarios | Changeset | ISTQB collaboration-based approaches; Freeman & Pryce GOOS (inferred) | story | M | T | documented (ISTQB) |
| test-after | Implement then add tests (acceptable only for trivial/chore) | Spec | Changeset | (baseline; SWEBOK ch.4 construction testing) | chore, docs | S | T | inferred |
| walking-skeleton | Implement thinnest end-to-end path first | SkeletonPlan | Changeset | Cockburn; DA "Prove Architecture Early" | proj (first slice) | M | T | documented (goal) |
| refactor-catalog-move | Apply named refactorings (Extract Function, Move, Rename, Split Phase...) with tests green | Code, tests | Changeset | Fowler *Refactoring* catalog, https://refactoring.com/catalog/ | story, debt | S–M | T | documented |
| parallel-change | Expand/migrate/contract an interface or schema across commits | DataMigrationPlan or APIContract | Changeset (phase-tagged) | Fowler/Sato Parallel Change, https://martinfowler.com/bliki/ParallelChange.html | story, debt, proj | M | T | documented |
| branch-by-abstraction | Introduce abstraction seam, swap implementations incrementally on trunk | DesignDoc | Changeset | Fowler bliki BranchByAbstraction (inferred) | debt, proj | M | T | inferred |
| strangler-fig | Route traffic gradually from legacy to new implementation | ArchitectureDescription | Changeset, RoutingConfig | Fowler StranglerFigApplication (inferred) | init, proj, debt | L | T | inferred |
| feature-flag-wrap | Put new behaviour behind a release/ops/experiment toggle | RolloutPlan | Changeset, FlagRecord | Hodgson Feature Toggles, https://martinfowler.com/articles/feature-toggles.html | story, proj | S | T | documented |
| schema-migration-script | Write reversible, idempotent migration + backfill scripts | DataMigrationPlan | MigrationScript | DORA database change management, https://dora.dev/capabilities/ | story, proj | M | T | documented |
| instrumentation | Emit logs/metrics/traces per ObservabilityPlan | ObservabilityPlan | Changeset | SRE Workbook SLOs; DORA monitoring | story | S | T | documented |
| error-handling-hardening | Implement failure handling per resilience selection | DesignDoc(resilience) | Changeset | SWEBOK ch.3 §3.5 Errors/Exception Handling/Fault Tolerance | story | S | T | documented |
| secure-coding-pass | Apply secure-coding practices, input validation, secrets handling | SecurityRequirements | Changeset | SSDF PW.5/PW.6; ASVS | story, vuln | S | T | documented |
| dependency-update | Bump/replace dependency; verify compatibility | VulnAssessment or DebtRegisterEntry | Changeset | SSDF PW.4 reuse well-secured software; DORA code maintainability | vuln, chore | S | T | documented |
| code-generation | Generate clients/models from contracts | APIContract, DataContract | Changeset | Contract-first practice (inferred) | story | S | T | inferred |
| lint-format-fix | Bring code to style/lint baseline | Changeset | Changeset' | DORA code maintainability | * | S | T | documented |
| docs-update | Update tutorial/how-to/reference/explanation as applicable | Changeset, Spec | Docs (Diátaxis quadrant) | Diátaxis, https://diataxis.fr/ ; DORA documentation quality | story, proj, docs | S | A | documented |
| runbook-authoring | Write operational runbook for new failure modes/alerts | ObservabilityPlan, FailureModeTable | Runbook | SRE practice (ch.11 on-call, inferred); DA "Ensure Production Readiness" | story, proj | S | A | inferred/documented(goal) |
| hotfix | Minimal targeted change to restore service, with follow-up debt logged | RootCause or Mitigation | Changeset(hotfix), DebtRegisterEntry | PagerDuty repair actions; ITIL emergency change (inferred) | inc, bug(sev-high) | S | T | documented (PD) / inferred |
| backport | Port a fix to supported release branches | Changeset | Changeset(backport) | Release-branch practice (inferred) | bug, vuln | S | T | inferred |
| commit-hygiene | Atomic commits, conventional messages, linked item ids | Changeset | Changeset' | Conventional Commits (inferred); SWEBOK ch.8 SCM | * | S | A | inferred |
| self-review | Author reviews own diff against checklist before requesting review | Changeset | SelfReviewNotes | Google eng-practices author guide (inferred sibling page) | * | S | A | inferred |
| implementation-report | Summarize what changed, how verified, deviations from plan | Changeset, TestReport | ImplementationReport | (structural) | story, bug | S | A | inferred |
| pairing-session | Human-agent or agent-agent pairing on a hard slice | Slice | Changeset | XP pair programming; mob programming (inferred) | story(complex) | M | H+A | inferred |
| i18n-l10n-pass | Externalize strings, locale handling | Changeset | Changeset' | Practice (inferred) | story(UI) | S | T | inferred |

### 2.8 Verify (test)

Goal: demonstrate AC/NFR conformance and no regression. Entry: Changeset. Exit: TestReport + ScanReport + VerificationVerdict.

Ordered options for test-design rigor on a story (pick the highest justified): `smoke-test` < `unit-tests` < `equivalence-boundary` < `decision-table-state` < `property-based` < `mutation-testing`. Unordered: levels/types, non-functional, security, accessibility, exploratory.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| test-analysis-design | Derive test conditions/cases from AC using named techniques | Spec(AC), TestStrategy | TestDesign | ISTQB CTFL v4 ch.4 Test Analysis & Design; ISO 29119-4 techniques | story, bug | S | A | documented |
| unit-tests | Component-level automated tests | Changeset | TestReport(unit) | ISTQB test levels; SWEBOK ch.5 | story, bug | S | T | documented |
| integration-tests | Test interactions between components/services | Changeset | TestReport(integration) | ISTQB test levels | story, proj | M | T | documented |
| contract-tests | Verify provider/consumer conform to APIContract | APIContract | TestReport(contract) | Consumer-driven contracts / Pact (inferred) | story, proj | S | T | inferred |
| e2e-acceptance-tests | Execute BDD scenarios / DoD E2E through the real app | GherkinScenarios | TestReport(e2e) | ISTQB acceptance testing; ATDD | story, proj | M | T | documented |
| equivalence-boundary | Equivalence partitioning + boundary value analysis | Spec(AC) | TestDesign, tests | ISTQB black-box techniques; 29119-4 | story, bug | S | A+T | documented |
| decision-table-state | Decision-table and state-transition testing | Spec, BehaviorModel | TestDesign, tests | ISTQB black-box techniques | story | S | A+T | documented |
| pairwise-combinatorial | Cover parameter interactions with pairwise/n-wise | Spec | TestDesign | 29119-4 combinatorial; ISTQB advanced (inferred) | story | S | A+T | inferred |
| white-box-coverage | Statement/branch coverage analysis and gap filling | Changeset, CoverageReport | CoverageReport' | ISTQB white-box techniques; 29119-4 | story | S | T | documented |
| property-based-testing | Generate inputs against invariants | DomainDesign | tests, TestReport | QuickCheck/Hypothesis (inferred) | story(core logic) | M | T | inferred |
| mutation-testing | Measure test suite strength by seeding faults | tests | MutationScore | PIT/Stryker (inferred); ISTQB advanced | story(critical) | M | T | inferred |
| fuzz-testing | Random/structured malformed inputs to find crashes | Changeset | FuzzReport | SSDF PW.8 "fuzz testing" example; OSS-Fuzz (inferred) | story(parsers), vuln | M | T | documented (SSDF) / inferred |
| exploratory-testing | Session-based charter-driven exploration | Changeset, Spec | SessionNotes, Defects | ISTQB experience-based techniques (error guessing, exploratory, checklist) | story, proj | M | A+H | documented |
| regression-suite | Run full existing suite | Changeset | TestReport(regression) | ISTQB regression testing; DORA CI | * | S | T | documented |
| smoke-test | Minimal build-verification checks | Changeset/Build | SmokeResult | ISTQB (confirmation/smoke); CD (inferred) | * | S | T | inferred |
| confirmation-test | Re-run the failing repro to confirm fix | ReproCase, Changeset | ConfirmationResult | ISTQB confirmation testing; Agans rule 9 | bug, vuln, inc | S | T | documented |
| performance-load-test | Load/throughput/latency against budgets | CapacityPlan, Changeset | PerfReport | Performance engineering (inferred); SRE ch.27 load testing | story, proj | M | T | inferred/documented |
| stress-soak-test | Beyond-capacity and long-duration stability | CapacityPlan | PerfReport | ISTQB non-functional (inferred) | proj | L | T | inferred |
| sast-scan | Static application security testing | Changeset | ScanReport(SAST) | SSDF PW.7 code review/analysis; SAMM Security Testing | * | S | T | documented |
| dast-scan | Dynamic scanning of running app | DeployedEnv | ScanReport(DAST) | SSDF PW.8 executable testing; SAMM | proj, vuln | M | T | documented |
| sca-scan | Software composition analysis vs known vulns/licenses | SBOM/lockfile | ScanReport(SCA) | SSDF PW.4, RV.1; CycloneDX, https://cyclonedx.org/guides/ | * | S | T | documented |
| secret-scan | Detect committed secrets | Changeset | ScanReport(secrets) | SSDF PS.1 protect code (inferred mapping) | * | S | T | inferred |
| security-test-asvs | Verify controls against ASVS requirements for the target level | SecurityRequirements | SecurityTestReport | OWASP ASVS; SAMM "Requirements-driven Testing" | proj, story(sec), vuln | M | T | documented |
| accessibility-audit | WCAG-EM: scope, explore, sample, audit, report (+ automated axe) | AccessibilityRequirements, DeployedEnv | AccessibilityReport | WCAG-EM 1.0, https://www.w3.org/TR/WCAG-EM/ | proj, story(UI) | M | T+H | documented |
| usability-test | Observe users performing tasks on the build | UXDesign, Build | UsabilityFindings | Nielsen usability testing (inferred) | proj | L | H | inferred |
| visual-regression | Snapshot/pixel diffs of UI | Build | VisualDiffReport | Practice (inferred) | story(UI) | S | T | inferred |
| migration-dry-run | Run migration + backfill on prod-like data; verify counts/invariants | MigrationScript, DataProfile | MigrationTestReport | DORA database change management; Parallel Change | story, proj | M | T | documented |
| backward-compat-test | Old clients vs new server / new code vs old schema | APIContract, Changeset | CompatReport | Parallel Change expand phase | story, proj | S | T | documented |
| chaos-experiment-preprod | Steady-state hypothesis, inject fault, compare | FailureModeTable, Env | ChaosReport | Principles of Chaos, https://principlesofchaos.org/ | proj | M | T | documented |
| test-completion-report | Summarize execution, coverage, residual risk, exit-criteria | all TestReports | VerificationVerdict | ISO 29119-3 test completion report; ISTQB "Test Completion" | story, proj | S | A | documented |
| flaky-test-triage | Identify and quarantine/fix flaky tests | TestReport history | FlakyTestList | DORA CI guidance (inferred detail) | chore | S | T | inferred |

### 2.9 Review

Goal: independent judgement. Entry: verified changeset/artifact. Exit: ReviewVerdict + ApprovalRecord.

Ordered options for change review (pick one): `checklist-review` < `code-review` < `adversarial-review` < `formal-inspection`. Unordered: specialist reviews, readiness reviews.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| code-review | Review design, functionality, complexity, tests, naming, comments, style, docs | Changeset, Spec | ReviewVerdict, ReviewFindings | Google eng-practices "What to look for", https://google.github.io/eng-practices/review/reviewer/ ; SSDF PW.7 | story, bug, vuln, chore | S | A (+H at gate) | documented |
| checklist-review | Lightweight pass over a fixed checklist | Changeset | ReviewVerdict | ISTQB review types (informal/checklist-based) | chore, docs, small story | S | A | documented |
| adversarial-review | Reviewer tries to break the change: missing AC, edge cases, reachability | Changeset, Spec, TestReport | ReviewVerdict, ReviewFindings | Red-team style review (inferred); redkiln existing practice | story, proj | M | A | inferred |
| formal-inspection | Role-based inspection with defect log and metrics | Artifact | InspectionRecord | Fagan inspection; ISTQB "Feedback & Review Process"; SWEBOK ch.12 | init(safety/compliance) | L | H | documented (ISTQB) |
| design-review | Review DesignDoc/ADRs before implementation | DesignDoc, ADRs | ReviewVerdict | SWEBOK ch.3 §6.1; ch.2 §4.3 Architecture Reviews | proj, init | M | A+H | documented |
| security-review | Review change against threat model/ASVS; sign off residual risk | Changeset, ThreatModel, ScanReports | SecurityReviewVerdict | SAMM Verification; SSDF PW.7 | story(sec), proj, vuln | M | A+H | documented |
| privacy-review | Confirm DPIA/privacy design honored | Changeset, DPIA | PrivacyReviewVerdict | GDPR DPIA process (inferred) | proj, story(PII) | S | A+H | inferred |
| api-review | Contract compatibility, naming, versioning sign-off | APIContract, Changeset | ReviewVerdict | API review board practice (inferred) | story, proj | S | A | inferred |
| accessibility-review | Sign off AccessibilityReport findings | AccessibilityReport | ReviewVerdict | WCAG-EM report step | story(UI), proj | S | A+H | documented |
| docs-review | Check docs land in the right Diátaxis quadrant and are accurate | Docs | ReviewVerdict | Diátaxis; DORA documentation quality | docs, story | S | A | documented |
| sbom-license-review | Review SBOM diff for new components/licenses | SBOM | ReviewVerdict | CycloneDX; SSDF PW.4 | story, chore | S | A+T | documented |
| test-review | Review test design adequacy vs AC and technique choice | TestDesign, TestReport | ReviewVerdict | ISTQB static testing of testware | story | S | A | documented |
| production-readiness-review | SRE PRR: reliability needs, monitoring, capacity, on-call, runbooks | DesignDoc, ObservabilityPlan, Runbook, CapacityPlan | PRRReport | SRE ch.32 PRR model, https://sre.google/sre-book/evolving-sre-engagement-model/ ; DA "Ensure Production Readiness" | init, proj | L | A+H | documented |
| launch-review | Launch coordination checklist across arch, capacity, failure modes, comms, rollback | RolloutPlan, PRRReport | LaunchChecklist(signed) | SRE ch.27 + Appendix E launch checklist, https://sre.google/sre-book/reliable-product-launches/ | init, proj | M | A+H | documented |
| change-authorization | Record and authorize the change (standard/normal/emergency) | ReleaseCandidate, RolloutPlan | ChangeRecord(authorized) | ITIL 4 change enablement (inferred; practice guide not fetched) | proj, story, inc | S | A+H | inferred |
| requirements-signoff | Stakeholder approval of spec before design | Spec | ApprovalRecord | SWEBOK ch.1 §5 Requirements Validation | init, proj | S | H | documented |
| spike-decision-review | Review spike report and take the decision | SpikeReport | DecisionRecord/ADR | XP spike; ADR | spike | S | H (A drafts) | documented (ADR) |
| review-gate | Generic human approval gate with verdict recorded | any artifact + ReviewVerdict | ApprovalRecord | (structural; DA "Govern Team") | * | S | H | inferred |

### 2.10 Release / Deploy

Goal: production safely, reversibly, observably. Entry: ApprovalRecord + RolloutPlan. Exit: ReleaseRecord + DeploymentLog + ReleaseNotes.

Ordered options for deployment pattern (pick one): `rolling-deploy` < `blue-green-deploy` < `canary-release` < `flag-progressive-rollout`. `dark-launch` is orthogonal (pick with any). Unordered: packaging, provenance, comms, migration phases.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| build-package | Reproducible build and artifact packaging | Changeset | BuildArtifact | Humble & Farley deployment pipeline (inferred); DORA CI | * | S | T | documented (DORA) |
| artifact-signing-provenance | Sign artifacts; generate provenance attestations | BuildArtifact | Attestation | SSDF PS.2/PS.3; SLSA (inferred); CycloneDX attestations guide | * | S | T | documented |
| sbom-generate | Produce SBOM for the release | BuildArtifact | SBOM | CycloneDX guides, https://cyclonedx.org/guides/ ; SSDF PS.3 | * | S | T | documented |
| versioning | Assign semantic version / tag | Changeset | VersionTag | SemVer (inferred) | * | S | A | inferred |
| release-notes | Write user-facing notes and changelog | Changeset set, Spec | ReleaseNotes, Changelog | Keep a Changelog (inferred); DORA documentation | proj, story, bug | S | A | inferred |
| deployment-pipeline-run | Execute staged pipeline with automated gates | BuildArtifact | DeploymentLog | DORA deployment automation / continuous delivery, https://dora.dev/capabilities/ | * | S | T | documented |
| rolling-deploy | Replace instances incrementally | BuildArtifact | DeploymentLog | Kubernetes rolling update (inferred) | story, bug | S | T | inferred |
| blue-green-deploy | Cut over between two identical environments; instant rollback | BuildArtifact | DeploymentLog | Fowler Blue-Green, https://martinfowler.com/bliki/BlueGreenDeployment.html | proj, story | M | T | documented |
| canary-release | Route a subset of users/regions to new version; ramp on metrics | BuildArtifact, SLOSpec | DeploymentLog, CanaryReport | Fowler/Sato Canary Release, https://martinfowler.com/bliki/CanaryRelease.html | proj, story(risky) | M | T | documented |
| flag-progressive-rollout | Enable feature toggle by cohort/percentage | FlagRecord | RolloutLog | Feature Toggles article "canary releasing"/release toggles | proj, story | S | T | documented |
| dark-launch | Exercise new path with real traffic without exposing results | FlagRecord | DarkLaunchMetrics | SRE ch.27 (dark launch described in Reliable Product Launches; inferred detail) | proj | M | T | inferred |
| migration-expand-execute | Run expand-phase schema change and backfill in prod | MigrationScript, MigrationSchedule | MigrationLog | Parallel Change; DORA DB change mgmt | story, proj | M | T | documented |
| migration-contract-execute | Remove old paths/columns after all consumers migrated | MigrationSchedule, Telemetry | MigrationLog | Parallel Change contract phase | story, debt | S | T | documented |
| post-deploy-smoke | Verify health/synthetics after deploy | DeploymentLog | SmokeResult | CD practice (inferred); SRE launch | * | S | T | inferred |
| rollback | Revert to previous version/flag state | DeploymentLog | RollbackRecord | PagerDuty "Bad deployment: roll it back"; Blue-Green | * | S | T | documented |
| launch-communication | Announce internally/externally; status page/notes | ReleaseNotes | Announcement | SRE ch.27 launch coordination; PagerDuty customer liaison | proj, init, inc | S | A+H | documented |
| change-record-close | Update change record with outcome | ChangeRecord, DeploymentLog | ChangeRecord(closed) | ITIL change enablement (inferred) | * | S | A | inferred |
| deprecation-notice | Publish deprecation with timeline and migration guide | DeprecationPlan | DeprecationNotice | API lifecycle practice (inferred) | proj, debt | S | A | inferred |
| release-record | Record what shipped, where, when, by which pattern, links to evidence | all release artifacts | ReleaseRecord | SWEBOK ch.8 SCM release management | * | S | A | documented (topic) |

### 2.11 Operate / Observe

Goal: stay within SLO; detect/mitigate incidents; learn from telemetry. Entry: change live or alert fired. Exit: IncidentRecord(resolved) / SLOReport / Runbook updates.

Ordered options for mitigation (pick the least invasive that restores service): `rollback` (see release) < `feature-flag-kill` < `rate-limit-shed` < `failover` < `rolling-restart` < `hotfix` (see implement). Unordered: rest.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| slo-monitoring | Track SLIs vs SLO and error budget burn | SLOSpec, Telemetry | SLOReport | SRE Workbook ch.2, https://sre.google/workbook/implementing-slos/ | proj, init, chore | S | T | documented |
| alert-rule-authoring | Define symptom-based, actionable alerts tied to SLOs | SLOSpec, ObservabilityPlan | AlertRules | SRE ch.10 Practical Alerting (inferred detail); Workbook ch.5 | story, proj | S | T | inferred |
| dashboard-authoring | Build dashboards for SLIs and key flows | ObservabilityPlan | Dashboard | SRE monitoring (inferred) | proj | S | T | inferred |
| synthetic-monitoring | Probe critical journeys continuously | Spec(critical paths) | SyntheticChecks | Practice (inferred) | proj | S | T | inferred |
| on-call-handoff | Brief on-call on new risks/runbooks after release | ReleaseRecord, Runbook | HandoffNote | SRE ch.11 Being On-Call (inferred); DA "Deploy the Solution" | proj | S | A | inferred |
| incident-command | Run IC/deputy/scribe roles, delegate, span of control, declare end | IncidentRecord | IncidentRecord(updated), IncidentTimeline | PagerDuty "During an Incident", https://response.pagerduty.com/during/during_an_incident/ ; SRE ch.14 Managing Incidents | inc | M | H (A as scribe/analyst) | documented |
| incident-communication | Internal updates, executive summary, public status page | IncidentRecord | StatusUpdate | PagerDuty customer liaison "if in doubt, post it out" | inc | S | A+H | documented |
| feature-flag-kill | Disable feature via ops toggle to mitigate | FlagRecord | MitigationRecord | Feature Toggles "ops toggles" | inc | S | T | documented |
| rate-limit-shed | Throttle or shed load to protect core | IncidentRecord | MitigationRecord | PagerDuty repair actions "rate-limit services"; SRE ch.21 | inc | S | T | documented |
| failover | Drain bad region/datacenter/DB replica | IncidentRecord | MitigationRecord | PagerDuty "validate automation has removed bad data center" | inc | S | T+H | documented |
| rolling-restart | Restart stuck/crashed processes | IncidentRecord | MitigationRecord | PagerDuty "web application stuck/crashed: rolling restart" | inc | S | T | documented |
| workaround-publish | Document temporary workaround for users/support | IncidentRecord | KnownErrorRecord | ITIL problem management known errors (inferred) | inc, bug | S | A | inferred |
| error-budget-review | Periodic review of budget burn; decide feature vs reliability work | SLOReport | ErrorBudgetDecision | SRE Workbook ch.2 "decision making using SLOs and error budgets" | init, proj | S | A+H | documented |
| capacity-planning | Forecast demand and provision | Telemetry, CapacityPlan | CapacityPlan' | SRE ch.27; Workbook capacity (inferred) | init, chore | M | A+T | documented (SRE) |
| cost-review | Review cloud/infra cost vs budget | BillingData | CostReport | FinOps (inferred); DORA flexible infrastructure "cost visibility" | chore, init | S | T | documented (DORA mention) |
| vuln-monitoring | Continuously match SBOM against new advisories | SBOM | VulnReport | SSDF RV.1; CycloneDX | chore, vuln | S | T | documented |
| chaos-experiment-prod | Run controlled fault injection in production with minimized blast radius | FailureModeTable, SLOSpec | ChaosReport | Principles of Chaos "run experiments in production, minimize blast radius" | proj, init | M | T+H | documented |
| game-day | Team exercise simulating an outage/DR to test runbooks and people | Runbook, FailureModeTable | GameDayReport | SRE ch.28 "disaster role-playing"; AWS game days (inferred) | init, chore | L | H (A facilitates) | inferred |
| dr-drill | Restore from backup / region failover rehearsal | DRPlan | DRDrillReport | SRE ch.26 Data Integrity (inferred) | init, chore | L | T+H | inferred |
| toil-reduction | Identify and automate repetitive ops work | OpsLog | Changeset/DebtRegisterEntry | SRE ch.5 Eliminating Toil (inferred) | chore, debt | M | A | inferred |
| adoption-measurement | Measure feature usage vs outcome hypothesis | Telemetry, OutcomeStatement | OutcomeReport | DORA "Customer feedback"; Torres outcome | proj, init | S | T | documented |
| user-feedback-collection | Gather post-release feedback (support, surveys, NPS) | ReleaseRecord | FeedbackSummary | DORA customer feedback capability | proj, init | S | A+T | documented |
| problem-record | Open a problem record for recurring incidents; link known errors | IncidentRecords | ProblemRecord | ITIL 4 problem management (inferred) | inc, bug | S | A | inferred |
| runbook-update | Update runbooks/alerts from what was learned | IncidentRecord | Runbook' | PagerDuty after-incident; SRE | inc | S | A | documented |

### 2.12 Closeout / Retrospective

Goal: learn, harvest, tidy, close. Entry: outcome delivered / incident resolved / spike time-box ended. Exit: Postmortem or Retrospective + ActionItems + ClosureRecord + KB atoms.

Ordered options for the learning ceremony (pick one): `lightweight-retro` < `structured-retrospective` < `blameless-postmortem`. Unordered: rest.

| id | purpose | inputs | outputs | lineage | kinds | cost | exec | ev |
|---|---|---|---|---|---|---|---|---|
| blameless-postmortem | Document impact, timeline, root causes, contributing factors, what went well/badly/lucky, action items; review meeting | IncidentTimeline, RootCause, ImpactAssessment | Postmortem, ActionItems | SRE ch.15 + Workbook Postmortem Culture, https://sre.google/sre-book/postmortem-culture/ ; PagerDuty Postmortem Process, https://response.pagerduty.com/after/post_mortem_process/ ; Atlassian, https://www.atlassian.com/incident-management/postmortem/blameless | inc, bug(sev-high), vuln | M | A drafts, H reviews | documented |
| postmortem-owner-designation | IC assigns owner; schedule within 3/5 days by severity | IncidentRecord | Assignment | PagerDuty postmortem process | inc | S | H | documented |
| postmortem-review-meeting | Review content, agree action items, external message | Postmortem draft | Postmortem(reviewed) | PagerDuty status flow Draft→In Review→Reviewed→Closed | inc | M | H | documented |
| structured-retrospective | Set the stage, gather data, generate insights, decide, close (e.g. 4Ls/starfish) | stage reports, metrics | Retrospective, ActionItems | Derby & Larsen *Agile Retrospectives* (inferred); DA "Evolve WoW" | init, proj | M | A drafts, H | inferred/documented(goal) |
| lightweight-retro | Short what-went-well/what-to-change note | ImplementationReport | RetroNote | Practice (inferred) | story, bug, spike | S | A | inferred |
| action-item-tracking | Convert learnings into tracked items with owners/dates | Postmortem/Retrospective | ActionItems → new work items | SRE postmortem "action items" best practice; PagerDuty "create follow-up tickets" | inc, init, proj | S | A | documented |
| dora-metrics-review | Review lead time, deploy frequency, change fail rate, MTTR for the item/period | DeploymentLog, IncidentRecords | MetricsReport | DORA four keys (dora.dev, inferred detail from catalog page) | init, proj | S | T | inferred/documented |
| outcome-vs-hypothesis | Compare delivered outcome against OutcomeStatement/OpportunityMap | OutcomeReport, OutcomeStatement | OutcomeAssessment | Torres OST; DA "Measure Outcomes" | init, proj | S | A | documented |
| knowledge-harvest | Extract concepts/decisions/playbooks into the KB; supersede stale atoms | stage reports, ADRs, Postmortem | KBAtoms | DORA documentation quality; Diátaxis "explanation" quadrant | * | M | A | documented |
| docs-finalization | Ensure tutorials/how-tos/reference reflect shipped behaviour | Docs, ReleaseRecord | Docs' | Diátaxis | proj, story, docs | S | A | documented |
| flag-cleanup | Remove transient release toggles; log long-lived ones | FlagRecord | Changeset, FlagRegistry' | Feature Toggles "managing the carrying cost" | proj, story | S | T | documented |
| debt-logging | Record shortcuts taken as tech-debt items | ImplementationReport | DebtRegisterEntry | DA "Improve Quality"; SWEBOK ch.7 | story, inc | S | A | documented (goal) |
| spike-report | Answer the spike question with evidence and a recommendation; record decision | SpikeCharter, experiment results | SpikeReport, DecisionRecord | XP spike; ADR | spike | S | A | documented (ADR) |
| decommission-archive | Retire old system/feature, archive artifacts, revoke access | DeprecationPlan | DecommissionRecord | ISO 12207 disposal process (inferred from process model); ITIL | proj, debt | M | T+H | inferred |
| problem-record-close | Close ITIL problem record once permanent fix verified | ProblemRecord, ReleaseRecord | ProblemRecord(closed) | ITIL 4 problem management (inferred) | inc, bug | S | A | inferred |
| closure-record | Formal close: AC coverage audit, tree clean, links reconciled, sign-off | all item artifacts | ClosureRecord | (structural; DA "Govern Team"; 12207 project assessment & control) | * | S | A+H | inferred |

---

## 3. Artifact type vocabulary

Artifacts are the join key: an activity's `inputs` must be satisfiable by artifacts produced earlier (or supplied at intake). Grouped by the stage that usually produces them; `→` marks the primary producing activity.

**Intake/Triage:** IntakeRecord, RawIdea, BugReport, IncidentSignal, VulnReport, TriageDecision, TriageScore, SeverityRating, Appetite, Assignment, DuplicateLink, ReproStatus, ReadinessVerdict, PrioritizedBacklog, VulnAssessment.

**Discovery:** ProblemStatement, StakeholderMap, OutcomeStatement, OpportunityMap, InterviewGuide, InterviewNotes, JobStory, Persona, CompetitorAnalysis, LandscapeReport, EvidenceSummary, ImpactMap, AssumptionList, StoryMap, EventStormBoard, DomainGlossary/Glossary, ValuePropositionCanvas, Breadboard, ConceptSketch, PrototypeFindings, RiskList, DiscoveryReport.

**Analysis:** ReproCase, RootCauseHypothesis, RootCause, DebugLog, DeltaFindings, CulpritCommit, TraceAnalysis, LogFindings, ChangeCorrelation, ProfileReport, MemoryFindings, IncidentTimeline, CausalChain, CauseCategoryMap, FaultTree, ImpactAssessment, BlastRadius, ContextNotes, DomainModel, ConflictLog, FeasibilityReport, DataProfile, ScanFindings, DebtRegisterEntry, AnalysisReport, SpikeQuestion.

**Specification:** Spec (Story+AC), RequirementsSet, GherkinScenarios, UseCaseSpec, SpecModel, ExampleMap, OpenQuestions, QualityAttributeScenarios, SLOSpec, APIContract, DataContract, SecurityRequirements, AbuseCases, PrivacyRequirements, DPIAScreening, AccessibilityRequirements, PrioritizedRequirements, TraceabilityMatrix, Pitch, DefinitionOfDone, SpikeCharter, Spec(Fix).

**Design:** C4ContextDiagram, C4ContainerDiagram, C4ComponentDiagram, ArchitectureDescription, OptionsAnalysis, ADR, ArchitectureEvaluation, ContextMap, AggregateDesign, DomainDesign, APIDesignNotes, DataModel, DataMigrationPlan, ThreatModel, DPIA, PrivacyDesign, UXDesign (Flows, Wireframes), InteractionSpec, CapacityPlan, FailureModeTable, ObservabilityPlan, BehaviorModel, DesignDoc, SkeletonPlan, DeprecationPlan, FixDesign.

**Planning:** SliceSet, ScopeMap, DependencyDAG, AppetiteVerdict, SizeEstimates/Estimates/ForecastRange, RiskRegister, PreMortemFindings, TestStrategy, RolloutPlan, ReleasePlan, SpikePlan, HillChart, MigrationSchedule, Plan.

**Implement:** Changeset, MigrationScript, FlagRecord, RoutingConfig, Docs (Tutorial/HowTo/Reference/Explanation), Runbook, SelfReviewNotes, ImplementationReport.

**Verify:** TestDesign, TestReport (unit/integration/contract/e2e/regression), CoverageReport, MutationScore, FuzzReport, SessionNotes, SmokeResult, ConfirmationResult, PerfReport, ScanReport (SAST/DAST/SCA/secrets), SecurityTestReport, AccessibilityReport, UsabilityFindings, VisualDiffReport, MigrationTestReport, CompatReport, ChaosReport, FlakyTestList, VerificationVerdict.

**Review:** ReviewVerdict, ReviewFindings, InspectionRecord, SecurityReviewVerdict, PrivacyReviewVerdict, PRRReport, LaunchChecklist, ChangeRecord, ApprovalRecord, DecisionRecord.

**Release:** BuildArtifact, Attestation, SBOM, VersionTag, ReleaseNotes, Changelog, DeploymentLog, CanaryReport, RolloutLog, DarkLaunchMetrics, MigrationLog, RollbackRecord, Announcement, DeprecationNotice, ReleaseRecord.

**Operate:** SLOReport, AlertRules, Dashboard, SyntheticChecks, HandoffNote, IncidentRecord, StatusUpdate, MitigationRecord, KnownErrorRecord, ErrorBudgetDecision, CostReport, GameDayReport, DRDrillReport, OutcomeReport, FeedbackSummary, ProblemRecord.

**Closeout:** Postmortem, Retrospective, RetroNote, ActionItems, MetricsReport, OutcomeAssessment, KBAtoms, FlagRegistry, SpikeReport, DecommissionRecord, ClosureRecord.

Counts: ~190 artifact types before de-duplication; a shipped catalog should collapse variants (e.g. all `*Verdict` into `Verdict{kind}`, all `TestReport(level)` into `TestReport{level}`, all `ScanReport(tool)` into `ScanReport{tool}`), landing at ~110 canonical types.

---

## 4. Kind × stage applicability (default path)

| kind | triage | discovery | analysis | spec | design | plan | implement | verify | review | release | operate | closeout |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| initiative | yes | yes | optional (feasibility) | yes (requirements) | yes (architecture) | yes (decompose to projects) | no (via projects) | no | yes (design/PRR) | optional (launch) | yes (SLO, adoption) | yes (retro, harvest) |
| project | yes | optional | optional | yes | yes | yes (stories) | via stories + integration | yes (integration/e2e) | yes | yes | yes | yes |
| story | readiness only | no | optional (impact) | yes (AC) | light | no | yes | yes | yes | yes | handoff only | light retro |
| bug | yes | no | yes (repro, RCA) | fix-spec | fix-design | no | yes | confirmation + regression | yes | yes | no | debt-logging; postmortem if sev-high |
| incident | declare/severity | no | after mitigation (RCA) | no | no | no | hotfix | confirmation | change-authorization | rollback/hotfix | **first** (mitigate) | **mandatory** postmortem |
| spike | appetite | optional | technical-spike | spike-question | no | timebox | experiment code | no | spike-decision-review | no | no | spike-report |
| chore | quick | no | no | no | no | no | yes | regression/smoke | checklist | yes | no | no |
| tech-debt | yes | no | assessment | optional | ADR/refactor design | migration-sequencing | refactor/parallel-change | regression + compat | yes | yes | no | debt close |
| security-vuln | vuln-score | no | exploitability | security-requirements | fix-design | no | secure-coding/dependency-update | sca/sast/asvs + confirmation | security-review | expedited | vuln-monitoring | postmortem if exploited |
| docs | quick | no | no | optional | no | no | docs-update | no | docs-review | yes | no | no |

Notes from the sources that shape this table: SRE and PagerDuty both put mitigation before diagnosis for incidents; SWEBOK ch.7 (maintenance) treats impact analysis as the entry for corrective and adaptive changes, which is why bugs and debt go through analysis and not discovery; DA marks Address Risk, Improve Quality, Coordinate Activities and Govern Team as ongoing goals, which supports the "re-enterable cross-cutting activities" rule; SSDF's RV practices (identify, assess, remediate, root-cause) map one-to-one onto vuln triage → analysis → implement → closeout.

## 5. Open questions for the design note

1. Whether `review` should be a stage or a gate attribute on every stage (12207 and DA both model verification/validation as processes that run alongside technical processes rather than as a phase). The catalog keeps it as a stage for orchestration simplicity but every stage lists a `*-review` activity.
2. DA per-goal decision points could not be fetched from pmi.org this session (timeouts); the DA goal poster PDF at https://www.pmi.org/-/media/pmi/microsites/disciplined-agile/posters/daprocessgoalsposter85x11.pdf lists the 24 goals only. A follow-up should capture the decision-point lists for Explore Scope (partially captured: Explore Purpose, Explore Usage, Explore the Domain...), Develop Test Strategy, Ensure Production Readiness, Address Risk, Deploy the Solution.
3. ITIL 4 practice guides are behind Axelos's cookie/login wall; incident/problem/change rows are inferred from the practice names and general knowledge.
4. IEEE 29148 content is inferred (Wikipedia page 404'd); SWEBOK ch.1 covers the same ground and is documented.

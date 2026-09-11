from probe_lib import probe

# ADR register
probe("adr.total", r"\b89\b[^\n]{0,20}(ADR|decision|record|frontmatter)")
probe("adr.total.b", r"(ADR|decision|record)s?[^\n]{0,12}\b89\b")
probe("adr.accepted", r"\b86\b[^\n]{0,25}accept|accept[^\n]{0,12}\b86\b")
probe("adr.superseded", r"\b(2|two)\b[^\n]{0,12}supersed|supersed[^\n]{0,12}\b(2|two)\b")
probe("adr.proposed", r"\b(1|one)\b[^\n]{0,12}proposed|proposed[^\n]{0,12}\b(1|one)\b")
probe("adr.cited", r"\b56\b[^\n]{0,30}cited|cited[^\n]{0,12}\b56\b")
probe("adr.uncited", r"\b33\b[^\n]{0,30}(cited|uncited|no criterion)|(uncited|not cited)[^\n]{0,12}\b33\b")
# areas
probe("areas.39", r"\b(39|thirty-nine|Thirty-nine)\b[^\n]{0,8}areas|areas[^\n]{0,10}\b39\b")
probe("areas.decided", r"\b(26|twenty-six|Twenty-six)\b[^\n]{0,4}DECIDED|DECIDED[^\n]{0,6}\b26\b", flags=0)
probe("areas.prov", r"\b(3|three|Three)\b[^\n]{0,4}PROVISIONAL|PROVISIONAL[^\n]{0,6}\b3\b")
probe("areas.blocked", r"\b(10|ten|Ten)\b[^\n]{0,4}BLOCKED|BLOCKED[^\n]{0,6}\b10\b")
# criteria
probe("crit.514", r"\b514\b")
probe("crit.469", r"\b469\b")
probe("crit.19", r"\b514\b[^\n]{0,80}\b19\b|\b19\b[^\n]{0,30}(criteria|AC|weak|deferred|blocked)")
probe("meas.135", r"\b135\b")

# ruvn

Gamma-entrainment protocol research agent — compare modalities/dosing, grade evidence, verify signed session bundles (ruv-neural)

## Pipeline

```
scout -> web-searcher -> source-grader -> synthesizer -> fact-checker -> citer
```

Each agent only sees the OUTPUT of the previous one, not the raw inputs. This forces information to flow through grading + verification gates.

## Agents

| Agent | Role | School |
|---|---|---|
| `scout` | Decompose the research question into subqueries |  |
| `web-searcher` | Run subqueries, collect raw hits |  |
| `source-grader` | Grade each source A/B/C/D by authority + freshness + relevance | |
| `synthesizer` | Synthesise findings from grade A/B sources only |  |
| `fact-checker` | Adversarially verify each claim in the synthesis |  |
| `citer` | Final pass: every claim must cite a graded source | |

## Evidence grading rubric

- **A**: Primary source (peer reviewed academics, impartial 3rd parties), <2 years old, on-topic
- **B**: Reputable secondary source (principal outlet, community forums, 3rd party blog), <5 years
- **C**: Tertiary source (Wikipedia, official blog) — informational only
- **D**: Discarded (corporate sauce, unsourced claim, broken link)

## Output

A dossier markdown: TL;DR + body (every claim cited) + bibliography with grades.

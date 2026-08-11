# Domain Docs

This monorepo uses a multi-context domain-documentation layout.

## Before exploring

- Read the root `CONTEXT-MAP.md`.
- Follow it to every `CONTEXT.md` relevant to the task.
- Read applicable system-wide ADRs under `docs/adr/`.
- Read applicable context-specific ADRs under the mapped context.

Missing documents are created lazily as terminology and decisions are resolved.

## Layout

```
/
├── CONTEXT-MAP.md
├── docs/adr/                    ← system-wide decisions
└── <context>/
    ├── CONTEXT.md
    └── docs/adr/                ← context-specific decisions
```

## Vocabulary

Use domain terms exactly as defined by the relevant `CONTEXT.md`. If a necessary concept is missing, record it for `/domain-modeling` instead of silently inventing competing terminology.

## ADR conflicts

If proposed work contradicts an existing ADR, identify the conflict explicitly instead of silently overriding the decision.

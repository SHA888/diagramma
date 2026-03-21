# @diagramma/bridge

[![npm](https://img.shields.io/npm/v/@diagramma/bridge.svg)](https://www.npmjs.com/package/@diagramma/bridge)

Conversation bridge for [diagramma](https://github.com/SHA888/diagramma) — connects LLM conversations to interactive diagrams.

## What This Package Provides

This is what makes diagramma *conversation-aware*. The bridge handles four concerns:

1. **Context ingestion** — conversation history → diagram spec generation prompts
2. **Click callbacks** — clicking a diagram node → follow-up question to the LLM
3. **Incremental updates** — conversation progresses → diagram patches (add/remove/modify nodes)
4. **Streaming support** — diagram renders progressively as the LLM streams tokens

## Usage

```typescript
import { DiagramBridge } from '@diagramma/bridge';

const bridge = new DiagramBridge({
  onSpecUpdate: (spec) => {
    // Feed to <Diagram spec={spec} />
  },
  onPromptGenerated: (prompt) => {
    // Send to your LLM API
  },
});

// Feed conversation messages
bridge.ingestMessage({ role: 'user', content: 'Explain the auth flow' });
bridge.ingestMessage({ role: 'assistant', content: '...' });

// Handle node clicks (generates follow-up prompt)
bridge.handleNodeClick('auth-service');
// → onPromptGenerated("Tell me more about the auth-service node...")

// Stream LLM response for progressive rendering
const stream = await fetch('/api/chat', { ... });
for await (const chunk of stream) {
  bridge.feedStreamChunk(chunk);
  // → onSpecUpdate fires as valid partial specs are parsed
}

// Apply incremental patches
bridge.patchSpec({
  add: [{ id: 'cache', label: 'Redis cache', color: 'coral' }],
  addEdges: [{ from: 'auth-service', to: 'cache' }],
});
```

## API

### `DiagramBridge`

| Method | Description |
|---|---|
| `ingestMessage(msg)` | Feed a conversation message for context |
| `handleNodeClick(nodeId)` | Generate a follow-up prompt about a node |
| `feedStreamChunk(chunk)` | Feed streaming LLM output for progressive rendering |
| `patchSpec(patch)` | Apply incremental changes to the current spec |
| `getSpec()` | Get the current diagram spec |
| `reset()` | Clear all state |

### `SpecPatch`

| Field | Type | Description |
|---|---|---|
| `add` | `Node[]` | Nodes to add |
| `remove` | `string[]` | Node IDs to remove |
| `update` | `Partial<Node>[]` | Node property updates |
| `addEdges` | `Edge[]` | Edges to add |
| `removeEdges` | `{from, to}[]` | Edges to remove |

### Streaming

The streaming parser handles partially valid JSON gracefully. As the LLM streams a spec token-by-token, the bridge:

1. Buffers incoming tokens
2. Attempts to parse valid partial specs at each step
3. Renders whatever is valid so far (nodes without edges, partial containers)
4. Completes rendering when the full spec arrives

## Framework Agnostic

This package has no React dependency. It works with any LLM API and any frontend framework. The `onSpecUpdate` callback gives you a `DiagramSpec` object — render it however you want.

## Status

Early development. See the [project roadmap](https://github.com/SHA888/diagramma/blob/main/TODO.md) for details.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

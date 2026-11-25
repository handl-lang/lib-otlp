# lib-otlp

`handl-plugin-lib-otlp` — OpenTelemetry/OTLP module for the handl
runtime: the `.hnl` **contract** (`otlp.*` conn-as-value surface); the
native impl lives behind `otlp_c.*` ops (`lib-otlp-c`,
`lib-otlp-std`).

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the `otlp.*` cap/op contract |
| `src/otlp_module/tests.rs` | module tests (Rust side of the package) |
| `handl-packages/otlp/otlp.hnl` + `docs/otlp.yml` | the .hnl package + docs |
| `examples/`, `codemod.yaml` | examples / codemod config |
| `pkg.hnl`, `scripts/hnl-gate.sh`, `ci.sh` | manifest / gate / bupha entry |

## Contract

- Caps: `configure`, `start_span`, `end_span`, `set_attribute`,
  `add_event`, `record_log`, `record_metric`, `flush`, `shutdown`.
- `otlp_c` ops are STATEFUL: `configure(config)` sets the connection,
  `start_span(name)` returns the span id — conn is internal to the
  state. The `.hnl` contract exposes conn-as-value shape; conn is a
  token (the url) the arrows drop when forwarding.
- Payload records carry the backend op's positional args as named
  fields: `kv` = `{key, value}`, `ev` = `{name, attributes}`,
  `log` = `{level, message, attributes}`, `metric` = `{name, value,
  kind?, attributes?}`.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->

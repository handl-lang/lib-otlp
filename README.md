# lib-otlp

`handl-plugin-lib-otlp` — OpenTelemetry/OTLP module for the handl runtime.

The package ships in two layouts: the `handl-packages/otlp` package
(`otlp.hnl` + `docs/otlp.yml`) and the `src/` module tree
(`index.hnl`, `otlp_module/`).

## Layout

```text
handl-packages/otlp/   the .hnl package + docs
src/                   the module tree
ci.sh                  bupha entry → scripts/hnl-gate.sh
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

# Directory layout

```text
src
├── lib.rs                  // public API, re-exports
├── prelude.rs              // commonly used types/functions
├── models/
│   ├── mod.rs              // shared structs (State, Inputs, Region enum)
├── calculations/
│   ├── mod.rs              // dispatch + common helpers
│   └── region_models.rs    // private region-specific coeffs/impls
├── properties/
│   ├── mod.rs              // final property bundle builders
├── units/
│   └── ...                 // unit modules, pressure, temperature etc
├── errors.rs               // Errors
└── constants.rs            // Lookup tables from specification
```
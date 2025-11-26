# Steam Calculation Library

A work-in-progress Rust library for steam property calculations.

Following the **The International Association for the Properties of Water and Steam (IAPWS)** publication with reference **[IAPWS R7-97(2012)](https://iapws.org/public/documents/UWTF-/IF97-Rev.pdf)**

⚠️ **Work in Progress** - This library is currently under active development.

## Flowchart
```mermaid
flowchart TD

    A[Input: any of p, T, h, s, v] --> B[Validate inputs]
    B --> C{Enough data to determine state?}

    C -- No --> Z[Return error: insufficient info]

    C -- Yes --> D{Determine Region}
    
    D --> D1[Check boundaries\n R1-R5 ]
    D1 --> R1[Region 1 Liquid]
    D1 --> R2[Region 2 Vapor]
    D1 --> R3[Region 3 Dense/Supercritical]
    D1 --> R4[Region 4 Saturation]
    D1 --> R5[Region 5 High T Vapor]

    %% Region calc
    R1 --> E[Compute state properties]
    R2 --> E
    R3 --> E
    R4 --> E
    R5 --> E

    E --> F[Return: p, T, h, s, v, ρ, cp, cv, μ, k...]
```

## Description

This library provides thermodynamic property calculations for steam and water, useful for engineering applications involving steam systems.

## License
This library is licensed under the Apache License, Version 2.0.
Commercial and open-source use is permitted. See LICENSE for details.

## Contributing

Contributions are welcome.

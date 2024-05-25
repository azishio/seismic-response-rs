[English](README.md) | 日本語

# seismic-response

This module performs seismic response analysis for a single-degree-of-freedom system using the Newmark β method.
From the earthquake acceleration waveform, the absolute response acceleration can be obtained.

## Usage

```rust
use csv::Reader;
use crate::{ResponseAccAnalyzer, ResponseAccAnalyzerParams};

fn example() {
    let mut csv = Reader::from_path("benches/seismic_acc_waveform.csv").unwrap();
    let data = csv.deserialize::<f64>().map(|x| x.unwrap()).collect::<Vec<_>>();

    let params = ResponseAccAnalyzerParams {
        // Natural period [ms]
        natural_period_ms: 500,
        // Time increment of input acceleration waveform [ms]
        dt_ms: 10,
        // Damping coefficient
        damping_h: 0.05,
        // β for Newmark's β method
        beta: 0.25,
        // Initial response displacement [m]
        init_x: 0.0,
        // Initial response velocity [m/s]
        init_v: 0.0,
        // Initial response acceleration [gal]
        init_a: 0.0,
        // Initial input acceleration [gal]
        init_xg: 0.0,
    };

    let analyzer = ResponseAccAnalyzer::from_params(params);

    let result: Vec<f64> = analyzer.analyze(data);
}
```

## Formulas

This program is implemented based on the following formulas.

### Stiffness Coefficient

The stiffness coefficient \( k \) is calculated based on the mass \( m \) and the natural period in milliseconds \( T_
{\text{ms}} \):

$$
k = \frac{4 \pi^2 m}{\left(\frac{T_{\text{ms}}}{1000}\right)^2}
$$

### Damping Coefficient

The damping coefficient \( c \) is calculated based on the damping constant \( h \), mass \( m \), and stiffness
coefficient \( k \):

$$
c = 2h\sqrt{km}
$$

### Step-by-Step Calculation

#### Response Acceleration

The acceleration \( a_{n+1} \) for the next step is calculated as:

$$
a_{n+1} = \frac{p_{n+1} - c\left(v_n + \frac{\Delta t}{2}a_n\right) - k\left(x_n + \Delta t v_n + \left(\frac{1}{2} -
\beta\right)\Delta t^2 a_n\right)}{m + \frac{\Delta t}{2}c + \beta \Delta t^2 k}
$$

where the external force \( p_{n+1} \) is given by:

$$
p_{n+1} = -xg_{n+1} m
$$

#### Response Velocity

The velocity \( v_{n+1} \) for the next step is calculated as:

$$
v_{n+1} = v_n + \frac{\Delta t}{2}(a_n + a_{n+1})
$$

#### Response Displacement

The displacement \( x_{n+1} \) for the next step is calculated as:

$$
x_{n+1} = x_n + \Delta t v_n + \left(\frac{1}{2} - \beta\right) \Delta t^2 a_n + \beta \Delta t^2 a_{n+1}
$$

### Absolute Response Acceleration

The final absolute response acceleration \( a_{\text{abs}} \) is calculated as:

$$
a_{\text{abs}} = a + xg
$$

These are the primary calculations implemented within the program.

> [!NOTE]
> Although the formulas treat the mass \( m \) as a variable, in the actual program, calculations are performed assuming
> a mass of 1. This is because the mass does not affect the absolute response acceleration. This is verified in the test
> code within the documentation.

## License

Licensed under either of

+ Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

(The English documentation comments and README file have been translated using DeepL and ChatGPT.)

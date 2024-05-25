English | [日本語](README.ja.md)

# seismic-response

This crate performs seismic response analysis of a single-degree-of-freedom system using Newmark's β method.
It can calculate the absolute response acceleration from the seismic acceleration waveform.

If your goal is to perform seismic response analysis, you can also use
the [calculation site](https://github.com/azishio/seismic-response-web) implemented using the wasm version of this
crate.

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
        // Time increment of the input acceleration waveform [ms]
        dt_ms: 10,
        // Damping constant
        damping_h: 0.05,
        // β of Newmark's β method
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

## WebAssembly

This program is published on npm as an [npm package](https://www.npmjs.com/package/seismic-response).
It can be used in the same way as the Rust crate.

## Equations

This program is implemented based on the following equations.

### Stiffness Coefficient

Calculate the stiffness coefficient \( k \) based on the mass \( m \) and the natural period in milliseconds \( T_
{\text{ms}} \):

$$
k = \frac{4 \pi^2 m}{\left(\frac{T_{\text{ms}}}{1000}\right)^2}
$$

### Damping Coefficient

Calculate the damping coefficient \( c \) based on the damping constant \( h \), mass \( m \), and stiffness
coefficient \( k \):

$$
c = 2h\sqrt{km}
$$

### Step-by-Step Calculation

#### Response Acceleration

Calculate the acceleration at the next step \( a_{n+1} \):

$$
a_{n+1} = \frac{p_{n+1} - c\left(v_n + \frac{\Delta t}{2}a_n\right) - k\left(x_n + \Delta t v_n + \left(\frac{1}{2} -
\beta\right)\Delta t^2 a_n\right)}{m + \frac{\Delta t}{2}c + \beta \Delta t^2 k}
$$

Here, the external force \( p_{n+1} \) is given by:

$$
p_{n+1} = -xg_{n+1} m
$$

#### Response Velocity

Calculate the velocity at the next step \( v_{n+1} \):

$$
v_{n+1} = v_n + \frac{\Delta t}{2}(a_n + a_{n+1})
$$

#### Response Displacement

Calculate the displacement at the next step \( x_{n+1} \):

$$
x_{n+1} = x_n + \Delta t v_n + \left(\frac{1}{2} - \beta\right) \Delta t^2 a_n + \beta \Delta t^2 a_{n+1}
$$

### Absolute Response Acceleration

Calculate the final absolute response acceleration \( a_{\text{abs}} \):

$$
a_{\text{abs}} = a + xg
$$

These are the main equations implemented within the program.

> [!NOTE]
> In these equations, the mass \( m \) is treated as a variable, but in the actual program, calculations are performed
> assuming a mass of 1.
> This is because the mass does not affect the absolute response acceleration.
> This can be confirmed in the test code within the documentation.

## License

Licensed under either of the following, at your option:

+ Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

(The English in the documentation comments and README file has been translated using DeepL and ChatGPT.)

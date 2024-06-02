English | [日本語](README.ja.md)

# Seismic Response

Using Newmark's β method, we will perform seismic response analysis of a single-degree-of-freedom (SDOF) system. From
the seismic acceleration waveform, we can determine the response displacement, response velocity, response acceleration,
and absolute response acceleration.

To conduct seismic response analysis, you can use
the [calculation site](https://github.com/azishio/seismic-response-web) implemented using the WASM version of this
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
        // Time step of the input acceleration waveform [ms]
        dt_ms: 10,
        // Damping ratio
        damping_h: 0.05,
        // Newmark's β method parameter
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

    let result: Result = analyzer.analyze(data);
    // struct Result {
    //     /// Response displacement [m]
    //     pub x: Vec<f64>,

    //     /// Response velocity [m/s]
    //     pub v: Vec<f64>,

    //     /// Response acceleration [gal]
    //     pub a: Vec<f64>,

    //     /// Absolute response acceleration [gal]
    //     pub abs_acc: Vec<f64>,
    // }
}
```

## WebAssembly

This program is published as an [npm package](https://www.npmjs.com/package/seismic-response). It can be used similarly
to the Rust crate.

## Mathematical Formulas

This program is implemented based on the following formulas:

### Stiffness Coefficient

The stiffness coefficient \( k \) is calculated based on the mass \( m \) and the natural period in milliseconds \( T_
{\text{ms}} \):

$$
k = \frac{4 \pi^2 m}{\left(\frac{T_{\text{ms}}}{1000}\right)^2}
$$

### Damping Coefficient

The damping coefficient \( c \) is calculated based on the damping ratio \( h \), the mass \( m \), and the stiffness
coefficient \( k \):

$$
c = 2h\sqrt{km}
$$

### Step-by-Step Calculation

#### Response Acceleration

The acceleration at the next step \( a_{n+1} \) is calculated as:

$$
a_{n+1} = \frac{p_{n+1} - c\left(v_n + \frac{\Delta t}{2}a_n\right) - k\left(x_n + \Delta t v_n + \left(\frac{1}{2} -
\beta\right)\Delta t^2 a_n\right)}{m + \frac{\Delta t}{2}c + \beta \Delta t^2 k}
$$

Here, the external force \( p_{n+1} \) is given by:

$$
p_{n+1} = -xg_{n+1} m
$$

#### Response Velocity

The velocity at the next step \( v_{n+1} \) is calculated as:

$$
v_{n+1} = v_n + \frac{\Delta t}{2}(a_n + a_{n+1})
$$

#### Response Displacement

The displacement at the next step \( x_{n+1} \) is calculated as:

$$
x_{n+1} = x_n + \Delta t v_n + \left(\frac{1}{2} - \beta\right) \Delta t^2 a_n + \beta \Delta t^2 a_{n+1}
$$

### Absolute Response Acceleration

The final absolute response acceleration \( a_{\text{abs}} \) is calculated as:

$$
a_{\text{abs}} = a + xg
$$

These are the main calculations implemented in the program.

> **Note:**
> While the formulas here treat mass \( m \) as a variable, the actual program calculates assuming mass is 1. This is
> because mass does not affect the absolute response acceleration. This can be confirmed in the test code within the
> documentation.

## License

Licensed under either of the following licenses:

+ Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

(Documentation comments and README file translations provided by DeepL and ChatGPT.)

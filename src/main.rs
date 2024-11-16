use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct BoxDim {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// impl BoxDim {
//     pub fn to_js_value(&self) -> JsValue {
//         JsValue::to_value(self).unwrap()
//     }
// }


pub fn main(){
    // Example usage
let box_dims = vec![
    BoxDim {
        x: 0.0,
        y: 0.0,
        width: 360.16936723459753,
        height: 355.38835793502324,
      
    },
    BoxDim {
        x: 360.16936723459753,
        y: 0.0,
        width: 291.85301406389794,
        height: 438.5769337025789,
     
    },
    BoxDim {
        x: 360.16936723459753,
        y: 438.5769337025789,
        width: 453.1870817592292,
        height: 282.44406151895623
    },
];

// let js_value = JsValue::from_serde(&box_dims).unwrap();
}

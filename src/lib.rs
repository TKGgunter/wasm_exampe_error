//wasm-pack build --target web

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
const WIDTH : usize = 600; 
const HEIGHT : usize = 600;

#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

#[wasm_bindgen]
pub fn testing(){
//NOTE this is the function of interest.
//It does not matter if we set the capacity of the fector
    let mut v : Vec<f32>= Vec::with_capacity(10);
    v.push(0f32);
    v.push(0f32);
    v.push(0f32); //NOTE: This push creates the error. If this is commented it out there would be no error.
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue>{

    //testing(); //NOTE this is the function of interest
                 //if uncommented the example works

    Ok(())
}





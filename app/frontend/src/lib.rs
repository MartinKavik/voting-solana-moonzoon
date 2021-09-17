use zoon::*;

// ------ ------
//    States
// ------ ------

// ------ ------
//    Signals
// ------ ------

// ------ ------
//   Commands
// ------ ------

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Text::new("voting")
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}

use wasm_bindgen::prelude::*;
mod util;

#[wasm_bindgen]
pub fn gen(seed: i32) -> String {
    util::gen(seed as u64).to_string()
    // String::new()
}

#[wasm_bindgen(getter_with_clone)]
pub struct Ret {
    pub score: i64,
    pub err: String,
    pub svg: String,
}


#[wasm_bindgen]
pub fn vis(_input: String, _output: String, turn: usize) -> Ret {
    let input = util::parse_input(&_input);
    let score = turn;
    
    let output = util::parse_output(&input,&_output);

    // Ret {
    //     score: score as i64,
    //     err: String::new(),
    //     svg: String::new(),

    // }
    let (score, err, svg) = match output {
        Ok(output) => util::vis(&input, &output, turn),
        Err(err) => (0, err, String::new()),
    };

    Ret {
        score: score as i64,
        err,
        svg,
    }
}

#[wasm_bindgen]
pub fn get_max_turn(_input: String, _output: String) -> usize {
    let input = util::parse_input(&_input);

    let result = util::parse_output(&input, &_output);

    match result {
        Ok(output) => output.out.len(),
        Err(_) => 0,
    }
    
}

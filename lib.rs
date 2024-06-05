use obi::{OBIDecode, OBIEncode, OBISchema};
use owasm_kit::{execute_entry_point, prepare_entry_point, oei, ext};

#[derive(OBIDecode, OBISchema)]
struct Input {
    url: String,
    asset: String
}

#[derive(OBIEncode, OBISchema)]
struct Output {
    price: String
}

const DATA_SOURCE_ID: i64 = 821;
const EXTERNAL_ID: i64 = 0;

#[no_mangle]
fn prepare_impl(input: Input) {
    oei::ask_external_data(
        EXTERNAL_ID,
        DATA_SOURCE_ID,
        format!("{} {}", input.url, input.asset).as_bytes(),
    )
}

#[no_mangle]
fn execute_impl(_input: Input) -> Output {
    Output {
        price: ext::load_majority::<String>(EXTERNAL_ID).unwrap(),
    }
}

prepare_entry_point!(prepare_impl);
execute_entry_point!(execute_impl);
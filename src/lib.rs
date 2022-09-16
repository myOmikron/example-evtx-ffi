use std::ptr::slice_from_raw_parts;

use evtx::EvtxParser;

#[no_mangle]
pub extern "C" fn parse_evtx_from_buff(buffer: *const u8, size: usize) {
    let f = unsafe {
        slice_from_raw_parts(buffer, size)
            .as_ref()
            .unwrap()
            .to_vec()
    };

    let foo = EvtxParser::from_buffer(f).expect("Buffer could not be parsed");
    for record in foo.into_chunks() {
        match record {
            Ok(data) => unsafe {
                todo!("Implement further processing");
            },
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
}

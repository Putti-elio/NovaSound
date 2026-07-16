mod generated {
    #![allow(warnings)]
    #![allow(clippy::all)]

    include!(concat!(env!("OUT_DIR"), "/_connectrpc.rs"));
}

pub use generated::*;

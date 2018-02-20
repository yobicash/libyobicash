// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `server` module provides the base transport server implementation for yobicash network

use network::transports::base::transport_session::*;


pub struct BaseServer {
}

impl BaseServer {
     /// Creates a new `BaseServer`
     pub fn new() -> BaseServer {
         let retval = BaseServer{}; // we pretty much never fail
         return retval;
     }

     pub fn listen_on(endpoint: String) -> Result<bool,String>{
         // in a real implementation, we'd bind to the specified endpoint and begin to listen
         Ok(true)
     }

     pub fn accept() -> Result<BaseTransportSession,String> {
         // If there's a new connection waiting, we accept it and return a new session
         let retval = BaseTransportSession{remote_endpoint: String::from("localhost:1337"),
                                       is_open: true};
         Ok(retval)
     }
}

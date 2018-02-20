// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `client` module provides the base transport client implementation for yobicash network

use network::transports::base::transport_session::BaseTransportSession;

pub struct BaseClient {
}

impl BaseClient {
     /// Creates a new `BaseClient`
     pub fn new() -> BaseClient {
         let retval = BaseClient{}; // we pretty much never fail
         return retval;
     }

     // TODO - define some standard errors here, instead of using String
     pub fn connect_to(endpoint: String) -> Result<BaseTransportSession,String> {
         let retval = BaseTransportSession  {remote_endpoint: endpoint,
                                             is_open: true};
         Ok(retval)
     }
}

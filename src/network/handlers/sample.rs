// Copyright 2018 Yobicash Ltd.
//
// Licensed under the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>
// and the Apache 2.0 license <LICENSE-APACHE or https://opensource.org/licenses/Apache-2.0>.
// This file may not be copied, modified, or distributed except according to those
// terms.

//! The `sample` module provides the Yobicash network sample handler type and methods.

use error::ErrorKind;
use result::Result;
use traits::Validate;
use crypto::BinarySerialize as CryptoBinarySerialize;
use store::Store;
use node::Node;
use network::session::Session;
use network::resource_type::ResourceType;
use network::message::{Message, SampleRequest, SampleResponse};

/// The type used for handling `Sample` messages.
#[derive(Clone, Debug)]
pub struct SampleHandler;

impl SampleHandler {
    /// Handles a sample request.
    pub fn handle<S: Store>(node: &mut Node<S>, session: &Session, req: &SampleRequest) -> Result<Message> {
        session.validate()?;

        if req.id != session.id {
            return Err(ErrorKind::InvalidID.into());
        }

        if req.network_type != session.network_type {
            return Err(ErrorKind::InvalidSession.into());
        }

        req.validate()?;

        let resource_type = req.resource_type;

        match resource_type {
            ResourceType::Peer => {
                let count = req.count;
                let addresses = node.sample_peers(count)?;
              
                let mut resources = Vec::new();

                for address in addresses {
                    resources.push(address.into_bytes());
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::Transaction => {
                let count = req.count;
                let ids = node.sample_transactions(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::WriteOp => {
                let count = req.count;
                let ids = node.sample_write_ops(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::UnspentCoin => {
                let count = req.count;
                let ids = node.sample_unspent_coins(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::SpentCoin => {
                let count = req.count;
                let ids = node.sample_spent_coins(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::UnspentOutput => {
                let count = req.count;
                let ids = node.sample_unspent_outputs(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::SpentOutput => {
                let count = req.count;
                let ids = node.sample_spent_outputs(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
            ResourceType::UndeletedData => {
                let count = req.count;
                let ids = node.sample_undeleted_data(count)?;
              
                let mut resources = Vec::new();

                for id in ids {
                    resources.push(id.to_bytes()?);
                }

                let res = SampleResponse::new(session, resource_type, &resources)?;

                let message = Message::SampleResponse(res);

                Ok(message)
            },
        }
    }
}

// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{error::Error, payload::transaction::Output, MessageId};

use bee_common::packable::{Packable, Read, Write};

use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct CreatedOutput {
    message_id: MessageId,
    inner: Output,
}

impl Deref for CreatedOutput {
    type Target = Output;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl CreatedOutput {
    pub fn new(message_id: MessageId, inner: Output) -> Self {
        Self { message_id, inner }
    }

    pub fn message_id(&self) -> &MessageId {
        &self.message_id
    }

    pub fn inner(&self) -> &Output {
        &self.inner
    }
}

impl Packable for CreatedOutput {
    type Error = Error;

    fn packed_len(&self) -> usize {
        self.message_id.packed_len() + self.inner.packed_len()
    }

    fn pack<W: Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        self.message_id.pack(writer)?;
        self.inner.pack(writer)?;

        Ok(())
    }

    fn unpack<R: Read + ?Sized>(reader: &mut R) -> Result<Self, Self::Error> {
        Ok(Self {
            message_id: MessageId::unpack(reader)?,
            inner: Output::unpack(reader)?,
        })
    }
}

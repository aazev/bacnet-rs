use crate::application::*;
use crate::{Decode, Encode};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;
use std::convert::TryFrom;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use tracing::trace;

/// Network Layer PDU Message Priority (6.2.2)
#[derive(Copy, Clone, Debug, Eq, PartialEq, FromPrimitive, ToPrimitive)]
pub enum NPDUPriority {
    LifeSafety = 0b11,
    CriticalEquipment = 0b10,
    Urgent = 0b01,
    Normal = 0b00,
}

impl From<NPDUPriority> for u8 {
    fn from(val: NPDUPriority) -> Self {
        match val {
            NPDUPriority::LifeSafety => 0b11,
            NPDUPriority::CriticalEquipment => 0b10,
            NPDUPriority::Urgent => 0b01,
            NPDUPriority::Normal => 0b00,
        }
    }
}

impl Default for NPDUPriority {
    fn default() -> Self {
        Self::Normal
    }
}

/// Network Layer PDU Message Type (6.2.4)
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NPDUMessage {
    WhoIsRouterToNetwork,          // = 0x00,
    IAmRouterToNetwork,            // = 0x01,
    ICouldBeRouterToNetwork,       // = 0x02,
    RejectMessageToNetwork,        // = 0x03,
    RouterBusyToNetwork,           // = 0x04,
    RouterAvailableToNetwork,      // = 0x05,
    InitializeRoutingTable,        // = 0x06,
    InitializeRoutingTableAck,     // = 0x07,
    EstablishConnectionToNetwork,  // = 0x08,
    DisconnectConnectionToNetwork, // = 0x09,
    ChallengeRequest,              // = 0x0A,
    SecurityPayload,               // = 0x0B,
    SecurityResponse,              // = 0x0C,
    RequestKeyUpdate,              // = 0x0D,
    UpdateKeySet,                  // = 0x0E,
    UpdateDistributionKey,         // = 0x0F,
    RequestMasterKey,              // = 0x10,
    SetMasterKey,                  // = 0x11,
    WhatIsNetworkNumber,           // = 0x12,
    NetworkNumberIs,               // = 0x13,
    Proprietary(u8),               // = 0x80 to 0xFF, Available for vendor proprietary messages
    Reserved(u8),                  // = 0x14 to 0x7F, Reserved for use by ASHRAE
}

impl TryFrom<u8> for NPDUMessage {
    type Error = String;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(Self::WhoIsRouterToNetwork),
            0x01 => Ok(Self::IAmRouterToNetwork),
            0x02 => Ok(Self::ICouldBeRouterToNetwork),
            0x03 => Ok(Self::RejectMessageToNetwork),
            0x04 => Ok(Self::RouterBusyToNetwork),
            0x05 => Ok(Self::RouterAvailableToNetwork),
            0x06 => Ok(Self::InitializeRoutingTable),
            0x07 => Ok(Self::InitializeRoutingTableAck),
            0x08 => Ok(Self::EstablishConnectionToNetwork),
            0x09 => Ok(Self::DisconnectConnectionToNetwork),
            0x0A => Ok(Self::ChallengeRequest),
            0x0B => Ok(Self::SecurityPayload),
            0x0C => Ok(Self::SecurityResponse),
            0x0D => Ok(Self::RequestKeyUpdate),
            0x0E => Ok(Self::UpdateKeySet),
            0x0F => Ok(Self::UpdateDistributionKey),
            0x10 => Ok(Self::RequestMasterKey),
            0x11 => Ok(Self::SetMasterKey),
            0x12 => Ok(Self::WhatIsNetworkNumber),
            0x13 => Ok(Self::NetworkNumberIs),
            0x14..=0x7F => Ok(Self::Reserved(v)),
            0x80..=0xFF => Ok(Self::Proprietary(v)),
        }
    }
}

impl Encode for NPDUMessage {
    fn encode<T: std::io::Write + Sized>(&self, writer: &mut T) -> std::io::Result<()> {
        let _: () = match self {
            Self::WhoIsRouterToNetwork => writer.write_u8(0x00)?,
            Self::IAmRouterToNetwork => writer.write_u8(0x01)?,
            Self::ICouldBeRouterToNetwork => writer.write_u8(0x02)?,
            Self::RejectMessageToNetwork => writer.write_u8(0x03)?,
            Self::RouterBusyToNetwork => writer.write_u8(0x04)?,
            Self::RouterAvailableToNetwork => writer.write_u8(0x05)?,
            Self::InitializeRoutingTable => writer.write_u8(0x06)?,
            Self::InitializeRoutingTableAck => writer.write_u8(0x07)?,
            Self::EstablishConnectionToNetwork => writer.write_u8(0x08)?,
            Self::DisconnectConnectionToNetwork => writer.write_u8(0x09)?,
            Self::ChallengeRequest => writer.write_u8(0x0A)?,
            Self::SecurityPayload => writer.write_u8(0x0B)?,
            Self::SecurityResponse => writer.write_u8(0x0C)?,
            Self::RequestKeyUpdate => writer.write_u8(0x0D)?,
            Self::UpdateKeySet => writer.write_u8(0x0E)?,
            Self::UpdateDistributionKey => writer.write_u8(0x0F)?,
            Self::RequestMasterKey => writer.write_u8(0x10)?,
            Self::SetMasterKey => writer.write_u8(0x11)?,
            Self::WhatIsNetworkNumber => writer.write_u8(0x12)?,
            Self::NetworkNumberIs => writer.write_u8(0x13)?,
            Self::Reserved(v) => writer.write_u8(*v)?,
            Self::Proprietary(v) => writer.write_u8(*v)?,
        };
        Ok(())
    }

    fn len(&self) -> usize {
        match self {
            Self::WhoIsRouterToNetwork => 2,
            Self::IAmRouterToNetwork => 2,
            Self::ICouldBeRouterToNetwork => 2,
            Self::RejectMessageToNetwork => 2,
            Self::RouterBusyToNetwork => 2,
            Self::RouterAvailableToNetwork => 2,
            Self::InitializeRoutingTable => 2,
            Self::InitializeRoutingTableAck => 2,
            Self::EstablishConnectionToNetwork => 2,
            Self::DisconnectConnectionToNetwork => 2,
            Self::ChallengeRequest => 2,
            Self::SecurityPayload => 2,
            Self::SecurityResponse => 2,
            Self::RequestKeyUpdate => 2,
            Self::UpdateKeySet => 2,
            Self::UpdateDistributionKey => 2,
            Self::RequestMasterKey => 2,
            Self::SetMasterKey => 2,
            Self::WhatIsNetworkNumber => 2,
            Self::NetworkNumberIs => 2,
            Self::Reserved(_) => 2,
            Self::Proprietary(_) => 2,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NPDUDest {
    net: u16,
    adr: Vec<u8>,
    hops: u8,
}

impl NPDUDest {
    pub fn new(net: u16, capacity: usize) -> Self {
        NPDUDest {
            net,
            adr: Vec::with_capacity(capacity),
            hops: 255,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct NPDUSource {
    net: u16,
    adr: Vec<u8>,
}

impl NPDUSource {
    pub fn new(net: u16, capacity: usize) -> Self {
        NPDUSource {
            net,
            adr: Vec::with_capacity(capacity),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NPDUContent<A: Encode = APDU, B: Encode = NPDUMessage> {
    APDU(A),
    Message(B),
}

impl<A: Encode, B: Encode> From<A> for NPDUContent<A, B> {
    fn from(apdu: A) -> Self {
        NPDUContent::APDU(apdu)
    }
}

impl<A: Encode, B: Encode> Encode for NPDUContent<A, B> {
    fn encode<T: std::io::Write + Sized>(&self, writer: &mut T) -> std::io::Result<()> {
        let _: () = match self {
            Self::APDU(apdu) => apdu.encode(writer)?,
            Self::Message(msg) => msg.encode(writer)?,
        };
        Ok(())
    }

    fn len(&self) -> usize {
        match self {
            Self::APDU(apdu) => apdu.len(),
            Self::Message(msg) => msg.len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NPDU<A: Encode = APDU, B: Encode = NPDUMessage> {
    /// Protocol Version Number (6.2.1)
    pub version: u8,
    pub destination: Option<NPDUDest>,
    pub source: Option<NPDUSource>,
    pub data_expecting_reply: bool,
    pub priority: NPDUPriority,
    pub content: NPDUContent<A, B>,
}

impl<A: Encode, B: Encode> NPDU<A, B> {
    pub fn new<T: Into<NPDUContent<A, B>>>(
        content: T,
        destination: Option<NPDUDest>,
        source: Option<NPDUSource>,
        priority: NPDUPriority,
    ) -> Self {
        NPDU {
            version: 1,
            content: content.into(),
            destination,
            source,
            data_expecting_reply: false,
            priority,
        }
    }
}

impl<A: Encode, B: Encode> Encode for NPDU<A, B> {
    fn encode<T: std::io::Write + Sized>(&self, writer: &mut T) -> std::io::Result<()> {
        // NPCI
        writer.write_u8(self.version)?;

        let mut control: u8 = self.priority.into();
        if self.data_expecting_reply {
            control |= 1 << 2;
        }
        if self.source.is_some() {
            control |= 1 << 3;
        }
        if self.destination.is_some() {
            control |= 1 << 5;
        }
        if let NPDUContent::Message(_) = self.content {
            control |= 1 << 7;
        }
        writer.write_u8(control)?;
        if let Some(ref d) = self.destination {
            writer.write_u16::<BigEndian>(d.net)?;
            writer.write_u8(d.adr.len() as u8)?;
            writer.write_all(&d.adr)?;
        }
        if let Some(ref s) = self.source {
            writer.write_u16::<BigEndian>(s.net)?;
            writer.write_u8(s.adr.len() as u8)?;
            writer.write_all(&s.adr)?;
        }
        if let Some(ref d) = self.destination {
            writer.write_u8(d.hops)?;
        }

        // Content
        self.content.encode(writer)?;

        Ok(())
    }

    fn len(&self) -> usize {
        let mut l: usize = 0;
        l += 1; // Version
        l += 1; // Control
        l += self
            .destination
            .as_ref()
            .map(|d| 2 + 1 + d.adr.len() + 1)
            .unwrap_or(0); // DNET(2) + DLEN(1) + DADR(*) + HOPS(1)
        l += self
            .source
            .as_ref()
            .map(|s| 2 + 1 + s.adr.len())
            .unwrap_or(0); // SNET(2) + SLEN(1) + SADR(*)
        l += self.content.len();
        l
    }
}

impl Decode for NPDU {
    fn decode<T: std::io::Read + Sized>(reader: &mut T) -> std::io::Result<Self> {
        let version = reader.read_u8()?;
        trace!("Version: {:02x}", version);
        // Read and parse the Network Layer Protocol Control Information (6.2.2)
        let control = reader.read_u8()?;
        trace!("Control: {:08b}", control);
        let priority = NPDUPriority::from_u8(control & 0b0000_00011).unwrap();
        let has_apdu = (control & 1 << 7) == 0;
        let has_dest = (control & 1 << 5) != 0;
        let has_source = (control & 1 << 3) != 0;
        let data_expecting_reply = (control & 1 << 2) != 0;

        let mut destination: Option<NPDUDest> = if has_dest {
            let net = reader.read_u16::<BigEndian>()?;
            let len = reader.read_u8()?;
            let mut dest = NPDUDest::new(net, len as usize);
            reader.read_exact(&mut dest.adr)?;
            Some(dest)
        } else {
            None
        };

        let source: Option<NPDUSource> = if has_source {
            let net = reader.read_u16::<BigEndian>()?;
            let len = reader.read_u8()?;
            let mut source = NPDUSource::new(net, len as usize);
            reader.read_exact(&mut source.adr)?;
            Some(source)
        } else {
            None
        };
        //println!("{:?}", destination);
        if let Some(dest) = &mut destination {
            dest.hops = reader.read_u8()?;
        };

        let content = if has_apdu {
            APDU::decode(reader)?.into()
        } else {
            /*Ok(NPDUContentSlice::Message(NPDUMessage::try_from(
                self.slice[0],
            )?))*/
            unimplemented!();
        };

        Ok(Self {
            version,
            destination,
            source,
            data_expecting_reply,
            priority,
            content,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Encode;
    use bytes::{BufMut, BytesMut};

    use crate::tests::*;

    #[test]
    fn test_encode_npdu() {
        let content = NPDUContent::<Dummy, Dummy>::APDU(Dummy::default());
        let npdu = NPDU::<Dummy, Dummy>::new(content, None, None, NPDUPriority::Normal);

        let mut w = BytesMut::new().writer();
        npdu.encode(&mut w).expect("Write NPDU to buffer");
        assert_eq!(w.into_inner().to_vec(), vec![1, 0]);
    }

    #[test]
    fn test_encode_npdu_with_dest() {
        let content = NPDUContent::<Dummy, Dummy>::APDU(Dummy::default());
        let dest = NPDUDest {
            net: 0x126,
            adr: vec![0; 16],
            hops: 255,
        };
        let npdu = NPDU::<Dummy, Dummy>::new(content, Some(dest), None, NPDUPriority::Normal);

        let mut w = BytesMut::new().writer();
        npdu.encode(&mut w).expect("Write NPDU to buffer");
        assert_eq!(
            w.into_inner().to_vec(),
            vec![
                1, 32, 1, 38, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255
            ]
        );
    }

    #[test]
    fn test_encode_npdu_with_source() {
        let content = NPDUContent::<Dummy, Dummy>::APDU(Dummy::default());
        let source = NPDUSource {
            net: 0x126,
            adr: vec![0; 16],
        };
        let npdu = NPDU::<Dummy, Dummy>::new(content, None, Some(source), NPDUPriority::Normal);

        let mut w = BytesMut::new().writer();
        npdu.encode(&mut w).expect("Write NPDU to buffer");
        assert_eq!(
            w.into_inner().to_vec(),
            vec![
                1, 8, 1, 38, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_encode_npdu_with_dest_and_source() {
        let content = NPDUContent::<Dummy, Dummy>::APDU(Dummy::default());
        let dest = NPDUDest {
            net: 0x126,
            adr: vec![0; 16],
            hops: 255,
        };
        let source = NPDUSource {
            net: 0x126,
            adr: vec![0; 16],
        };
        let npdu =
            NPDU::<Dummy, Dummy>::new(content, Some(dest), Some(source), NPDUPriority::Normal);

        let mut w = BytesMut::with_capacity(1024).writer();
        npdu.encode(&mut w).expect("Write NPDU to buffer");
        assert_eq!(
            w.into_inner().to_vec(),
            vec![
                1, 40, 1, 38, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 38, 16, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255
            ]
        );
    }
}

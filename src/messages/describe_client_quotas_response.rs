//! DescribeClientQuotasResponse
//!
//! See the schema for this message [here](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/DescribeClientQuotasResponse.json).
// WARNING: the items of this module are generated and should not be edited directly
#![allow(unused)]

use std::borrow::Borrow;
use std::collections::BTreeMap;

use bytes::Bytes;
use log::error;
use uuid::Uuid;

use crate::protocol::{
    Encodable, Decodable, MapEncodable, MapDecodable, Encoder, Decoder, EncodeError, DecodeError, Message, HeaderVersion, VersionRange,
    types, write_unknown_tagged_fields, compute_unknown_tagged_fields_size, StrBytes, buf::{ByteBuf, ByteBufMut}, Builder
};


/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct EntityData {
    /// The entity type.
    /// 
    /// Supported API versions: 0-1
    pub entity_type: StrBytes,

    /// The entity name, or null if the default.
    /// 
    /// Supported API versions: 0-1
    pub entity_name: Option<StrBytes>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for EntityData {
    type Builder = EntityDataBuilder;

    fn builder() -> Self::Builder{
        EntityDataBuilder::default()
    }
}

impl Encodable for EntityData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactString.encode(buf, &self.entity_type)?;
        } else {
            types::String.encode(buf, &self.entity_type)?;
        }
        if version >= 1 {
            types::CompactString.encode(buf, &self.entity_name)?;
        } else {
            types::String.encode(buf, &self.entity_name)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.entity_type)?;
        } else {
            total_size += types::String.compute_size(&self.entity_type)?;
        }
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.entity_name)?;
        } else {
            total_size += types::String.compute_size(&self.entity_name)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for EntityData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let entity_type = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let entity_name = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            entity_type,
            entity_name,
            unknown_tagged_fields,
        })
    }
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_type: Default::default(),
            entity_name: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for EntityData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct ValueData {
    /// The quota configuration key.
    /// 
    /// Supported API versions: 0-1
    pub key: StrBytes,

    /// The quota configuration value.
    /// 
    /// Supported API versions: 0-1
    pub value: f64,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for ValueData {
    type Builder = ValueDataBuilder;

    fn builder() -> Self::Builder{
        ValueDataBuilder::default()
    }
}

impl Encodable for ValueData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactString.encode(buf, &self.key)?;
        } else {
            types::String.encode(buf, &self.key)?;
        }
        types::Float64.encode(buf, &self.value)?;
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.key)?;
        } else {
            total_size += types::String.compute_size(&self.key)?;
        }
        total_size += types::Float64.compute_size(&self.value)?;
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for ValueData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let key = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let value = types::Float64.decode(buf)?;
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            key,
            value,
            unknown_tagged_fields,
        })
    }
}

impl Default for ValueData {
    fn default() -> Self {
        Self {
            key: Default::default(),
            value: 0.0,
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for ValueData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct EntryData {
    /// The quota entity description.
    /// 
    /// Supported API versions: 0-1
    pub entity: Vec<EntityData>,

    /// The quota values for the entity.
    /// 
    /// Supported API versions: 0-1
    pub values: Vec<ValueData>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for EntryData {
    type Builder = EntryDataBuilder;

    fn builder() -> Self::Builder{
        EntryDataBuilder::default()
    }
}

impl Encodable for EntryData {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.entity)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.entity)?;
        }
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.values)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.values)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.entity)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.entity)?;
        }
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.values)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.values)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for EntryData {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let entity = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let values = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            entity,
            values,
            unknown_tagged_fields,
        })
    }
}

impl Default for EntryData {
    fn default() -> Self {
        Self {
            entity: Default::default(),
            values: Default::default(),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for EntryData {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

/// Valid versions: 0-1
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, derive_builder::Builder)]
pub struct DescribeClientQuotasResponse {
    /// The duration in milliseconds for which the request was throttled due to a quota violation, or zero if the request did not violate any quota.
    /// 
    /// Supported API versions: 0-1
    pub throttle_time_ms: i32,

    /// The error code, or `0` if the quota description succeeded.
    /// 
    /// Supported API versions: 0-1
    pub error_code: i16,

    /// The error message, or `null` if the quota description succeeded.
    /// 
    /// Supported API versions: 0-1
    pub error_message: Option<StrBytes>,

    /// A result entry.
    /// 
    /// Supported API versions: 0-1
    pub entries: Option<Vec<EntryData>>,

    /// Other tagged fields
    pub unknown_tagged_fields: BTreeMap<i32, Vec<u8>>,
}

impl Builder for DescribeClientQuotasResponse {
    type Builder = DescribeClientQuotasResponseBuilder;

    fn builder() -> Self::Builder{
        DescribeClientQuotasResponseBuilder::default()
    }
}

impl Encodable for DescribeClientQuotasResponse {
    fn encode<B: ByteBufMut>(&self, buf: &mut B, version: i16) -> Result<(), EncodeError> {
        types::Int32.encode(buf, &self.throttle_time_ms)?;
        types::Int16.encode(buf, &self.error_code)?;
        if version >= 1 {
            types::CompactString.encode(buf, &self.error_message)?;
        } else {
            types::String.encode(buf, &self.error_message)?;
        }
        if version >= 1 {
            types::CompactArray(types::Struct { version }).encode(buf, &self.entries)?;
        } else {
            types::Array(types::Struct { version }).encode(buf, &self.entries)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            types::UnsignedVarInt.encode(buf, num_tagged_fields as u32)?;

            write_unknown_tagged_fields(buf, 0.., &self.unknown_tagged_fields)?;
        }
        Ok(())
    }
    fn compute_size(&self, version: i16) -> Result<usize, EncodeError> {
        let mut total_size = 0;
        total_size += types::Int32.compute_size(&self.throttle_time_ms)?;
        total_size += types::Int16.compute_size(&self.error_code)?;
        if version >= 1 {
            total_size += types::CompactString.compute_size(&self.error_message)?;
        } else {
            total_size += types::String.compute_size(&self.error_message)?;
        }
        if version >= 1 {
            total_size += types::CompactArray(types::Struct { version }).compute_size(&self.entries)?;
        } else {
            total_size += types::Array(types::Struct { version }).compute_size(&self.entries)?;
        }
        if version >= 1 {
            let num_tagged_fields = self.unknown_tagged_fields.len();
            if num_tagged_fields > std::u32::MAX as usize {
                error!("Too many tagged fields to encode ({} fields)", num_tagged_fields);
                return Err(EncodeError);
            }
            total_size += types::UnsignedVarInt.compute_size(num_tagged_fields as u32)?;

            total_size += compute_unknown_tagged_fields_size(&self.unknown_tagged_fields)?;
        }
        Ok(total_size)
    }
}

impl Decodable for DescribeClientQuotasResponse {
    fn decode<B: ByteBuf>(buf: &mut B, version: i16) -> Result<Self, DecodeError> {
        let throttle_time_ms = types::Int32.decode(buf)?;
        let error_code = types::Int16.decode(buf)?;
        let error_message = if version >= 1 {
            types::CompactString.decode(buf)?
        } else {
            types::String.decode(buf)?
        };
        let entries = if version >= 1 {
            types::CompactArray(types::Struct { version }).decode(buf)?
        } else {
            types::Array(types::Struct { version }).decode(buf)?
        };
        let mut unknown_tagged_fields = BTreeMap::new();
        if version >= 1 {
            let num_tagged_fields = types::UnsignedVarInt.decode(buf)?;
            for _ in 0..num_tagged_fields {
                let tag: u32 = types::UnsignedVarInt.decode(buf)?;
                let size: u32 = types::UnsignedVarInt.decode(buf)?;
                let mut unknown_value = vec![0; size as usize];
                buf.try_copy_to_slice(&mut unknown_value)?;
                unknown_tagged_fields.insert(tag as i32, unknown_value);
            }
        }
        Ok(Self {
            throttle_time_ms,
            error_code,
            error_message,
            entries,
            unknown_tagged_fields,
        })
    }
}

impl Default for DescribeClientQuotasResponse {
    fn default() -> Self {
        Self {
            throttle_time_ms: 0,
            error_code: 0,
            error_message: Some(Default::default()),
            entries: Some(Default::default()),
            unknown_tagged_fields: BTreeMap::new(),
        }
    }
}

impl Message for DescribeClientQuotasResponse {
    const VERSIONS: VersionRange = VersionRange { min: 0, max: 1 };
}

impl HeaderVersion for DescribeClientQuotasResponse {
    fn header_version(version: i16) -> i16 {
        if version >= 1 {
            1
        } else {
            0
        }
    }
}


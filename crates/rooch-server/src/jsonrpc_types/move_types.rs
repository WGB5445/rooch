// Copyright (c) RoochNetwork
// SPDX-License-Identifier: Apache-2.0

use crate::jsonrpc_types::StrView;
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::{ModuleId, StructTag, TypeTag},
    u256,
};
use move_resource_viewer::{AnnotatedMoveStruct, AnnotatedMoveValue};
use moveos_types::event::{Event, EventID};
use moveos_types::event_filter::MoveOSEvent;
use moveos_types::h256::H256;
use moveos_types::{
    move_types::FunctionId,
    object::{AnnotatedObject, ObjectID},
    transaction::FunctionCall,
};
use serde::{Deserialize, Serialize};

pub type ModuleIdView = StrView<ModuleId>;
pub type TypeTagView = StrView<TypeTag>;
pub type StructTagView = StrView<StructTag>;

impl_str_view_for! {TypeTag StructTag}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnnotatedMoveStructView {
    pub abilities: u8,
    pub type_: StructTagView,
    pub value: Vec<(Identifier, AnnotatedMoveValueView)>,
}

impl From<AnnotatedMoveStruct> for AnnotatedMoveStructView {
    fn from(origin: AnnotatedMoveStruct) -> Self {
        Self {
            abilities: origin.abilities.into_u8(),
            type_: StrView(origin.type_),
            value: origin
                .value
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AnnotatedMoveValueView {
    U8(u8),
    U64(StrView<u64>),
    U128(StrView<u128>),
    Bool(bool),
    Address(AccountAddress),
    Vector(TypeTagView, Vec<AnnotatedMoveValueView>),
    Bytes(StrView<Vec<u8>>),
    Struct(AnnotatedMoveStructView),
    U16(StrView<u16>),
    U32(StrView<u32>),
    U256(StrView<u256::U256>),
}

impl From<AnnotatedMoveValue> for AnnotatedMoveValueView {
    fn from(origin: AnnotatedMoveValue) -> Self {
        match origin {
            AnnotatedMoveValue::U8(u) => AnnotatedMoveValueView::U8(u),
            AnnotatedMoveValue::U64(u) => AnnotatedMoveValueView::U64(StrView(u)),
            AnnotatedMoveValue::U128(u) => AnnotatedMoveValueView::U128(StrView(u)),
            AnnotatedMoveValue::Bool(b) => AnnotatedMoveValueView::Bool(b),
            AnnotatedMoveValue::Address(data) => AnnotatedMoveValueView::Address(data),
            AnnotatedMoveValue::Vector(type_tag, data) => AnnotatedMoveValueView::Vector(
                type_tag.into(),
                data.into_iter().map(Into::into).collect(),
            ),
            AnnotatedMoveValue::Bytes(data) => AnnotatedMoveValueView::Bytes(StrView(data)),
            AnnotatedMoveValue::Struct(data) => AnnotatedMoveValueView::Struct(data.into()),
            AnnotatedMoveValue::U16(u) => AnnotatedMoveValueView::U16(StrView(u)),
            AnnotatedMoveValue::U32(u) => AnnotatedMoveValueView::U32(StrView(u)),
            AnnotatedMoveValue::U256(u) => AnnotatedMoveValueView::U256(StrView(u)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AnnotatedObjectView {
    pub id: ObjectID,
    pub owner: AccountAddress,
    pub value: AnnotatedMoveStructView,
}

impl From<AnnotatedObject> for AnnotatedObjectView {
    fn from(origin: AnnotatedObject) -> Self {
        Self {
            id: origin.id,
            owner: origin.owner,
            value: origin.value.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCallView {
    pub function_id: FunctionId,
    pub ty_args: Vec<TypeTagView>,
    pub args: Vec<StrView<Vec<u8>>>,
}

impl From<FunctionCall> for FunctionCallView {
    fn from(origin: FunctionCall) -> Self {
        Self {
            function_id: origin.function_id,
            ty_args: origin.ty_args.into_iter().map(StrView).collect(),
            args: origin.args.into_iter().map(StrView).collect(),
        }
    }
}

impl From<FunctionCallView> for FunctionCall {
    fn from(value: FunctionCallView) -> Self {
        Self {
            function_id: value.function_id,
            ty_args: value.ty_args.into_iter().map(Into::into).collect(),
            args: value.args.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<H256>,
    /// Sender's address.
    pub sender: AccountAddress,
    pub event_data: StrView<Vec<u8>>,
    pub type_tag: TypeTagView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_index: Option<u32>,
    pub event_id: EventID,
    // pub event_seq_number: StrView<u64>,
}

impl From<Event> for EventView {
    fn from(event: Event) -> Self {
        EventView {
            tx_hash: None,
            sender: AccountAddress::ZERO, //Reserved as an extension field
            event_data: StrView(event.event_data().to_vec()),
            type_tag: event.type_tag().clone().into(),
            event_index: Some(event.event_index),
            event_id: *event.event_id(),
            // event_seq_number: event.sequence_number().into(),
        }
    }
}

impl From<MoveOSEvent> for EventView {
    fn from(event: MoveOSEvent) -> Self {
        EventView {
            tx_hash: event.tx_hash,
            sender: AccountAddress::ZERO, //Reserved as an extension field
            event_data: StrView(event.event_data.to_vec()),
            type_tag: event.type_tag.clone().into(),
            event_index: Some(event.event_index),
            event_id: event.event_id,
            // event_seq_number: event.sequence_number.into(),
        }
    }
}

impl EventView {
    pub fn try_from(event: Event, tx_hash: H256) -> Self {
        let Event {
            event_id,
            // sequence_number,
            type_tag,
            event_data,
            event_index,
        } = event;

        EventView {
            tx_hash: Some(tx_hash),
            sender: AccountAddress::ZERO, //Reserved as an extension field
            event_data: StrView(event_data.to_vec()),
            type_tag: type_tag.into(),
            event_index: Some(event_index),
            event_id,
            // event_seq_number: sequence_number.into(),
        }
    }
}

// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CabLocationRequest {
    // message fields
    pub name: ::std::string::String,
    pub location: ::protobuf::SingularPtrField<Location>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CabLocationRequest {}

impl CabLocationRequest {
    pub fn new() -> CabLocationRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CabLocationRequest {
        static mut instance: ::protobuf::lazy::Lazy<CabLocationRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CabLocationRequest,
        };
        unsafe {
            instance.get(CabLocationRequest::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .foobar.Location location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Location) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut Location {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> Location {
        self.location.take().unwrap_or_else(|| Location::new())
    }

    pub fn get_location(&self) -> &Location {
        self.location.as_ref().unwrap_or_else(|| Location::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<Location> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Location> {
        &mut self.location
    }
}

impl ::protobuf::Message for CabLocationRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CabLocationRequest {
    fn new() -> CabLocationRequest {
        CabLocationRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CabLocationRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CabLocationRequest::get_name_for_reflect,
                    CabLocationRequest::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Location>>(
                    "location",
                    CabLocationRequest::get_location_for_reflect,
                    CabLocationRequest::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CabLocationRequest>(
                    "CabLocationRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CabLocationRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CabLocationRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CabLocationRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CabLocationResponse {
    // message fields
    pub accepted: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CabLocationResponse {}

impl CabLocationResponse {
    pub fn new() -> CabLocationResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CabLocationResponse {
        static mut instance: ::protobuf::lazy::Lazy<CabLocationResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CabLocationResponse,
        };
        unsafe {
            instance.get(CabLocationResponse::new)
        }
    }

    // bool accepted = 1;

    pub fn clear_accepted(&mut self) {
        self.accepted = false;
    }

    // Param is passed by value, moved
    pub fn set_accepted(&mut self, v: bool) {
        self.accepted = v;
    }

    pub fn get_accepted(&self) -> bool {
        self.accepted
    }

    fn get_accepted_for_reflect(&self) -> &bool {
        &self.accepted
    }

    fn mut_accepted_for_reflect(&mut self) -> &mut bool {
        &mut self.accepted
    }
}

impl ::protobuf::Message for CabLocationResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.accepted = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.accepted != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.accepted != false {
            os.write_bool(1, self.accepted)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CabLocationResponse {
    fn new() -> CabLocationResponse {
        CabLocationResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CabLocationResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "accepted",
                    CabLocationResponse::get_accepted_for_reflect,
                    CabLocationResponse::mut_accepted_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CabLocationResponse>(
                    "CabLocationResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CabLocationResponse {
    fn clear(&mut self) {
        self.clear_accepted();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CabLocationResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CabLocationResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetCabRequest {
    // message fields
    pub location: ::protobuf::SingularPtrField<Location>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetCabRequest {}

impl GetCabRequest {
    pub fn new() -> GetCabRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetCabRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetCabRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetCabRequest,
        };
        unsafe {
            instance.get(GetCabRequest::new)
        }
    }

    // .foobar.Location location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Location) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut Location {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> Location {
        self.location.take().unwrap_or_else(|| Location::new())
    }

    pub fn get_location(&self) -> &Location {
        self.location.as_ref().unwrap_or_else(|| Location::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<Location> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Location> {
        &mut self.location
    }
}

impl ::protobuf::Message for GetCabRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetCabRequest {
    fn new() -> GetCabRequest {
        GetCabRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetCabRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Location>>(
                    "location",
                    GetCabRequest::get_location_for_reflect,
                    GetCabRequest::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetCabRequest>(
                    "GetCabRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetCabRequest {
    fn clear(&mut self) {
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetCabRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetCabRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetCabResponse {
    // message fields
    pub cabs: ::protobuf::RepeatedField<Cab>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetCabResponse {}

impl GetCabResponse {
    pub fn new() -> GetCabResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetCabResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetCabResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetCabResponse,
        };
        unsafe {
            instance.get(GetCabResponse::new)
        }
    }

    // repeated .foobar.Cab cabs = 1;

    pub fn clear_cabs(&mut self) {
        self.cabs.clear();
    }

    // Param is passed by value, moved
    pub fn set_cabs(&mut self, v: ::protobuf::RepeatedField<Cab>) {
        self.cabs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cabs(&mut self) -> &mut ::protobuf::RepeatedField<Cab> {
        &mut self.cabs
    }

    // Take field
    pub fn take_cabs(&mut self) -> ::protobuf::RepeatedField<Cab> {
        ::std::mem::replace(&mut self.cabs, ::protobuf::RepeatedField::new())
    }

    pub fn get_cabs(&self) -> &[Cab] {
        &self.cabs
    }

    fn get_cabs_for_reflect(&self) -> &::protobuf::RepeatedField<Cab> {
        &self.cabs
    }

    fn mut_cabs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Cab> {
        &mut self.cabs
    }
}

impl ::protobuf::Message for GetCabResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.cabs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cabs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cabs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cabs {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetCabResponse {
    fn new() -> GetCabResponse {
        GetCabResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetCabResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Cab>>(
                    "cabs",
                    GetCabResponse::get_cabs_for_reflect,
                    GetCabResponse::mut_cabs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetCabResponse>(
                    "GetCabResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetCabResponse {
    fn clear(&mut self) {
        self.clear_cabs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetCabResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetCabResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cab {
    // message fields
    pub name: ::std::string::String,
    pub location: ::protobuf::SingularPtrField<Location>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cab {}

impl Cab {
    pub fn new() -> Cab {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cab {
        static mut instance: ::protobuf::lazy::Lazy<Cab> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cab,
        };
        unsafe {
            instance.get(Cab::new)
        }
    }

    // string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::std::string::String {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // .foobar.Location location = 2;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: Location) {
        self.location = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location(&mut self) -> &mut Location {
        if self.location.is_none() {
            self.location.set_default();
        }
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> Location {
        self.location.take().unwrap_or_else(|| Location::new())
    }

    pub fn get_location(&self) -> &Location {
        self.location.as_ref().unwrap_or_else(|| Location::default_instance())
    }

    fn get_location_for_reflect(&self) -> &::protobuf::SingularPtrField<Location> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Location> {
        &mut self.location
    }
}

impl ::protobuf::Message for Cab {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.location)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.location.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.location.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Cab {
    fn new() -> Cab {
        Cab::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cab>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Cab::get_name_for_reflect,
                    Cab::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Location>>(
                    "location",
                    Cab::get_location_for_reflect,
                    Cab::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cab>(
                    "Cab",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cab {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cab {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cab {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Location {
    // message fields
    pub latitude: f32,
    pub longitude: f32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Location {}

impl Location {
    pub fn new() -> Location {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Location {
        static mut instance: ::protobuf::lazy::Lazy<Location> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Location,
        };
        unsafe {
            instance.get(Location::new)
        }
    }

    // float latitude = 1;

    pub fn clear_latitude(&mut self) {
        self.latitude = 0.;
    }

    // Param is passed by value, moved
    pub fn set_latitude(&mut self, v: f32) {
        self.latitude = v;
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude
    }

    fn get_latitude_for_reflect(&self) -> &f32 {
        &self.latitude
    }

    fn mut_latitude_for_reflect(&mut self) -> &mut f32 {
        &mut self.latitude
    }

    // float longitude = 2;

    pub fn clear_longitude(&mut self) {
        self.longitude = 0.;
    }

    // Param is passed by value, moved
    pub fn set_longitude(&mut self, v: f32) {
        self.longitude = v;
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude
    }

    fn get_longitude_for_reflect(&self) -> &f32 {
        &self.longitude
    }

    fn mut_longitude_for_reflect(&mut self) -> &mut f32 {
        &mut self.longitude
    }
}

impl ::protobuf::Message for Location {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.latitude = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.longitude = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.latitude != 0. {
            my_size += 5;
        }
        if self.longitude != 0. {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.latitude != 0. {
            os.write_float(1, self.latitude)?;
        }
        if self.longitude != 0. {
            os.write_float(2, self.longitude)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Location {
    fn new() -> Location {
        Location::new()
    }

    fn descriptor_static(_: ::std::option::Option<Location>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "latitude",
                    Location::get_latitude_for_reflect,
                    Location::mut_latitude_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "longitude",
                    Location::get_longitude_for_reflect,
                    Location::mut_longitude_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Location>(
                    "Location",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Location {
    fn clear(&mut self) {
        self.clear_latitude();
        self.clear_longitude();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Location {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cfoobar.proto\x12\x06foobar\"V\n\x12CabLocationRequest\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12,\n\x08location\x18\x02\x20\x01(\
    \x0b2\x10.foobar.LocationR\x08location\"1\n\x13CabLocationResponse\x12\
    \x1a\n\x08accepted\x18\x01\x20\x01(\x08R\x08accepted\"=\n\rGetCabRequest\
    \x12,\n\x08location\x18\x01\x20\x01(\x0b2\x10.foobar.LocationR\x08locati\
    on\"1\n\x0eGetCabResponse\x12\x1f\n\x04cabs\x18\x01\x20\x03(\x0b2\x0b.fo\
    obar.CabR\x04cabs\"G\n\x03Cab\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x12,\n\x08location\x18\x02\x20\x01(\x0b2\x10.foobar.LocationR\x08loc\
    ation\"D\n\x08Location\x12\x1a\n\x08latitude\x18\x01\x20\x01(\x02R\x08la\
    titude\x12\x1c\n\tlongitude\x18\x02\x20\x01(\x02R\tlongitude2\x9a\x01\n\
    \rFooBarService\x12N\n\x13record_cab_location\x12\x1a.foobar.CabLocation\
    Request\x1a\x1b.foobar.CabLocationResponse\x129\n\x08get_cabs\x12\x15.fo\
    obar.GetCabRequest\x1a\x16.foobar.GetCabResponseb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}

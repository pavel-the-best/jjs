// Autogenerated by Thrift Compiler (1.0.0-dev)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments, type_complexity))]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate ordered_float;
extern crate thrift;
extern crate try_from;

use ordered_float::OrderedFloat;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::From;
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use try_from::TryFrom;

use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

//
// AuthToken
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AuthToken {
  pub buf: Option<Vec<u8>>,
}

impl AuthToken {
  pub fn new<F1>(buf: F1) -> AuthToken where F1: Into<Option<Vec<u8>>> {
    AuthToken {
      buf: buf.into(),
    }
  }
  pub fn read_from_in_protocol(i_prot: &mut TInputProtocol) -> thrift::Result<AuthToken> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<u8>> = Some(Vec::new());
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = i_prot.read_bytes()?;
          f_1 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    let ret = AuthToken {
      buf: f_1,
    };
    Ok(ret)
  }
  pub fn write_to_out_protocol(&self, o_prot: &mut TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("AuthToken");
    o_prot.write_struct_begin(&struct_ident)?;
    if let Some(ref fld_var) = self.buf {
      o_prot.write_field_begin(&TFieldIdentifier::new("buf", TType::String, 1))?;
      o_prot.write_bytes(fld_var)?;
      o_prot.write_field_end()?;
      ()
    } else {
      ()
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

impl Default for AuthToken {
  fn default() -> Self {
    AuthToken{
      buf: Some(Vec::new()),
    }
  }
}

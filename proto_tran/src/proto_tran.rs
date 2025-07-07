// @generated, do not edit
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update {
  /// 源文本的语言
  pub lang: u32,
  pub hash: ::std::vec::Vec<u8>,
  /// 语言 - 有更新的译文
  pub map: ::std::vec::Vec<Update_MapEntry>,
}
impl ::std::default::Default for Update {
  fn default() -> Self {
    Update {
      lang: ::std::default::Default::default(),
      hash: ::std::default::Default::default(),
      map: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Update_default: Update = Update::default();
}
impl ::pb_jelly::Message for Update {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Update",
      full_name: "Update",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "Update.lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "hash",
          full_name: "Update.hash",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "map",
          full_name: "Update.map",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.lang, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.hash, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    for val in &self.map {
      size += ::pb_jelly::helpers::compute_size_field::<Update_MapEntry>(val, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.lang, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.hash, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    for val in &self.map {
      ::pb_jelly::helpers::serialize_field::<W, Update_MapEntry>(w, val, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Update", 1)?;
          self.lang = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "Update", 2)?;
          self.hash = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, Update_MapEntry>(buf, typ, "Update", 3)?;
          self.map.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Update {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "lang" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.lang)
      }
      "hash" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.hash)
      }
      "map" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Update_MapEntry {
  pub key: u32,
  pub value: ::std::string::String,
}
impl ::std::default::Default for Update_MapEntry {
  fn default() -> Self {
    Update_MapEntry {
      key: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Update_MapEntry_default: Update_MapEntry = Update_MapEntry::default();
}
impl ::pb_jelly::Message for Update_MapEntry {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Update_MapEntry",
      full_name: "Update_MapEntry",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "key",
          full_name: "Update_MapEntry.key",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "value",
          full_name: "Update_MapEntry.value",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.key, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.value, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.key, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.value, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Update_MapEntry", 1)?;
          self.key = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "Update_MapEntry", 2)?;
          self.value = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Update_MapEntry {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "key" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.key)
      }
      "value" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.value)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tran {
  pub from_lang: u32,
  pub to_lang_li: ::std::vec::Vec<u32>,
  pub txt: ::std::string::String,
  pub path: ::std::string::String,
}
impl ::std::default::Default for Tran {
  fn default() -> Self {
    Tran {
      from_lang: ::std::default::Default::default(),
      to_lang_li: ::std::default::Default::default(),
      txt: ::std::default::Default::default(),
      path: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Tran_default: Tran = Tran::default();
}
impl ::pb_jelly::Message for Tran {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Tran",
      full_name: "Tran",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "from_lang",
          full_name: "Tran.from_lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "to_lang_li",
          full_name: "Tran.to_lang_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "txt",
          full_name: "Tran.txt",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "path",
          full_name: "Tran.path",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.from_lang, 1, ::pb_jelly::wire_format::Type::Varint);
    if !self.to_lang_li.is_empty() {
      let mut to_lang_li_size = 0usize;
      for val in &self.to_lang_li {
        to_lang_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(2);
      size += ::pb_jelly::varint::serialized_length(to_lang_li_size as u64);
      size += to_lang_li_size;
    }
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.txt, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.path, 4, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.from_lang, 1, ::pb_jelly::wire_format::Type::Varint)?;
    if !self.to_lang_li.is_empty() {
      let mut size = 0usize;
      for val in &self.to_lang_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.to_lang_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.txt, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.path, 4, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Tran", 1)?;
          self.from_lang = val;
        }
        2 => {
          ::pb_jelly::helpers::deserialize_packed::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Tran", 2, &mut self.to_lang_li)?;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "Tran", 3)?;
          self.txt = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "Tran", 4)?;
          self.path = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Tran {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "from_lang" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.from_lang)
      }
      "to_lang_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "txt" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.txt)
      }
      "path" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.path)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Job {
  pub update_li: ::std::vec::Vec<Update>,
  pub tran_li: ::std::vec::Vec<Tran>,
}
impl ::std::default::Default for Job {
  fn default() -> Self {
    Job {
      update_li: ::std::default::Default::default(),
      tran_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Job_default: Job = Job::default();
}
impl ::pb_jelly::Message for Job {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Job",
      full_name: "Job",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "update_li",
          full_name: "Job.update_li",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "tran_li",
          full_name: "Job.tran_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    for val in &self.update_li {
      size += ::pb_jelly::helpers::compute_size_field::<Update>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    for val in &self.tran_li {
      size += ::pb_jelly::helpers::compute_size_field::<Tran>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.update_li {
      ::pb_jelly::helpers::serialize_field::<W, Update>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    for val in &self.tran_li {
      ::pb_jelly::helpers::serialize_field::<W, Tran>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, Update>(buf, typ, "Job", 1)?;
          self.update_li.push(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, Tran>(buf, typ, "Job", 2)?;
          self.tran_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Job {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "update_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "tran_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}


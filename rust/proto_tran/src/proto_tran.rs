// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Filetype {
  Md = 0,
  Yml = 1,
}
impl Filetype {
  pub const KNOWN_VARIANTS: [Filetype; 2] = [Filetype::Md, Filetype::Yml];
}
impl ::std::default::Default for Filetype {
  fn default() -> Self {
    Filetype::Md
  }
}
impl From<Filetype> for i32 {
  fn from(v: Filetype) -> i32 {
    match v {
      Filetype::Md => 0,
      Filetype::Yml => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Filetype {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Filetype::Md),
      1 => Ok(Filetype::Yml),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for Filetype {
}
impl ::pb_jelly::ClosedProtoEnum for Filetype {
  fn name(self) -> &'static str {
    match self {
      Filetype::Md => "Md",
      Filetype::Yml => "Yml",
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LangTxt {
  pub lang: u32,
  pub txt: ::std::string::String,
}
impl ::std::default::Default for LangTxt {
  fn default() -> Self {
    LangTxt {
      lang: ::std::default::Default::default(),
      txt: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref LangTxt_default: LangTxt = LangTxt::default();
}
impl ::pb_jelly::Message for LangTxt {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "LangTxt",
      full_name: "LangTxt",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "lang",
          full_name: "LangTxt.lang",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "txt",
          full_name: "LangTxt.txt",
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
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.lang, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.txt, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.lang, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.txt, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "LangTxt", 1)?;
          self.lang = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "LangTxt", 2)?;
          self.txt = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for LangTxt {
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
      "txt" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.txt)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UpdateLi {
  pub filetype: Filetype,
  /// 源文本的语言
  pub src_lang: u32,
  pub hash: ::std::vec::Vec<u8>,
  pub li: ::std::vec::Vec<LangTxt>,
}
impl ::std::default::Default for UpdateLi {
  fn default() -> Self {
    UpdateLi {
      filetype: ::std::default::Default::default(),
      src_lang: ::std::default::Default::default(),
      hash: ::std::default::Default::default(),
      li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref UpdateLi_default: UpdateLi = UpdateLi::default();
}
impl ::pb_jelly::Message for UpdateLi {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UpdateLi",
      full_name: "UpdateLi",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "filetype",
          full_name: "UpdateLi.filetype",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "src_lang",
          full_name: "UpdateLi.src_lang",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "hash",
          full_name: "UpdateLi.hash",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "li",
          full_name: "UpdateLi.li",
          index: 3,
          number: 4,
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
    size += ::pb_jelly::helpers::compute_size_scalar::<Filetype>(&self.filetype, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.src_lang, 2, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.hash, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    for val in &self.li {
      size += ::pb_jelly::helpers::compute_size_field::<LangTxt>(val, 4, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, Filetype>(w, &self.filetype, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.src_lang, 2, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.hash, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    for val in &self.li {
      ::pb_jelly::helpers::serialize_field::<W, LangTxt>(w, val, 4, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Filetype>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "UpdateLi", 1)?;
          self.filetype = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "UpdateLi", 2)?;
          self.src_lang = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "UpdateLi", 3)?;
          self.hash = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, LangTxt>(buf, typ, "UpdateLi", 4)?;
          self.li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for UpdateLi {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "filetype" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.filetype)
      }
      "src_lang" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.src_lang)
      }
      "hash" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.hash)
      }
      "li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// 更新缓存的接口
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UpdateCache {
  pub update_li: ::std::vec::Vec<UpdateLi>,
}
impl ::std::default::Default for UpdateCache {
  fn default() -> Self {
    UpdateCache {
      update_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref UpdateCache_default: UpdateCache = UpdateCache::default();
}
impl ::pb_jelly::Message for UpdateCache {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UpdateCache",
      full_name: "UpdateCache",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "update_li",
          full_name: "UpdateCache.update_li",
          index: 0,
          number: 1,
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
      size += ::pb_jelly::helpers::compute_size_field::<UpdateLi>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.update_li {
      ::pb_jelly::helpers::serialize_field::<W, UpdateLi>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UpdateLi>(buf, typ, "UpdateCache", 1)?;
          self.update_li.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for UpdateCache {
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
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// 翻译文件的接口
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tran {
  pub from_lang: u32,
  pub to_lang: u32,
  /// 术语替换放到客户端完成,
  /// repeated Term term_li = 4;
  pub txt: ::std::string::String,
}
impl ::std::default::Default for Tran {
  fn default() -> Self {
    Tran {
      from_lang: ::std::default::Default::default(),
      to_lang: ::std::default::Default::default(),
      txt: ::std::default::Default::default(),
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
          name: "to_lang",
          full_name: "Tran.to_lang",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
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
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.from_lang, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.to_lang, 2, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.txt, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.from_lang, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.to_lang, 2, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.txt, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
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
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Tran", 2)?;
          self.to_lang = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "Tran", 3)?;
          self.txt = val;
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
      "to_lang" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.to_lang)
      }
      "txt" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.txt)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}


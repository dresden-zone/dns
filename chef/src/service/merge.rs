use entity::prelude::{RecordA, RecordAaaa, RecordCname, RecordMx, RecordNs, RecordTxt};
use entity::record;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) enum RecordType {
  #[serde(rename = "a")]
  A,
  #[serde(rename = "aaaa")]
  Aaaa,
  #[serde(rename = "cname")]
  Cname,
  #[serde(rename = "mx")]
  Mx,
  #[serde(rename = "ns")]
  Ns,
  #[serde(rename = "txt")]
  Txt,
}

#[derive(Serialize)]
pub(crate) struct ApiRecord<M> {
  pub record_type: RecordType,
  pub record: record::Model,
  pub value: M,
}

pub(crate) trait MergeObject<E: EntityTrait<Model = M>, M> {
  fn merge(value: (record::Model, Option<M>)) -> ApiRecord<M>;
}

impl MergeObject<RecordA, entity::record_a::Model> for ApiRecord<entity::record_a::Model> {
  fn merge(
    value: (record::Model, Option<entity::record_a::Model>),
  ) -> ApiRecord<entity::record_a::Model> {
    ApiRecord::<entity::record_a::Model> {
      record_type: RecordType::A,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

impl MergeObject<RecordAaaa, entity::record_aaaa::Model> for ApiRecord<entity::record_aaaa::Model> {
  fn merge(
    value: (record::Model, Option<entity::record_aaaa::Model>),
  ) -> ApiRecord<entity::record_aaaa::Model> {
    ApiRecord::<entity::record_aaaa::Model> {
      record_type: RecordType::Aaaa,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

impl MergeObject<RecordCname, entity::record_cname::Model>
  for ApiRecord<entity::record_cname::Model>
{
  fn merge(
    value: (record::Model, Option<entity::record_cname::Model>),
  ) -> ApiRecord<entity::record_cname::Model> {
    ApiRecord::<entity::record_cname::Model> {
      record_type: RecordType::Cname,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

impl MergeObject<RecordMx, entity::record_mx::Model> for ApiRecord<entity::record_mx::Model> {
  fn merge(
    value: (record::Model, Option<entity::record_mx::Model>),
  ) -> ApiRecord<entity::record_mx::Model> {
    ApiRecord::<entity::record_mx::Model> {
      record_type: RecordType::Mx,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

impl MergeObject<RecordNs, entity::record_ns::Model> for ApiRecord<entity::record_ns::Model> {
  fn merge(
    value: (record::Model, Option<entity::record_ns::Model>),
  ) -> ApiRecord<entity::record_ns::Model> {
    ApiRecord::<entity::record_ns::Model> {
      record_type: RecordType::Ns,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

impl MergeObject<RecordTxt, entity::record_txt::Model> for ApiRecord<entity::record_txt::Model> {
  fn merge(
    value: (record::Model, Option<entity::record_txt::Model>),
  ) -> ApiRecord<entity::record_txt::Model> {
    ApiRecord::<entity::record_txt::Model> {
      record_type: RecordType::Ns,
      record: value.0,
      value: value.1.unwrap(),
    }
  }
}

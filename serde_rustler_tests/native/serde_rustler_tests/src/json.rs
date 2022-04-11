use rustler::{Env, Error as NifError, NifResult, Term};
use serde_bytes::Bytes;
use serde_rustler::{from_term, to_term, Deserializer, Serializer};
use serde_transcode::transcode;

#[inline]
#[rustler::nif(name = "decode_json")]
/// Deserializes a JSON string into an Elixir term.
pub fn decode<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_decode(env, term)
}

#[inline]
#[rustler::nif(name = "decode_json_dirty", schedule = "DirtyCpu")]
/// Deserializes a JSON string into an Elixir term.
pub fn decode_dirty<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_decode(env, term)
}

#[inline]
fn do_decode<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    let json_bytes: &[u8] = from_term(term)?;
    let mut de = serde_json::Deserializer::from_slice(json_bytes);
    let ser = Serializer::from(env);
    transcode(&mut de, ser).map_err(|err| err.into())
}

#[inline]
#[rustler::nif(name = "encode_json_compact")]
/// Serializes an Elixir term into a compact JSON string.
pub fn encode_compact<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_encode_compact(env, term)
}

#[inline]
#[rustler::nif(name = "encode_json_compact_dirty", schedule = "DirtyCpu")]
/// Serializes an Elixir term into a compact JSON string.
pub fn encode_compact_dirty<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_encode_compact(env, term)
}

#[inline]
fn do_encode_compact<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    let de = Deserializer::from(term);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::new(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}

#[inline]
#[rustler::nif(name = "encode_json_pretty")]
/// Serializes an Elixir term into a pretty-printed JSON string.
pub fn encode_pretty<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_encode_pretty(env, term)
}

#[inline]
#[rustler::nif(name = "encode_json_pretty_dirty", schedule = "DirtyCpu")]
/// Serializes an Elixir term into a pretty-printed JSON string.
pub fn encode_pretty_dirty<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    do_encode_pretty(env, term)
}

#[inline]
fn do_encode_pretty<'a>(env: Env<'a>, term: Term<'a>) -> NifResult<Term<'a>> {
    let de = Deserializer::from(term);
    let mut ser_vec = Vec::new();
    let mut ser = serde_json::Serializer::pretty(&mut ser_vec);
    transcode(de, &mut ser).or(Err(NifError::RaiseAtom("transcode error")))?;
    to_term(env, Bytes::new(&ser_vec)).map_err(|err| err.into())
}

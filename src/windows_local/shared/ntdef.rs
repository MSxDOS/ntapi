use crate::ctypes::{c_long, c_longlong, c_ulong, c_ulonglong};

UNION! {union LARGE_INTEGER {
  [i64; 1],
  s s_mut: LARGE_INTEGER_s,
  u u_mut: LARGE_INTEGER_u,
  QuadPart QuadPart_mut: c_longlong,
}}
STRUCT! {struct LARGE_INTEGER_s {
  LowPart: c_ulong,
  HighPart: c_long,
}}
STRUCT! {struct LARGE_INTEGER_u {
  LowPart: c_ulong,
  HighPart: c_long,
}}
UNION! {union ULARGE_INTEGER {
  [u64; 1],
  s s_mut: ULARGE_INTEGER_s,
  u u_mut: ULARGE_INTEGER_s,
  QuadPart QuadPart_mut: c_ulonglong,
}}
STRUCT! {struct ULARGE_INTEGER_s {
  LowPart: c_ulong,
  HighPart: c_ulong,
}}
STRUCT! {struct ULARGE_INTEGER_u {
  LowPart: c_ulong,
  HighPart: c_ulong,
}}

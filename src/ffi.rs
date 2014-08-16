// Bitcoin secp256k1 bindings
// Written in 2014 by
//   Dawid Ciężarkiewicz
//   Andrew Poelstra
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! FFI bindings
use libc::{c_int, c_uchar};

#[link(name = "secp256k1")]
extern "C" {
    pub fn secp256k1_start();

    pub fn secp256k1_stop();

    pub fn secp256k1_ecdsa_verify(msg: *const c_uchar, msg_len: c_int,
                                  sig: *const c_uchar, sig_len: c_int,
                                  pk: *const c_uchar, pk_len: c_int)
                                  -> c_int;

    pub fn secp256k1_ecdsa_pubkey_create(pk: *mut c_uchar, pk_len : *mut c_int,
                                         sk: *const c_uchar, compressed: c_int)
                                         -> c_int;

    pub fn secp256k1_ecdsa_sign(msg: *const c_uchar, msg_len: c_int,
                                sig: *mut c_uchar, sig_len: *mut c_int,
                                sk: *const c_uchar, nonce: *const c_uchar)
                                -> c_int;

    pub fn secp256k1_ecdsa_sign_compact(msg: *const c_uchar, msg_len: c_int,
                                        sig64: *mut c_uchar, sk: *const c_uchar,
                                        nonce: *const c_uchar, recid: *mut c_int)
                                        -> c_int;

    pub fn secp256k1_ecdsa_recover_compact(msg: *const c_uchar, msg_len: c_int,
                                           sig64: *const c_uchar, pk: *mut c_uchar,
                                           pk_len: *mut c_int, compressed: c_int,
                                           recid: c_int) -> c_int;
}

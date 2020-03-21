// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


use super::*;
use self::syscall::*;
use ::libc::c_uint;
use ::libc::c_ulong;
use ::libc::ENAMETOOLONG;
use ::libc::ENXIO;
use ::libc::EOVERFLOW;
use ::libc::EROFS;
use ::libc::ESPIPE;
use ::libc::ETXTBSY;
use ::libc::F_GETFL;
use ::libc::F_SETFL;
use ::libc::fcntl;
use ::libc::iovec;
use ::std::ffi::CString;
use ::std::mem::transmute_copy;


pub(crate) mod syscall;


include!("PipeFileDescriptor.rs");
include!("ReceivePipeFileDescriptor.rs");
include!("SendPipeFileDescriptor.rs");
include!("SpliceRecipient.rs");
include!("SpliceSender.rs");

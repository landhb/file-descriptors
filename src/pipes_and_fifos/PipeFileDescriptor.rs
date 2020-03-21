// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Extensions for a pipe file descriptor to make it useful for clone'd processes.
pub trait PipeFileDescriptor: AsRawFdExt + Sized
{
	/// Clones a pipe file descriptor so the pipe is accessible in a child process.
	fn clone_for_child_process(&self) -> Self;
}

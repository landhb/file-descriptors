// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a POSIX message queue instance for receiving messages.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceivePosixMessageQueueFileDescriptor(PosixMessageQueueFileDescriptor);

impl AsRawFd for ReceivePosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0.as_raw_fd()
	}
}

impl AsRawFdExt for ReceivePosixMessageQueueFileDescriptor
{
}

impl IntoRawFd for ReceivePosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0.into_raw_fd()
	}
}

impl FromRawFd for ReceivePosixMessageQueueFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(PosixMessageQueueFileDescriptor::from_raw_fd(fd))
	}
}

impl PosixMessageQueue for ReceivePosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn new(name: &CStr, open_or_create: &OpenOrCreatePosixMessageQueue) -> Result<(Self, PosixMessageQueueConstraints), CreationError>
	{
		PosixMessageQueueFileDescriptor::new(name, PosixMessageQueueCreateSendOrReceive::Receive, open_or_create).map(|(message_queue_file_descriptor, posix_message_queue_constraints)| (Self(message_queue_file_descriptor), posix_message_queue_constraints))
	}

	#[inline(always)]
	fn queue_depth(&self) -> usize
	{
		self.0.queue_depth()
	}
}

impl Receive for ReceivePosixMessageQueueFileDescriptor
{
	#[inline(always)]
	fn receive(&self, message_buffer: &mut [u8]) -> Result<(usize, PosixMessagePriority), StructReadError>
	{
		self.0.receive(message_buffer)
	}
}

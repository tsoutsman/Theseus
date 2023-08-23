(function() {var implementors = {
"dereffer":[["impl&lt;Inner, Ref: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"dereffer/struct.DerefsToMut.html\" title=\"struct dereffer::DerefsToMut\">DerefsToMut</a>&lt;Inner, Ref&gt;"],["impl&lt;Inner, Ref: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"dereffer/struct.DerefsTo.html\" title=\"struct dereffer::DerefsTo\">DerefsTo</a>&lt;Inner, Ref&gt;"]],
"dfqueue":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"dfqueue/struct.PeekedData.html\" title=\"struct dfqueue::PeekedData\">PeekedData</a>&lt;T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"dfqueue/struct.QueuedData.html\" title=\"struct dfqueue::QueuedData\">QueuedData</a>&lt;T&gt;"]],
"frame_allocator":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"frame_allocator/struct.PhysicalMemoryRegion.html\" title=\"struct frame_allocator::PhysicalMemoryRegion\">PhysicalMemoryRegion</a>"],["impl&lt;const S: MemoryState&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"frame_allocator/struct.Frames.html\" title=\"struct frame_allocator::Frames\">Frames</a>&lt;S&gt;"],["impl&lt;'f&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"frame_allocator/struct.AllocatedFrame.html\" title=\"struct frame_allocator::AllocatedFrame\">AllocatedFrame</a>&lt;'f&gt;"]],
"io":[["impl&lt;'io, IO, L, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"io/struct.LockableIo.html\" title=\"struct io::LockableIo\">LockableIo</a>&lt;'io, IO, L, B&gt;<span class=\"where fmt-newline\">where\n    IO: 'io + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    L: for&lt;'a&gt; <a class=\"trait\" href=\"lockable/trait.Lockable.html\" title=\"trait lockable::Lockable\">Lockable</a>&lt;'a, IO&gt; + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,\n    B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;L&gt;,</span>"],["impl&lt;IO&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"io/struct.ReaderWriter.html\" title=\"struct io::ReaderWriter\">ReaderWriter</a>&lt;IO&gt;"]],
"irq_safety":[["impl&lt;'a, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"irq_safety/struct.MutexIrqSafeGuard.html\" title=\"struct irq_safety::MutexIrqSafeGuard\">MutexIrqSafeGuard</a>&lt;'a, T&gt;"],["impl&lt;'rwlock, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"irq_safety/struct.RwLockIrqSafeReadGuard.html\" title=\"struct irq_safety::RwLockIrqSafeReadGuard\">RwLockIrqSafeReadGuard</a>&lt;'rwlock, T&gt;"],["impl&lt;'rwlock, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"irq_safety/struct.RwLockIrqSafeWriteGuard.html\" title=\"struct irq_safety::RwLockIrqSafeWriteGuard\">RwLockIrqSafeWriteGuard</a>&lt;'rwlock, T&gt;"]],
"memory":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory/struct.MappedPages.html\" title=\"struct memory::MappedPages\">MappedPages</a>"],["impl&lt;T: FromBytes, M: <a class=\"trait\" href=\"memory/trait.Mutability.html\" title=\"trait memory::Mutability\">Mutability</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"memory/struct.MappedPages.html\" title=\"struct memory::MappedPages\">MappedPages</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory/struct.BorrowedSliceMappedPages.html\" title=\"struct memory::BorrowedSliceMappedPages\">BorrowedSliceMappedPages</a>&lt;T, M, B&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory/struct.PageTable.html\" title=\"struct memory::PageTable\">PageTable</a>"],["impl&lt;T: FromBytes, M: <a class=\"trait\" href=\"memory/trait.Mutability.html\" title=\"trait memory::Mutability\">Mutability</a>, B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html\" title=\"trait core::borrow::Borrow\">Borrow</a>&lt;<a class=\"struct\" href=\"memory/struct.MappedPages.html\" title=\"struct memory::MappedPages\">MappedPages</a>&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory/struct.BorrowedMappedPages.html\" title=\"struct memory::BorrowedMappedPages\">BorrowedMappedPages</a>&lt;T, M, B&gt;"]],
"memory_structs":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory_structs/struct.PageRange.html\" title=\"struct memory_structs::PageRange\">PageRange</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"memory_structs/struct.FrameRange.html\" title=\"struct memory_structs::FrameRange\">FrameRange</a>"]],
"mod_mgmt":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"mod_mgmt/struct.AppCrateRef.html\" title=\"struct mod_mgmt::AppCrateRef\">AppCrateRef</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"mod_mgmt/struct.NamespaceDir.html\" title=\"struct mod_mgmt::NamespaceDir\">NamespaceDir</a>"]],
"net":[["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"net/struct.LockedSocket.html\" title=\"struct net::LockedSocket\">LockedSocket</a>&lt;'a, T&gt;<span class=\"where fmt-newline\">where\n    T: AnySocket&lt;'static&gt;,</span>"]],
"nic_buffers":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"nic_buffers/struct.TransmitBuffer.html\" title=\"struct nic_buffers::TransmitBuffer\">TransmitBuffer</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"nic_buffers/struct.ReceiveBuffer.html\" title=\"struct nic_buffers::ReceiveBuffer\">ReceiveBuffer</a>"]],
"no_drop":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"no_drop/struct.NoDrop.html\" title=\"struct no_drop::NoDrop\">NoDrop</a>&lt;T&gt;"]],
"owned_borrowed_trait":[["impl&lt;'t, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"owned_borrowed_trait/struct.Borrowed.html\" title=\"struct owned_borrowed_trait::Borrowed\">Borrowed</a>&lt;'t, T&gt;"],["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"owned_borrowed_trait/struct.Owned.html\" title=\"struct owned_borrowed_trait::Owned\">Owned</a>&lt;T&gt;"]],
"page_table_entry":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"page_table_entry/struct.UnmappedFrameRange.html\" title=\"struct page_table_entry::UnmappedFrameRange\">UnmappedFrameRange</a>"]],
"path":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"path/struct.Path.html\" title=\"struct path::Path\">Path</a>"]],
"pci":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"pci/struct.PciDevice.html\" title=\"struct pci::PciDevice\">PciDevice</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"pci/struct.MsixVectorTable.html\" title=\"struct pci::MsixVectorTable\">MsixVectorTable</a>"]],
"per_cpu":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"per_cpu/struct.CpuLocalCpuId.html\" title=\"struct per_cpu::CpuLocalCpuId\">CpuLocalCpuId</a>"]],
"root":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"root/struct.ROOT.html\" title=\"struct root::ROOT\">ROOT</a>"]],
"runqueue_epoch":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_epoch/struct.RunQueue.html\" title=\"struct runqueue_epoch::RunQueue\">RunQueue</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_epoch/struct.EpochTaskRef.html\" title=\"struct runqueue_epoch::EpochTaskRef\">EpochTaskRef</a>"]],
"runqueue_priority":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_priority/struct.PriorityTaskRef.html\" title=\"struct runqueue_priority::PriorityTaskRef\">PriorityTaskRef</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_priority/struct.RunQueue.html\" title=\"struct runqueue_priority::RunQueue\">RunQueue</a>"]],
"runqueue_round_robin":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_round_robin/struct.RoundRobinTaskRef.html\" title=\"struct runqueue_round_robin::RoundRobinTaskRef\">RoundRobinTaskRef</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"runqueue_round_robin/struct.RunQueue.html\" title=\"struct runqueue_round_robin::RunQueue\">RunQueue</a>"]],
"serial_port":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"serial_port/struct.SerialPort.html\" title=\"struct serial_port::SerialPort\">SerialPort</a>"]],
"spawn":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"spawn/struct.BootstrapTaskRef.html\" title=\"struct spawn::BootstrapTaskRef\">BootstrapTaskRef</a>"]],
"stack":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"stack/struct.Stack.html\" title=\"struct stack::Stack\">Stack</a>"]],
"stdio":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"stdio/struct.KeyEventReadGuard.html\" title=\"struct stdio::KeyEventReadGuard\">KeyEventReadGuard</a>"]],
"str_ref":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"str_ref/struct.StrRef.html\" title=\"struct str_ref::StrRef\">StrRef</a>"]],
"task":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"task/struct.TaskRef.html\" title=\"struct task::TaskRef\">TaskRef</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"task/struct.ExitableTaskRef.html\" title=\"struct task::ExitableTaskRef\">ExitableTaskRef</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"task/struct.JoinableTaskRef.html\" title=\"struct task::JoinableTaskRef\">JoinableTaskRef</a>"]],
"text_terminal":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"text_terminal/struct.ScrollbackBuffer.html\" title=\"struct text_terminal::ScrollbackBuffer\">ScrollbackBuffer</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"text_terminal/struct.Unit.html\" title=\"struct text_terminal::Unit\">Unit</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"text_terminal/struct.Line.html\" title=\"struct text_terminal::Line\">Line</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
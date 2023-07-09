(function() {var implementors = {
"apic":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"apic/struct.LocalApic.html\" title=\"struct apic::LocalApic\">LocalApic</a>"]],
"async_channel":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, P: DeadlockPrevention&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"async_channel/struct.Receiver.html\" title=\"struct async_channel::Receiver\">Receiver</a>&lt;T, P&gt;"],["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>, P: DeadlockPrevention&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"async_channel/struct.Sender.html\" title=\"struct async_channel::Sender\">Sender</a>&lt;T, P&gt;"]],
"atomic_linked_list":[["impl&lt;K, V&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"atomic_linked_list/atomic_map/struct.AtomicMap.html\" title=\"struct atomic_linked_list::atomic_map::AtomicMap\">AtomicMap</a>&lt;K, V&gt;<span class=\"where fmt-newline\">where\n    K: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a>,</span>"]],
"cpu_local_preemption":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"cpu_local_preemption/struct.PreemptionGuard.html\" title=\"struct cpu_local_preemption::PreemptionGuard\">PreemptionGuard</a>"]],
"crate_metadata":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"crate_metadata/struct.LoadedCrate.html\" title=\"struct crate_metadata::LoadedCrate\">LoadedCrate</a>"]],
"dfqueue":[["impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"dfqueue/mpsc_queue/struct.MpscQueue.html\" title=\"struct dfqueue::mpsc_queue::MpscQueue\">MpscQueue</a>&lt;T&gt;"]],
"frame_allocator":[["impl&lt;'list&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"frame_allocator/struct.DeferredAllocAction.html\" title=\"struct frame_allocator::DeferredAllocAction\">DeferredAllocAction</a>&lt;'list&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"frame_allocator/struct.AllocatedFrames.html\" title=\"struct frame_allocator::AllocatedFrames\">AllocatedFrames</a>"]],
"irq_safety":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"irq_safety/struct.HeldInterrupts.html\" title=\"struct irq_safety::HeldInterrupts\">HeldInterrupts</a>"]],
"memory":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"memory/struct.MappedPages.html\" title=\"struct memory::MappedPages\">MappedPages</a>"]],
"mod_mgmt":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"mod_mgmt/struct.AppCrateRef.html\" title=\"struct mod_mgmt::AppCrateRef\">AppCrateRef</a>"]],
"mutex_sleep":[["impl&lt;'a, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"mutex_sleep/struct.MutexSleepGuard.html\" title=\"struct mutex_sleep::MutexSleepGuard\">MutexSleepGuard</a>&lt;'a, T&gt;"],["impl&lt;'rwlock, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"mutex_sleep/struct.RwLockSleepReadGuard.html\" title=\"struct mutex_sleep::RwLockSleepReadGuard\">RwLockSleepReadGuard</a>&lt;'rwlock, T&gt;"],["impl&lt;'rwlock, T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"mutex_sleep/struct.RwLockSleepWriteGuard.html\" title=\"struct mutex_sleep::RwLockSleepWriteGuard\">RwLockSleepWriteGuard</a>&lt;'rwlock, T&gt;"]],
"nic_buffers":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"nic_buffers/struct.ReceiveBuffer.html\" title=\"struct nic_buffers::ReceiveBuffer\">ReceiveBuffer</a>"]],
"page_allocator":[["impl&lt;'list&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"page_allocator/struct.DeferredAllocAction.html\" title=\"struct page_allocator::DeferredAllocAction\">DeferredAllocAction</a>&lt;'list&gt;"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"page_allocator/struct.AllocatedPages.html\" title=\"struct page_allocator::AllocatedPages\">AllocatedPages</a>"]],
"pmu_x86":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"pmu_x86/struct.Counter.html\" title=\"struct pmu_x86::Counter\">Counter</a>"]],
"runqueue_epoch":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"runqueue_epoch/struct.PriorityInheritanceGuard.html\" title=\"struct runqueue_epoch::PriorityInheritanceGuard\">PriorityInheritanceGuard</a>&lt;'a&gt;"]],
"runqueue_priority":[["impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"runqueue_priority/struct.PriorityInheritanceGuard.html\" title=\"struct runqueue_priority::PriorityInheritanceGuard\">PriorityInheritanceGuard</a>&lt;'a&gt;"]],
"serial_port_basic":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"serial_port_basic/struct.SerialPort.html\" title=\"struct serial_port_basic::SerialPort\">SerialPort</a>"]],
"spawn":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"spawn/struct.BootstrapTaskRef.html\" title=\"struct spawn::BootstrapTaskRef\">BootstrapTaskRef</a>"]],
"stdio":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"stdio/struct.KeyEventReadGuard.html\" title=\"struct stdio::KeyEventReadGuard\">KeyEventReadGuard</a>"]],
"task":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"task/struct.ScheduleOnDrop.html\" title=\"struct task::ScheduleOnDrop\">ScheduleOnDrop</a>"],["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"task/struct.JoinableTaskRef.html\" title=\"struct task::JoinableTaskRef\">JoinableTaskRef</a>"]],
"task_struct":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"task_struct/struct.Task.html\" title=\"struct task_struct::Task\">Task</a>"]],
"text_terminal":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"enum\" href=\"text_terminal/enum.ScrollAction.html\" title=\"enum text_terminal::ScrollAction\">ScrollAction</a>"]],
"virtual_nic":[["impl&lt;S: RxQueueRegisters, T: RxDescriptor, U: TxQueueRegisters, V: TxDescriptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"virtual_nic/struct.VirtualNic.html\" title=\"struct virtual_nic::VirtualNic\">VirtualNic</a>&lt;S, T, U, V&gt;"]],
"wait_guard":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"wait_guard/struct.WaitGuard.html\" title=\"struct wait_guard::WaitGuard\">WaitGuard</a>"]],
"window":[["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"window/struct.Window.html\" title=\"struct window::Window\">Window</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
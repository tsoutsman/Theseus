window.SIDEBAR_ITEMS = {"struct":[["KeyEventQueue",""],["KeyEventQueueReader","A reader to keyevent ring buffer."],["KeyEventQueueWriter","A writer to keyevent ring buffer."],["KeyEventReadGuard","A structure that allows applications to access keyboard events directly.  When it gets instantiated, it `take`s the reader of the `KeyEventQueue` away from the `shell`,  or whichever entity previously owned the queue. When it goes out of the scope, the taken reader will be automatically returned back to the `shell` or the original owner in its `Drop` routine."],["RingBufferEof","A ring buffer with an EOF mark."],["Stdio","A ring buffer containing bytes. It forms `stdin`, `stdout` and `stderr`. The two `Arc`s actually point to the same ring buffer. It is designed to prevent interleaved reading but at the same time allow writing to the ring buffer while the reader is holding its lock, and vice versa."],["StdioReadGuard","`StdioReadGuard` acts like `MutexGuard`, it locks the underlying ring buffer during its lifetime, and provides reading methods to the ring buffer. The lock will be automatically released on dropping of this structure."],["StdioReader","A reader to stdio buffers."],["StdioWriteGuard","`StdioReadGuard` acts like `MutexGuard`, it locks the underlying ring buffer during its lifetime, and provides writing methods to the ring buffer. The lock will be automatically released on dropping of this structure."],["StdioWriter","A writer to stdio buffers."]],"type":[["RingBufferEofRef","A reference to a ring buffer with an EOF mark with mutex protection."]]};
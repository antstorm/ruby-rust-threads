# Ruby and Rust thread contention

This is a small demo that shows a weird relationship between Ruby and Rust threads.

## Setup

1. Run `bundle install` to get Ruby dependencies up
2. Run `bundle exec rake build` to compile Rust part
3. Run `time bundle exec ruby run.rb` to execute the Ruby script and time it

## The issue

Out of the box the execution takes 5 seconds (because of the `thread::sleep` call in the Rust
producer thread), however it is expected to only take 1 second, which is the time the main thread
stays alive for (somehow the Ruby consumer thread is unable to stop before the `rx1.recv()` is
unblocked).

When the Ruby thread is commented out, the code behaves as expected — terminates after 1 second
without waiting for the message to get published.

Removing a Rust consumer thread has no effect, as Ruby is still stuck waiting for the message to get
published.

Removing `Thread::call_without_gvl` will not allow Ruby's VM to preemt the consumer thread
effectively blocking the main Ruby thread and not allowing the program to finish.

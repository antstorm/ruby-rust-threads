require 'rutie'

DIR = File.expand_path('./rust/src', File.dirname(__FILE__))

Rutie.new(:thread_term, release: :debug).init('init_rust', DIR)

puts 'Starting RustRunner'

RustRunner.run

sleep 1

puts 'Ruby main thread finished'

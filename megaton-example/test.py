from __future__ import print_function
import frida
import sys

session = frida.spawn("env CC=clang RUST_TARGET_PATH=. xargo build --target aarch64-roblabla-switch")
script = session.create_script("""
Interceptor.attach(ptr("%s"), {
    onEnter: function(args) {
        send(args[0].toInt32());
    }
});
""")
def on_message(message, data):
    print(message)
script.on('message', on_message)
script.load()
sys.stdin.read()

# Compat with python3
from __future__ import print_function

import sys
import os
if os.environ.get('SWIPC_PATH') is None:
    print("Please set $SWIPC_PATH before running this script")
    exit(0)

sys.path.append(os.environ.get('SWIPC_PATH'))
import idparser
import errno
import argparse
from builtins import input

def mkdir_p(path):
	try:
		os.makedirs(path)
	except OSError as exc:  # Python >2.5
		if exc.errno == errno.EEXIST and os.path.isdir(path):
			pass
		else:
			raise

BUILTINS = {
	"bool": { "len": 1, "rtype": "bool" },
	"u8": { "len": 1, "rtype": "u8" },
	"i8": { "len": 1, "rtype": "i8" },
	"u16": { "len": 2, "rtype": "u16" },
	"u32": { "len": 4, "rtype": "u32" },
	"i32": { "len": 4, "rtype": "i32" },
	"f32": { "len": 4, "rtype": "f32" },
	"u64": { "len": 8, "rtype": "u64" },
	"i64": { "len": 8, "rtype": "i64" },
	"u128": { "len": 16, "rtype": "u128" }, # DERP
}

def camelToSnake(s):
	""" 
	Is it ironic that this function is written in camel case, yet it
	converts to snake case? hmm..
	"""
	import re
	_underscorer1 = re.compile(r'(.)([A-Z][a-z]+)')
	_underscorer2 = re.compile('([a-z0-9])([A-Z])')
	subbed = _underscorer1.sub(r'\1_\2', s)
	return _underscorer2.sub(r'\1_\2', subbed).lower()

def ninty_to_c(s):
	return s.replace("-", "_").replace("::", "_").replace(":", "_")

def findCmd(ifname, id):
	for name, cmd in ifaces[ifname].items():
		if cmd['cmdId'] == id:
			return (name, cmd)
	return None

def emitInt(x):
	return '0x%x' % x if x > 9 else str(x)

class UnsupportedStructException(Exception):
    pass

def getType(output, ty):
	if ty[0] == 'array': # Actually a special type of buffer!
		return '&%s[%s]' % ("mut " if output else "", getType(output, ty[1]))
	elif ty[0] == 'buffer':
		# Blow up if we do not know the size
		if ty[1][0] == "unknown" and ty[3] == 0:
			#ret = 'buffer<%s, %s, %s>' % (getType(output, (None, (None, ty[1]))), emitInt(ty[2]), emitInt(ty[3]))
			raise UnsupportedStructException()
		# Treat as an opaque byte array
		elif ty[1][0] == "unknown":
			return '[u8; %s]' % emitInt(ty[3])
		else:
			# Depending on type of buffer, this will be a const or
			# a mutable reference...
			return ("&mut Option<%s>" if output else "&%s") % getType(output, ty[1])
	elif ty[0] == 'object':
		it = ty[1][0]
		if it in ifaces:
			return '::' + it
		else:
			return "Session"
	elif ty[0] == 'KObject':
		return "KObject"
	elif ty[0] == 'align':
		raise UnsupportedStructException()
		#return 'align<%s, %s>' % (emitInt(ty[1]), getType(output, ty[2]))
	elif ty[0] == 'bytes':
		return '[u8; %s]' % emitInt(ty[1])
	elif ty[0] == 'pid':
		# TODO: maybe I need it in some situations ?
		raise Exception("pid is not a valid type")
	elif ty[0] in types:
		return '::' + ty[0]
	elif ty[0] in BUILTINS:
		assert len(ty) == 1
		return BUILTINS[ty[0]]['rtype']
	else:
		raise Exception("Unknown type")

def formatArgs(elems, is_output=False, is_arg=True):
	from functools import partial

	def sub(output, is_arg, elem):
		idx, elem = elem
		name, ty = elem

		# pids aren't actual argument.
		if ty[0] == 'pid':
			return None

		if name == "type":
			# TODO: EWW
			elem[0] = "ty"
			name = "ty"

		ty = getType(output, ty)

		assert ty is not None

		if is_arg:
			return '%s: %s' % (name, ty)
		else:
			return '%s' % ty

	out = ', '.join(filter(None, map(partial(sub, is_output, is_arg), enumerate(elems))))
	if not is_arg and len(list(elems)) != 1:
		return '(%s)' % out
	else:
		return out

types, ifaces, services = idparser.getAll()
invServices = {svc : ifname for ifname, svcs in services.items() for svc in svcs}
returnedBy = {}
takenBy = {}

for name, cmds in ifaces.items():
	for cmd in cmds['cmds']:
		for _, elem in cmd['inputs']:
			if elem[0] == 'object':
				c = elem[1][0]
				if c not in takenBy:
					takenBy[c] = []
				takenBy[c].append((name, cmd['cmdId']))
		for _, elem in cmd['outputs']:
			if elem[0] == 'object':
				c = elem[1][0]
				if c not in returnedBy:
					returnedBy[c] = []
				returnedBy[c].append((name, cmd['cmdId']))

def raw_input_type(ty):
	# Buffers
	if ty[0] == 'buffer':
		return None
	# Handles (copy/move)
	elif ty[0] == 'object':
		return None
	elif ty[0] == 'KObject':
		return None
	elif ty[0] == 'pid':
		return None
	elif ty[0] == 'array':
		return None
	# Raw
	elif ty[0] == 'align':
		raise UnsupportedStructException("align")
	# Anonymous struct, turn into tuple ?
	elif ty[0] == 'struct':
		raise UnsupportedStructException("struct")
	elif ty[0] == 'bytes':
		raise UnsupportedStructException("bytes")
	elif ty[0] in types:
		#print("\tmemcpy(raw + %d, %s, sizeof(%s));" % (rawoffset, name, ninty_to_c(ty[0])))
		return ty[0]
	elif ty[0] in BUILTINS:
		return BUILTINS[ty[0]]["rtype"]
	else:
		raise Exception("Unknown type %s" % ty[0])

def is_buffer_argument(ty):
        return ty[0] in ['buffer', 'array']

def gen_ipc_method(cmd, f):
	# Get args type
	args_arr = []
	for (idx, (name, ty)) in enumerate(cmd['inputs']):
		if raw_input_type(ty) is not None:
			args_arr.append((name, getType(False, ty)))
	if len(args_arr) == 1:
		args = "%s" % args_arr[0][0]
	elif len(args_arr) == 0:
		args = "()"
	else:
		print("\t\t#[repr(C)] #[derive(Clone)]", file=f)
		print("\t\tstruct InRaw {", file=f)
		for name, ty, in args_arr:
			print("\t\t\t%s: %s," % (name, ty), file=f)
		print("\t\t}", file=f)
		args = "InRaw {\n"
		for name, ty, in args_arr:
			args += "\t\t\t\t%s,\n" % name
		args += "\t\t\t}"


	print("\t\tlet req = Request::new(%d)" % cmd['cmdId'], file=f)

	# even if we have none, put () so the type inference is happy
	print("\t\t\t.args(%s)" % args, file=f)
	if any(ty[0] == "pid" for (_, ty) in cmd['inputs']):
		print("\t\t\t.send_pid()", file=f)
	# TODO: Buffers
	print("\t\t\t;", file=f)

	args_arr = []
	for (idx, (name, ty)) in enumerate(cmd['outputs']):
		if ty[0] in ['buffer', 'object', 'KObject', 'pid', 'array']:
			continue
		args_arr.append((idx, name, getType(True, ty)))
	if len(args_arr) == 1:
		response_args = "%s" % args_arr[0][2]
	elif len(args_arr) == 0:
		response_args = "()"
	else:
		# Generate an anonymous repr(c) struct so we get correct
		# alignment semantics
		print("\t\t#[repr(C)] #[derive(Clone)] struct OutRaw {", file=f)
		for idx, name, ty, in args_arr:
			print("\t\t\t%s: %s," % (name, ty), file=f)
		print("\t\t}", file=f)
		response_args = "OutRaw"

	args_arr = []

	raw_input_len = len(list(filter(lambda x: raw_input_type(x[1]) is not None, cmd['outputs'])))
	for (name, ty) in cmd['outputs']:
		if ty[0] in ['buffer', 'array']: # TODO: What about pid
			continue
		t = raw_input_type(ty)
		if t is not None and raw_input_len == 1:
			args_arr.append("*res.get_raw()")
		elif t is not None:
			args_arr.append("res.get_raw().%s.clone()" % name)
		elif ty[0] == "object":
			args_arr.append("unsafe { FromKObject::from_kobject(res.pop_handle()) }")
		elif ty[0] == "KObject":
			args_arr.append("res.pop_handle()")
		else:
			raise UnsupportedStructException(ty[0])

	print("\t\tlet mut res : Response<%s> = self.0.send(req)?;" % response_args, file=f)
	if len(args_arr) != 1:
		ret = "(%s)" % (",".join(args_arr))
		print("\t\tOk(%s)" % ret, file=f)
	else:
		print("\t\tOk(%s)" % args_arr[0], file=f)

levels = dict()

parser = argparse.ArgumentParser(description='Generate the NN rust crate source')
parser.add_argument('path', nargs='?', action='store', type=str, help='Path of the NN crate. Will create a folder src in it.', default=os.path.realpath('%s/../megaton-ipc' % os.path.dirname(os.path.realpath(__file__))))
prog_args = parser.parse_args()

if os.path.exists(os.path.join(prog_args.path, "src")):
	r = ""
	while r != "Y" and r != "n":
		r = input("Delete %s? (Y/n)" % os.path.join(prog_args.path, "src"))
		if r == "Y":
			import shutil
			shutil.rmtree(os.path.join(prog_args.path, "src"), True)

mkdir_p(os.path.join(prog_args.path, "src"))
with open(os.path.join(prog_args.path, "src", "lib.rs"), 'a') as f:
	print("#![feature(i128_type)]", file=f)
	print("#![no_std]", file=f)
	print("extern crate megaton_hammer;", file=f)
	print("", file=f)

try:
    from StringIO import StringIO
except ImportError:
    from io import StringIO

# Generate IPC
for name, cmds in ifaces.items():
	elmts = name.split("::")
	mkdir_p(os.path.join(prog_args.path, "src", *elmts[:-1]))
	# Add a mod.rs at each level adding a mod.rs
	for i in range(len(elmts)):
		if levels.get(os.path.join(*elmts[:(i + 1)])) is None:
			levels[os.path.join(*elmts[:(i + 1)])] = True
			filename = elmts[:i] + ["mod.rs" if i > 0 else "lib.rs"]
			with open(os.path.join(prog_args.path, "src", *filename), 'a') as f:
				if i != len(elmts) - 1:
					print("pub mod %s;" % elmts[i], file=f)
				else:
					print("mod impl_%s;" % elmts[i], file=f)
					print("pub use self::impl_%s::*;" % elmts[i], file=f)

	# Open the interface and generate the struct
	filename = elmts[:-1] + ["impl_" + elmts[-1] + ".rs"]
	ifacename = elmts[-1]
	print(name)
	with open(os.path.join(prog_args.path, "src", *filename), "w") as f:
		# Print module documentation
		print("", file=f)
		# Use statements
		print("use megaton_hammer::kernel::{FromKObject, KObject, Session};", file=f)
		print("use megaton_hammer::error::Result;", file=f)
		print("use megaton_hammer::ipc::{Request, Response};", file=f)
		print("", file=f)
		print("pub struct %s(Session);" % ifacename, file=f)
		print("", file=f)
		if name in services:
			print("impl %s {" % ifacename, file=f)
			print("\tpub fn get_service() {", file=f)
			# sm: has a different way to get acquired.
			if name == "nn::sm::detail::IUserInterface":
				
			# Use sm
			print("\t\t
		print("impl %s {" % ifacename, file=f)
		for cmd in cmds['cmds']:
			fn_io = StringIO()
			print("    ", cmd['name'])
			# Let's first give a proper unique name to every input/output
			for idx, elem in enumerate(cmd['inputs'] + cmd['outputs']):
				if elem[0] is None:
					elem[0] = 'unk%s' % idx
			try:
				inputs = formatArgs(cmd['inputs'])
				# Handle out buffers
				out_args = formatArgs(filter(lambda x: is_buffer_argument(x[1]), cmd['outputs']), is_output=True)
				outputs = formatArgs(filter(lambda x: not is_buffer_argument(x[1]), cmd['outputs']), is_arg=False, is_output=True)
				args = ", ".join(filter(None, [inputs, out_args]))
				# Added at version X, never removed
				if cmd['versionAdded'] != '1.0.0' and cmd['lastVersion'] is None:
					print("\t#[cfg(feature = \"switch-%s\")]" % cmd['versionAdded'], file=fn_io)
				# Removed at version Y
				elif cmd['versionAdded'] == '1.0.0' and cmd['lastVersion'] is not None:
					versionRemoved = idparser.versionInfo[idparser.versionInfo.index(cmd['lastVersion']) + 1]
					print("\t#[cfg(not(feature = \"switch-%s\"))]" % versionRemoved, file=fn_io)
				elif cmd['versionAdded'] != '1.0.0' and cmd['lastVersion'] is not None:
					versionRemoved = idparser.versionInfo[idparser.versionInfo.index(cmd['lastVersion']) + 1]
					print("\t#[cfg(all(feature = \"switch-%s\", not(feature = \"switch-%s\")))]" % (cmd['versionAdded'], versionRemoved), file=fn_io)
				print("\tpub fn %s(&self, %s) -> Result<%s> {" % (cmd['name'], args, outputs), file=fn_io)
				gen_ipc_method(cmd, fn_io)
				print("\t}", file=fn_io)
				print(fn_io.getvalue(), file=f)
			except UnsupportedStructException as e:
				print("\t// fn %s(&self, UNKNOWN) -> Result<UNKNOWN>;" % cmd['name'], file=f)
		print("}", file=f)
		print("", file=f)
		print("impl FromKObject for %s {" % ifacename, file=f)
		print("\tunsafe fn from_kobject(obj: KObject) -> %s {" % ifacename, file=f)
		print("\t\t%s(Session::from_kobject(obj))" % ifacename, file=f)
		print("\t}", file=f)
		print("}", file=f)


# Generate structs
for name, val in types.items():
	elmts = name.split("::")
	mkdir_p(os.path.join(prog_args.path, "src", *elmts[:-1]))
	# Add a mod.rs at each level adding a mod.rs
	for i in range(len(elmts)):
		if levels.get(os.path.join(*elmts[:(i + 1)])) is None:
			levels[os.path.join(*elmts[:(i + 1)])] = True
			filename = elmts[:i] + ["mod.rs" if i > 0 else "lib.rs"]
			with open(os.path.join(prog_args.path, "src", *filename), 'a') as f:
				if i != len(elmts) - 1:
					print("pub mod %s;" % elmts[i], file=f)

	# Open the interface and generate the struct
	filename = elmts[:-1] + ["mod.rs" if len(elmts) > 1 else "lib.rs"]
	ifacename = elmts[-1]
	print("    ", name)
	with open(os.path.join(prog_args.path, "src", *filename), "a") as f:
		if val[0] == 'struct':
			print("#[repr(C)]", file=f)
			print("#[derive(Debug, Clone)]", file=f)
			print("pub struct %s {" % ifacename, file=f)
			for structField in val[1]:
				print("\t%s: %s," % (structField[0], getType(False, structField[1])), file=f)
			print("}", file=f)
		elif val[0] == 'unknown': # This... kinda sucks.
			print("pub type %s = ();" % ifacename, file=f)
		else:
			print("pub type %s = %s;" % (ifacename, getType(False, val)), file=f)

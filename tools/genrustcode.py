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
	return _underscorer2.sub(r'\1_\2', subbed).lower().replace("__", "_")

def to_camel_case(snake_str):
	"""
	Let's follow the convention: Since this function convets to camel case,
	its name shall be in snake case.
	"""
	components = snake_str.split('_')
	# We capitalize the first letter of each component except the first one
	# with the 'title' method and join them together.
	return ''.join(x[0].upper() + x[1:] for x in components)

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
			return '%s[u8; %s]' % ("&mut " if output else "&", emitInt(ty[3]))
		elif ty[3] != 0:
			# Depending on type of buffer, this will be a const or
			# a mutable reference...
			return ("&mut %s" if output else "&%s") % getType(output, ty[1])
		else:
			return ("&mut [%s]" if output else "&[%s]") % getType(output, ty[1])
	elif ty[0] == 'object':
		it = ty[1][0]
		if it in ifaces:
			ret = '::' + it + "<Session>"
		else:
			ret = "Session"
		if output:
			return ret
		else:
			return "&" + ret
	elif ty[0] == 'KObject':
		if output:
			return "KObject"
		else:
			# TODO: Add support for move object, that take a KObject by-value!
			return "&KObject"
	elif ty[0] == 'align':
		raise UnsupportedStructException()
		#return 'align<%s, %s>' % (emitInt(ty[1]), getType(output, ty[2]))
	elif ty[0] == 'bytes':
		return '[u8; %s]' % emitInt(ty[1])
	elif ty[0] == 'pid':
		# TODO: maybe I need it in some situations ?
		raise Exception("pid is not a valid type")
	elif ty[0] in types:
		return "::".join([""] + ty[0].split("::")[:-1] + [to_camel_case(ty[0].split("::")[-1])])
	elif ty[0] in BUILTINS:
		assert len(ty) == 1
		return BUILTINS[ty[0]]['rtype']
	elif ty[0] == 'unknown':
		raise UnsupportedStructException('unknown')
	else:
		raise Exception("Unknown type %s" % ty[0])
 
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

	input_buf = any(map(lambda (name, ty): ty[0] in ["buffer", "array"], cmd['inputs']))
	output_buf = any(map(lambda (name, ty): ty[0] in ["buffer", "array"], cmd['outputs']))

	if input_buf or output_buf:
		print("\t\tuse megaton_hammer::ipc::IPCBuffer;", file=f)
	print("\t\tuse megaton_hammer::ipc::{Request, Response};", file=f)
	print("", file=f)

	# Get args type
	args_arr = []
	objects_arr = []
	buffers = []
	for (idx, (name, ty)) in enumerate(cmd['inputs']):
		if raw_input_type(ty) is not None:
			args_arr.append((name, getType(False, ty)))
		elif ty[0] == "KObject":
			objects_arr.append(name)
		elif ty[0] == "object":
			objects_arr.append(name + ".as_ref()")
		elif ty[0] == "buffer":
			if ty[3] == 0:
				buffers.append("IPCBuffer::from_slice(%s, %s)" % (name, emitInt(ty[2])))
			else:
				buffers.append("IPCBuffer::from_ref(%s, %s)" % (name, emitInt(ty[2])))
		elif ty[0] == "array":
			buffers.append("IPCBuffer::from_slice(%s, %s)" % (name, emitInt(ty[2])))
		elif ty[0] == "pid":
			pass
		else:
			raise UnsupportedStructException("MEH" + ty[0])

	# Grab output buffers to
	for (name, ty) in cmd['outputs']:
		if ty[0] == "buffer":
			if ty[3] == 0:
				buffers.append("IPCBuffer::from_mut_slice(%s, %s)" % (name, emitInt(ty[2])))
			else:
				buffers.append("IPCBuffer::from_mut_ref(%s, %s)" % (name, emitInt(ty[2])))
		elif ty[0] == "array":
			buffers.append("IPCBuffer::from_mut_slice(%s, %s)" % (name, emitInt(ty[2])))


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
	for obj in objects_arr:
		print("\t\t\t.copy_handle(%s)" % obj, file=f)
	for buf in buffers:
		print("\t\t\t.descriptor(%s)" % buf, file=f)
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
		t = raw_input_type(ty)
		if t is not None and raw_input_len == 1:
			args_arr.append("*res.get_raw()")
		elif t is not None:
			args_arr.append("res.get_raw().%s.clone()" % name)
		elif ty[0] == "object":
			args_arr.append("unsafe { FromKObject::from_kobject(res.pop_handle()) }")
		elif ty[0] == "KObject":
			args_arr.append("res.pop_handle()")
		elif ty[0] == "buffer" or ty[0] == "array":
			pass
		else:
			raise UnsupportedStructException(ty[0])

	is_mut = any(map(lambda (name, ty): ty[0] in ["object", "KObject"], cmd['outputs']))
	is_used = len(list(filter(lambda (name, ty): ty[0] not in ["buffer", "array"], cmd['outputs']))) != 0

	print("\t\tlet %sres : Response<%s> = self.0.send(req)?;" % ("mut " if is_mut else "_" if not is_used else "", response_args), file=f)
	if len(args_arr) != 1:
		ret = "(%s)" % (",".join(args_arr))
		print("\t\tOk(%s)" % ret, file=f)
	else:
		print("\t\tOk(%s)" % args_arr[0], file=f)

def gen_new_method(f, ifacename, servicename):
	s_name = s + ("\\0" * (8 - len(s)))
	print("\t\tuse alloc::arc::Weak;", file=f)
	print("\t\tuse spin::Mutex;", file=f)
	print("\t\tuse core::mem::ManuallyDrop;", file=f)
	print("\t\tlazy_static! {", file=f)
	print("\t\t\tstatic ref HANDLE : Mutex<Weak<%s<Session>>> = Mutex::new(Weak::new());" % ifacename, file=f)
	print("\t\t}", file=f)
	print("\t\tif let Some(hnd) = HANDLE.lock().upgrade() {", file=f)
	print("\t\t\treturn Ok(hnd)", file=f)
	print("\t\t}", file=f)
	print("", file=f)
	if name == "nn::sm::detail::IUserInterface":
		# TODO: Call Initialize
		print("\t\tuse megaton_hammer::kernel::svc;", file=f)
		print("\t\tuse megaton_hammer::error::Error;", file=f)
		print("", file=f)
		print("\t\tif let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b\"%s\") {" % s_name, file=f)
		print("\t\t\tlet ret = Arc::new(%s(ManuallyDrop::into_inner(hnd)));" % ifacename, file=f)
		print("\t\t\t::core::mem::forget(ret.clone());", file=f)
		print("\t\t\t*HANDLE.lock() = Arc::downgrade(&ret);", file=f)
		print("\t\t\treturn Ok(ret);", file=f)
		print("\t\t}", file=f)
		print("", file=f)
		print("\t\tlet mut session = 0;", file=f)
		print("\t\tlet r = unsafe { svc::connect_to_named_port(&mut session, \"sm:\".as_ptr()) };", file=f)
		print("\t\tif r != 0 {", file=f)
		print("\t\t\treturn Err(Error(r))", file=f)
		print("\t\t} else {", file=f)
		print("\t\t\tlet ret = Arc::new(unsafe { %s::from_kobject(KObject::new(session)) });" % ifacename, file=f)
		print("\t\t\t*HANDLE.lock() = Arc::downgrade(&ret);", file=f)
		print("\t\t\treturn Ok(ret);", file=f)
		print("\t\t}", file=f)
	else:
		print("\t\tuse nn::sm::detail::IUserInterface;", file=f)
		print("", file=f)
		print("\t\tlet sm = IUserInterface::new()?;", file=f)
		print("", file=f)
		print("\t\tif let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b\"%s\") {" % s_name, file=f)
		print("\t\t\tlet ret = Arc::new(%s(ManuallyDrop::into_inner(hnd)));" % ifacename, file=f)
		print("\t\t\t::core::mem::forget(ret.clone());", file=f)
		print("\t\t\t*HANDLE.lock() = Arc::downgrade(&ret);", file=f)
		print("\t\t\treturn Ok(ret);", file=f)
		print("\t\t}", file=f)
		print("", file=f)
		print("\t\tlet r = sm.get_service(*b\"%s\").map(|s| Arc::new(unsafe { %s::from_kobject(s) }));" % (s_name, ifacename), file=f)
		print("\t\tif let Ok(service) = r {", file=f)
		print("\t\t\t*HANDLE.lock() = Arc::downgrade(&service);", file=f)
		print("\t\t\treturn Ok(service);", file=f)
		print("\t\t}", file=f)
		print("\t\tr", file=f)

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
	print("#![feature(alloc, i128_type)]", file=f)
	print("#![no_std]", file=f)
	print("extern crate megaton_hammer;", file=f)
	print("extern crate spin;", file=f)
	print("extern crate alloc;", file=f)
	print("#[macro_use]", file=f)
	print("extern crate lazy_static;", file=f)
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
					print("mod impl_%s;" % camelToSnake(elmts[i]), file=f)
					print("pub use self::impl_%s::*;" % camelToSnake(elmts[i]), file=f)

	# Open the interface and generate the struct
	filename = elmts[:-1] + ["impl_" + camelToSnake(elmts[-1]) + ".rs"]
	ifacename = elmts[-1]
	print(name)
	with open(os.path.join(prog_args.path, "src", *filename), "w") as f:
		# Print module documentation
		print("", file=f)
		# Use statements
		print("use megaton_hammer::kernel::{FromKObject, KObject, Session, Domain, Object};", file=f)
		print("use megaton_hammer::error::Result;", file=f)
		print("use core::ops::{Deref, DerefMut};", file=f)
		if name in services:
			print("use alloc::arc::Arc;", file=f)
		# Check if we'll need to send/receive a buffer

		print("", file=f)
		print("#[derive(Debug)]", file=f)
		print("pub struct %s<T>(T);" % ifacename, file=f)
		print("", file=f)
		if name in services:
			print("impl %s<Session> {" % ifacename, file=f)
			for s in services[name]:
				if len(services[name]) == 1:
					print("\tpub fn new() -> Result<Arc<%s<Session>>> {" % ifacename, file=f)
				else:
					print("\tpub fn new_%s() -> Result<Arc<%s<Session>>> {" % (s.replace(":", "_").replace("-", "_"), ifacename), file=f)
				gen_new_method(f, ifacename, s)
				print("\t}", file=f)
			print("}", file=f)
			print("", file=f)

		print("impl<T> Deref for %s<T> {" % ifacename, file=f)
		print("\ttype Target = T;", file=f)
		print("\tfn deref(&self) -> &T {", file=f)
		print("\t\t&self.0", file=f)
		print("\t}", file=f)
		print("}", file=f)

		print("impl<T> DerefMut for %s<T> {" % ifacename, file=f)
		print("\tfn deref_mut(&mut self) -> &mut T {", file=f)
		print("\t\t&mut self.0", file=f)
		print("\t}", file=f)
		print("}", file=f)

		print("impl<T: Object> %s<T> {" % ifacename, file=f)
		for cmd in cmds['cmds']:
			fn_io = StringIO()
			print("    ", cmd['name'])
			# Let's first give a proper unique name to every input/output
			for idx, elem in enumerate(cmd['inputs'] + cmd['outputs']):
				if elem[0] is None:
					elem[0] = 'unk%s' % idx
				elem[0] = camelToSnake(elem[0])
			try:
				if cmd['undocumented'] == True:
					# huge hack. i'm tired.
					raise UnsupportedStructException("")

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
				name = camelToSnake(cmd['name'])
				if name == "move": # move is a keyword in rust
					name = "_move"
				print("\tpub fn %s(&self, %s) -> Result<%s> {" % (name, args, outputs), file=fn_io)
				gen_ipc_method(cmd, fn_io)
				print("\t}", file=fn_io)
				print(fn_io.getvalue(), file=f)
			except UnsupportedStructException as e:
				print("This failed", e)
				print("\t// fn %s(&self, UNKNOWN) -> Result<UNKNOWN>;" % camelToSnake(cmd['name']), file=f)
		print("}", file=f)
		print("", file=f)
		print("impl FromKObject for %s<Session> {" % ifacename, file=f)
		print("\tunsafe fn from_kobject(obj: KObject) -> %s<Session> {" % ifacename, file=f)
		print("\t\t%s(Session::from_kobject(obj))" % ifacename, file=f)
		print("\t}", file=f)
		print("}", file=f)
		print("", file=f)
		print("impl FromKObject for %s<Domain> {" % ifacename, file=f)
		print("\tunsafe fn from_kobject(obj: KObject) -> %s<Domain> {" % ifacename, file=f)
		print("\t\t%s(Domain::from_kobject(obj))" % ifacename, file=f)
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
			print("pub struct %s {" % to_camel_case(ifacename), file=f)
			for structField in val[1]:
				for line in structField[2]:
					print("\t/// %s" % line, file=f)
				print("\tpub %s: %s," % (camelToSnake(structField[0]), getType(False, structField[1])), file=f)
			print("}", file=f)
		elif val[0] == 'unknown': # This... kinda sucks.
			print("pub type %s = ();" % to_camel_case(ifacename), file=f)
		else:
			print("pub type %s = %s;" % (to_camel_case(ifacename), getType(False, val)), file=f)

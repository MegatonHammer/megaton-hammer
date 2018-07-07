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
        "uint8_t": { "len": 1, "rtype": "u8" },
}

kobject_types = ["KObject", "KHandle"]

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
		underlying_type = "T" if output else "Session"
		if it in ifaces:
			ret = '::ipcdefs::' + it + "<" + underlying_type + ">"
		else:
			ret = underlying_type
		if output:
			return ret
		else:
			return "&" + ret
	elif ty[0] in kobject_types:
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
		return "::ipcdefs" + "::".join([""] + ty[0].split("::")[:-1] + [to_camel_case(ty[0].split("::")[-1])])
	elif ty[0] in BUILTINS:
		assert len(ty) == 1
		return BUILTINS[ty[0]]['rtype']
	elif ty[0] == 'unknown':
		raise UnsupportedStructException('unknown')
	else:
		raise Exception("Unknown type %s" % ty[0])
 

IS_NAME = 1
IS_NAME_PARENS = 4
IS_TYPE = 2
IS_TYPE_PARENS = 5
IS_TYPE_NAME = 3

def formatArgs(elems, is_output=False, format_ty=IS_TYPE_NAME):
	from functools import partial

	def sub(output, format_ty, elem):
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

		if format_ty == IS_TYPE_NAME:
			return '%s: %s' % (name, ty)
		elif format_ty == IS_NAME:
			return '%s' % name
		else:
			return '%s' % ty

	return ', '.join(filter(None, map(partial(sub, is_output, format_ty), enumerate(elems))))

def formatInputs(cmd, format_ty=IS_TYPE_NAME):
	for idx, elem in enumerate(cmd['inputs'] + cmd['outputs']):
		if elem[0] is None:
			elem[0] = 'unk%s' % idx
		elem[0] = camelToSnake(elem[0])
	inputs = formatArgs(cmd['inputs'], format_ty=format_ty)
	# Handle out buffers
	out_args = formatArgs(filter(lambda x: is_buffer_argument(x[1]), cmd['outputs']), is_output=True, format_ty=format_ty)
	args = ", ".join(filter(None, [inputs, out_args]))

	if format_ty == IS_NAME_PARENS or format_ty == IS_TYPE_PARENS:
		return '(%s)' % args
	else:
		return args

def formatOutputs(cmd):
	outputs = formatArgs(filter(lambda x: not is_buffer_argument(x[1]), cmd), is_output=True, format_ty=IS_TYPE)
	if outputs == "" or "," in outputs:
		return "(%s)" % outputs
	else:
		return outputs

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
	elif ty[0] in kobject_types:
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
		print("\t\tuse ::ipc::IPCBuffer;", file=f)
	print("\t\tuse ::ipc::{Request, Response};", file=f)
	print("", file=f)

	# Get args type
	args_arr = []
	objects_arr = []
	buffers = []
	for (idx, (name, ty)) in enumerate(cmd['inputs']):
		if raw_input_type(ty) is not None:
			args_arr.append((name, getType(False, ty)))
		elif ty[0] in kobject_types:
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


        print("\t\tlet req : Request<_, [_; %d], [_; %d], [_; 0]> = Request::new(%d)" % (len(buffers), len(objects_arr), cmd['cmdId']), file=f)

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
		if ty[0] in ['buffer', 'object', 'pid', 'array'] + kobject_types:
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
			args_arr.append("T::from_res(&mut res).into()")
		elif ty[0] in kobject_types:
			args_arr.append("res.pop_handle()")
		elif ty[0] == "buffer" or ty[0] == "array":
			pass
		else:
			raise UnsupportedStructException(ty[0])

	is_mut = any(map(lambda (name, ty): ty[0] in ["object"] + kobject_types, cmd['outputs']))
	is_used = len(list(filter(lambda (name, ty): ty[0] not in ["buffer", "array"], cmd['outputs']))) != 0

	print("\t\tlet %sres : Response<%s> = self.0.send(req)?;" % ("mut " if is_mut else "_" if not is_used else "", response_args), file=f)
	if len(args_arr) != 1:
		ret = "(%s)" % (",".join(args_arr))
		print("\t\tOk(%s)" % ret, file=f)
	else:
		print("\t\tOk(%s)" % args_arr[0], file=f)

def gen_raw_new_method(f, ifacename, servicename, has_initialize_output, initialize_args):
	s_name = s + ("\\0" * (8 - len(s)))
	if name == "nn::sm::detail::IUserInterface":
		# TODO: Call Initialize
		print("\t\tuse ::kernel::svc;", file=f)
		print("", file=f)
		print("\t\tlet session = unsafe { svc::connect_to_named_port(\"sm:\\0\".as_ptr())? };", file=f)
		print("\t\tlet ret = Session::from(unsafe { KObject::new(session) }).into();", file=f)
		print("\t\tOk(ret)", file=f)
	else:
                # TODO: Always creating a connection to sm kinda sucks...
		print("\t\tuse ::ipcdefs::nn::sm::detail::IUserInterface;", file=f)
		print("", file=f)
		print("\t\tlet sm = IUserInterface::raw_new()?;", file=f)
		print("", file=f)
		print("\t\tlet session = sm.get_service(*b\"%s\")?;" % s_name, file=f)
		print("\t\tlet object : Self = Session::from(session).into();", file=f)
		if has_initialize_output == "()":
			print("\t\tobject.initialize(%s)?;" % initialize_args, file=f)
		elif has_initialize_output == "u32":
			print("\t\tlet errcode = object.initialize(%s)?;" % initialize_args, file=f)
			print("\t\tif errcode != 0 {", file=f)
			print("\t\t\treturn Err(Error(1))", file=f)
			print("\t\t}", file=f)
		print("\t\tOk(object)", file=f)

def gen_new_method(f, ifacename, servicename, raw_new_name, has_initialize_output, initialize_args):
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
	print("\t\tif let Some(hnd) = ::loader::get_override_service(*b\"%s\") {" % s_name, file=f)
	print("\t\t\tlet ret = Arc::new(%s(ManuallyDrop::into_inner(hnd)));" % ifacename, file=f)
	print("\t\t\t::core::mem::forget(ret.clone());", file=f)
	print("\t\t\t*HANDLE.lock() = Arc::downgrade(&ret);", file=f)
	print("\t\t\treturn Ok(ret);", file=f)
	print("\t\t}", file=f)
	print("", file=f)
	if has_initialize_output == "u32" or has_initialize_output == "()" and initialize_args != "":
		print("\t\tlet hnd = f(Self::%s)?;" % raw_new_name, file=f)
	else:
		print("\t\tlet hnd = Self::%s(%s)?;" % (raw_new_name, initialize_args), file=f)
	print("\t\tlet ret = Arc::new(hnd);", file=f)
	print("\t\t*HANDLE.lock() = Arc::downgrade(&ret);", file=f)
	print("\t\tOk(ret)", file=f)

levels = dict()

parser = argparse.ArgumentParser(description='Generate the NN rust crate source')
parser.add_argument('path', nargs='?', action='store', type=str, help='Path of the NN crate. Will create a folder src in it.', default=os.path.realpath('%s/../megaton-hammer/src/ipcdefs' % os.path.dirname(os.path.realpath(__file__))))
prog_args = parser.parse_args()

if os.path.exists(os.path.join(prog_args.path)):
	r = ""
	while r != "Y" and r != "n":
		r = input("Delete %s? (Y/n)" % os.path.join(prog_args.path))
		if r == "Y":
			import shutil
			shutil.rmtree(os.path.join(prog_args.path), True)

mkdir_p(os.path.join(prog_args.path))
#with open(os.path.join(prog_args.path, "mod.rs"), 'a') as f:
	#print("#![feature(alloc, i128_type)]", file=f)
	#print("#![no_std]", file=f)
	#print("extern crate megaton_hammer;", file=f)
	#print("extern crate spin;", file=f)
	#print("extern crate alloc;", file=f)
	#print("#[macro_use]", file=f)
	#print("extern crate lazy_static;", file=f)
	#print("", file=f)

try:
    from StringIO import StringIO
except ImportError:
    from io import StringIO

# Generate IPC
for name, cmds in ifaces.items():
	elmts = name.split("::")
	mkdir_p(os.path.join(prog_args.path, *elmts[:-1]))
	# Add a mod.rs at each level adding a mod.rs
	for i in range(len(elmts)):
		if levels.get(os.path.join(*elmts[:(i + 1)])) is None:
			levels[os.path.join(*elmts[:(i + 1)])] = True
			filename = elmts[:i] + ["mod.rs"]
			with open(os.path.join(prog_args.path, *filename), 'a') as f:
				if i != len(elmts) - 1:
					print("pub mod %s;" % elmts[i], file=f)
				else:
					print("mod impl_%s;" % camelToSnake(elmts[i]), file=f)
					print("pub use self::impl_%s::*;" % camelToSnake(elmts[i]), file=f)

	# Open the interface and generate the struct
	filename = elmts[:-1] + ["impl_" + camelToSnake(elmts[-1]) + ".rs"]
	ifacename = elmts[-1]
	print(name)
	with open(os.path.join(prog_args.path, *filename), "w") as f:
		# Print module documentation
		print("", file=f)
		# Use statements
		print("use ::kernel::{Session, Domain, Object};", file=f)
		print("#[allow(unused_imports)]", file=f)
		print("use ::kernel::KObject;", file=f)
		print("use ::error::*;", file=f)
		print("use core::ops::{Deref, DerefMut};", file=f)
		if name in services:
			print("use alloc::arc::Arc;", file=f)
		# Check if we'll need to send/receive a buffer

		print("", file=f)
		print("#[derive(Debug)]", file=f)
		print("pub struct %s<T>(T);" % ifacename, file=f)
		print("", file=f)
		print("impl %s<Session> {" % ifacename, file=f)
		for s in services.get(name, []):
			initialize_method = next((cmd for cmd in cmds['cmds'] if cmd['name'] == "Initialize"), None)
			if s != "sm:" and initialize_method is not None:
				args = formatInputs(initialize_method)
				args_names = formatInputs(initialize_method, format_ty=IS_NAME)
				args_ty = formatInputs(initialize_method, format_ty=IS_TYPE)
				outputs = formatOutputs(initialize_method['outputs'])
				if outputs != "()" and outputs != "u32":
					(args, outputs) = ("", None)
			else:
				(args, outputs, args_names, args_ty) = ("", None, "", "")

			new_method_name = "new" if len(services[name]) == 1 else "new_%s" % s.replace(":", "_").replace("-", "_")
			# Handle out buffers
			print("\tpub fn raw_%s(%s) -> Result<%s<Session>> {" % (new_method_name, args, ifacename), file=f)
			gen_raw_new_method(f, ifacename, s, outputs, args_names)
			print("\t}", file=f)
			print("", file=f)
			if args != "":
				args_ty_new = []
				# TODO: Lifetimes are going to be a giant pain...
				fn_params = []
				#for i in args.split(", "):
				#	(arg_name, ty) = i.split(": ")
				#	if ty == "&KObject":
				#		fn_params.append("'" + arg_name)
				#		args_ty_new.append("&%s KObject" % ("'" + arg_name))
				#	else:
				#		args_ty_new.append("%s" % ty)
				#if len(args_ty_new) == 1:
				#	args_ty_new = args_ty_new[0]
				#else:
				#	args_ty_new = "(" + ", ".join(args_ty_new) + ")"
				fn_params.append("T: FnOnce(fn(%s) -> Result<%s<Session>>) -> Result<%s<Session>>" % (args_ty, ifacename, ifacename))
				print("\tpub fn %s<%s>(f: T) -> Result<Arc<%s<Session>>> {" % (new_method_name, ", ".join(fn_params), ifacename), file=f)
			else:
				print("\tpub fn %s() -> Result<Arc<%s<Session>>> {" % (new_method_name, ifacename), file=f)
			gen_new_method(f, ifacename, s, "raw_" + new_method_name, outputs, args_names)
			print("\t}", file=f)
			print("", file=f)

		print("\tpub fn to_domain(self) -> ::core::result::Result<%s<Domain>, (Self, Error)> {" % ifacename, file=f)
		print("\t\tmatch self.0.to_domain() {", file=f)
		print("\t\t\tOk(domain) => Ok(%s(domain))," % ifacename, file=f)
		print("\t\t\tErr((sess, err)) => Err((%s(sess), err))" % ifacename, file=f)
		print("\t\t}", file=f)
		print("\t}", file=f)
		print("", file=f)
		print("\tpub fn duplicate(&self) -> Result<%s<Session>> {" % ifacename, file=f)
		print("\t\tOk(%s(self.0.duplicate()?))" % ifacename, file=f)
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
			try:
				if cmd['undocumented'] == True:
					# huge hack. i'm tired.
					raise UnsupportedStructException("")

				args = formatInputs(cmd)
				outputs = formatOutputs(cmd['outputs'])
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
		# TODO: Think about the safety implications of this.
		print("impl<T: Object> From<T> for %s<T> {" % ifacename, file=f)
		print("\tfn from(obj: T) -> %s<T> {" % ifacename, file=f)
		print("\t\t%s(obj)" % ifacename, file=f)
		print("\t}", file=f)
		print("}", file=f)


# Generate structs
for name, val in types.items():
	elmts = name.split("::")
	mkdir_p(os.path.join(prog_args.path, *elmts[:-1]))
	# Add a mod.rs at each level adding a mod.rs
	for i in range(len(elmts)):
		if levels.get(os.path.join(*elmts[:(i + 1)])) is None:
			levels[os.path.join(*elmts[:(i + 1)])] = True
			filename = elmts[:i] + ["mod.rs"]
			with open(os.path.join(prog_args.path, *filename), 'a') as f:
				if i != len(elmts) - 1:
					print("pub mod %s;" % elmts[i], file=f)

	# Open the interface and generate the struct
	filename = elmts[:-1] + ["mod.rs"]
	ifacename = elmts[-1]
	print("    ", name)
	with open(os.path.join(prog_args.path, *filename), "a") as f:
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

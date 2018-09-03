
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IManagerDisplayService<T>(T);

impl IManagerDisplayService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IManagerDisplayService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IManagerDisplayService(domain)),
			Err((sess, err)) => Err((IManagerDisplayService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IManagerDisplayService<Session>> {
		Ok(IManagerDisplayService(self.0.duplicate()?))
	}
}

impl<T> Deref for IManagerDisplayService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IManagerDisplayService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IManagerDisplayService<T> {
	// fn allocate_process_heap_block(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn free_process_heap_block(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_display_resolution(&self, unk0: u64) -> Result<(i64, i64)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1102)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i64,
			unk2: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn create_managed_layer(&self, unk0: u32, unk1: u64, unk2: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<u64> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
			unk2: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2010)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn destroy_managed_layer(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2011)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_indirect_layer(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2050)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn destroy_indirect_layer(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2051)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_indirect_producer_end_point(&self, unk0: u64, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<u64> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2052)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn destroy_indirect_producer_end_point(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2053)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_indirect_consumer_end_point(&self, unk0: u64, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<u64> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2054)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn destroy_indirect_consumer_end_point(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2055)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_layer_texture_presenting_event(&self, unk0: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2300)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn release_layer_texture_presenting_event(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2301)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_display_hotplug_event(&self, unk0: u64) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2302)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_display_hotplug_state(&self, unk0: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2402)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_compositor_error_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_display_error_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_display_alpha(&self, unk0: f32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4201)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_display_layer_stack(&self, unk0: u32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_display_power_state(&self, unk0: u32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_default_display(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn add_to_layer_stack(&self, unk0: u32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn remove_from_layer_stack(&self, unk0: u32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6001)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_layer_visibility(&self, unk0: bool, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6002)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_layer_config(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn attach_layer_presentation_tracer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn detach_layer_presentation_tracer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_layer_presentation_recording(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn stop_layer_presentation_recording(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_layer_presentation_fence_wait(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn stop_layer_presentation_fence_wait(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_layer_presentation_all_fences_expired_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_content_visibility(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7000)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_conductor_layer(&self, unk0: bool, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_indirect_producer_flip_offset(&self, unk0: u64, unk1: u64, unk2: ::ipcdefs::nn::TimeSpan) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: ::ipcdefs::nn::TimeSpan,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8100)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_shared_buffer_static_storage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_shared_buffer_transfer_memory(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn destroy_shared_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn bind_shared_low_level_layer_to_managed_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn bind_shared_low_level_layer_to_indirect_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unbind_shared_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn connect_shared_low_level_layer_to_shared_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn disconnect_shared_low_level_layer_from_shared_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_shared_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn destroy_shared_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn attach_shared_layer_to_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn force_detach_shared_layer_from_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_detach_shared_layer_from_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn finish_detach_shared_layer_from_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_layer_detach_ready_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_low_level_layer_synchronized_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn check_shared_low_level_layer_synchronized(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_shared_buffer_importer_aruid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unregister_shared_buffer_importer_aruid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_shared_buffer_process_heap(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_layer_layer_stacks(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_shared_layer_layer_stacks(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn present_detached_shared_frame_buffer_to_low_level_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn fill_detached_shared_frame_buffer_color(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_detached_shared_frame_buffer_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_detached_shared_frame_buffer_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn copy_detached_shared_frame_buffer_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_detached_shared_frame_buffer_sub_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_shared_frame_buffer_content_parameter(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn expand_startup_logo_on_shared_frame_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IManagerDisplayService<T> {
	fn from(obj: T) -> IManagerDisplayService<T> {
		IManagerDisplayService(obj)
	}
}

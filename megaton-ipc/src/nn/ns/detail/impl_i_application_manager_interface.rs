
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IApplicationManagerInterface(Session);

impl AsRef<Session> for IApplicationManagerInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IApplicationManagerInterface {
	pub fn list_application_record(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn generate_application_record_count(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_application_record_update_system_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_application_view_deprecated(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_application_entity(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_application_completely(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_any_application_entity_redundant(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn delete_redundant_application_entity(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_application_entity_movable(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn move_application_entity(&self, unk0: u8, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn calculate_application_occupied_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn push_application_record(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_record_content_meta(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn launch_application(&self, unk0: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_application_content_path(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate_application(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resolve_application_content_path(&self, unk0: u8, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req = Request::new(23)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn begin_install_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_application_record(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(27)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_application_update_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_application_download(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(32)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resume_application_download(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(33)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn update_version_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn push_launch_version(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(36)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_required_version(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn check_application_launch_version(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(38)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn check_application_launch_rights(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(39)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_logo_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn calculate_application_download_required_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(41)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cleanup_sd_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(42)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn check_sd_card_mount_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_sd_card_mount_status_changed_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(44)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_game_card_attachment_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(45)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_game_card_attachment_info(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(46)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_total_space_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(47)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_space_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(48)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_sd_card_removed_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(49)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_game_card_update_detection_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(52)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn disable_application_auto_delete(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(53)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_application_auto_delete(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(54)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_desired_language(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(55)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_application_terminate_result(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(56)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_application_terminate_result(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(57)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_last_sd_card_mount_unexpected_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(58)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn convert_application_language_to_language_code(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(59)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn convert_language_code_to_application_language(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_background_download_stress_task_info(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_game_card_stopper(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(62)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn is_system_program_installed(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(63)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start_apply_delta_task(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(64)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_request_server_stopper(&self, ) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(65)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_background_apply_delta_stress_task_info(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(66)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn cancel_application_apply_delta(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(67)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resume_application_apply_delta(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(68)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn calculate_application_apply_delta_required_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(69)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resume_all(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(70)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_storage_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(71)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_download_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(80)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_download_add_on_content(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(81)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn download_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(82)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn check_application_resume_rights(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(83)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_dynamic_commit_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(84)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_update_application2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(85)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_application_crash_report(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(86)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_application_crash_report_enabled(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(87)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn boost_system_memory_resource_limit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(90)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_to_factory_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_to_factory_settings_without_user_save_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reset_to_factory_settings_for_refurbishment(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn calculate_user_save_data_statistics(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_user_save_data_all(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(201)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_user_system_save_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(210)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_network_service_account(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(220)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_shell_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(300)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn pop_application_shell_event_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn launch_library_applet(&self, unk0: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(302)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn terminate_library_applet(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(303)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn launch_system_applet(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(304)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn terminate_system_applet(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(305)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn launch_overlay_applet(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(306)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn terminate_overlay_applet(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(307)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_control_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(400)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn invalidate_all_application_control_cache(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(401)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_download_application_control_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(402)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_max_application_control_cache_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(403)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn invalidate_application_control_cache(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(404)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_application_control_cache_entry_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_check_game_card_registration(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(502)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_game_card_registration_gold_point(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(503)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_register_game_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(504)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_card_mount_failure_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(505)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn is_game_card_inserted(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(506)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ensure_game_card_access(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(507)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_last_game_card_mount_failure_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(508)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_id_on_game_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(509)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn count_application_content_meta(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(600)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_application_content_meta_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(601)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_available_add_on_content(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(602)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_owned_application_content_meta_status(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(603)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_contents_external_key(&self, unk0: u64, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(604)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_content_meta_status_with_rights_check(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(605)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_content_meta_storage(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(606)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn push_download_task_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_task_status_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(701)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_download_task_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(702)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_ensure_download_task(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(703)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_download_task_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_download_task_list_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(705)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_version_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(800)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_version_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_version_list_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(802)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_record(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(900)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_record_property(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(901)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_application_auto_update(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(902)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_application_auto_update(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(903)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn touch_application(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(904)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_application_update(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(905)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_application_update_requested(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(906)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn withdraw_application_update_request(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(907)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_record_installed_content_meta(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(908)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn withdraw_cleanup_add_on_contents_with_no_rights_recommendation(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(909)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_verify_application_deprecated(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1000)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn corrupt_application_for_debug(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_verify_add_on_contents_rights(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1002)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_verify_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1003)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn corrupt_content_for_debug(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1004)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn needs_update_vulnerability(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1200)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_any_application_entity_installed(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1300)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn delete_application_content_entities(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1301)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cleanup_unrecorded_application_entity(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1302)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cleanup_add_on_contents_with_no_rights(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1303)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_application_content_entity(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1304)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn try_delete_running_application_entity(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1305)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn try_delete_running_application_completely(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1306)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn try_delete_running_application_content_entities(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1307)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn prepare_shutdown(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1400)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn format_sd_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1500)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn needs_system_update_to_format_sd_card(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1501)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_last_sd_card_format_unexpected_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1502)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn insert_sd_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1504)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn remove_sd_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1505)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_system_seed_for_pseudo_device_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn reset_system_seed_for_pseudo_device_id(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1601)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_application_downloading_content_meta(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1700)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_view(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1701)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_download_task_status(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1702)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_application_view_download_error_context(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1703)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_notification_setup_completed(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1800)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_last_notification_info_count(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1801)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_last_notification_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_notification_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_active_account(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1900)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn request_download_application_prepurchased_rights(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1901)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_ticket_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1902)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_system_delivery_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2000)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn select_latest_system_delivery_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn verify_delivery_protocol_version(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2002)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_delivery_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2003)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn has_all_contents_to_deliver(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2004)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn compare_application_delivery_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2005)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn can_deliver_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2006)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_content_meta_key_to_deliver_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2007)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn needs_system_update_to_deliver_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2008)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn estimate_required_size(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2009)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_receive_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2010)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn commit_receive_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2011)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_receive_application_progress(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2012)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_send_application(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2013)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_send_application_progress(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2014)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn compare_system_delivery_info(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2015)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn list_not_committed_content_meta(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2016)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_download_task(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2017)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IApplicationManagerInterface {
	unsafe fn from_kobject(obj: KObject) -> IApplicationManagerInterface {
		IApplicationManagerInterface(Session::from_kobject(obj))
	}
}

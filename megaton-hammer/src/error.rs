use core::fmt;
use failure;
use enum_primitive::FromPrimitive;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Error(pub u32);

impl Error {
    pub fn from_module_description(module: Module, description: u32) -> Error {
        assert!(module != Module::__AnExtraHiddenParam);
        Error::from_ids(module as u32, description)
    }

    fn from_ids(module: u32, description: u32) -> Error {
        assert_eq!(description & !((1 << 13) - 1), 0, "Description is more than 13 bit long");
        assert_eq!(module & !((1 << 9) - 1), 0, "Module is more than 9 bit long");
        Error(module | description << 9)
    }

    // TODO: eww. Don't do that.
    pub fn module_id(&self) -> u32 {
        self.0 & ((1 << 9) - 1)
    }

    pub fn module(&self) -> ::core::result::Result<Module, u32> {
        match Module::from_u32(self.module_id()) {
            Some(Module::__AnExtraHiddenParam) => Err(0),
            Some(m) => Ok(m),
            None => Err(self.module_id())
        }
    }

    pub fn description_id(&self) -> u32 {
        self.0 >> 9
    }
}

#[cfg(feature = "std")]
impl From<Error> for ::std::io::Error {
    fn from(err: Error) -> ::std::io::Error {
        ::std::io::Error::from_raw_os_error(err.0 as i32)
    }
}

impl failure::Fail for Error { }

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.module() {
            Ok(m) =>
                write!(f, "Error {:x} in module {:?}: {}", self.0, m, self.description_id()),
            Err(m) =>
                write!(f, "Error {:x} in module {}: {}", self.0, m, self.description_id())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Actually print the description if we have it.
        match self.module() {
            Ok(m) =>
                write!(f, "Error {:x} in module {:?}: {}", self.0, m, self.description_id()),
            Err(m) =>
                write!(f, "Error {:x} in module {}: {}", self.0, m, self.description_id())
        }
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;

enum_from_primitive! {
    #[repr(u16)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Module {
        #[doc(hidden)]
        __AnExtraHiddenParam = 0,
        Kernel = 1,
        FS = 2,
        OS = 3,
        HTCS = 4,
        NCM = 5,
        DD = 6,
        DebugMonitor = 7,
        LR = 8,
        Loader = 9,
        CMIF = 10,
        HIPC = 11,
        PM = 15,
        NS = 16,
        Sockets = 17,
        HTC = 18,
        NCMContent = 20,
        SM = 21,
        ROuserland = 22,
        SDMMC = 24,
        OVLN = 25,
        SPL = 26,
        ETHC = 100,
        I2C = 101,
        GPIO = 102,
        UART = 103,
        Settings = 105,
        WLAN = 107,
        XCD = 108,
        NIFM = 110,
        Hwopus = 111,
        Bluetooth = 113,
        VI = 114,
        NFP = 115,
        Time = 116,
        FGM = 117,
        OE = 118,
        PCIe = 120,
        Friends = 121,
        BCAT = 122,
        SSL = 123,
        Account = 124,
        News = 125,
        Mii = 126,
        NFC = 127,
        AM = 128,
        PlayReport = 129,
        AHID = 130,
        HomeMenu = 132,
        PCV = 133,
        OMM = 134,
        BPC = 135,
        PSM = 136,
        NIM = 137,
        PSC = 138,
        TC = 139,
        USB = 140,
        NSD = 141,
        PCTL = 142,
        BTM = 143,
        EC = 144,
        ETicket = 145,
        NGC = 146,
        ErrorReport = 147,
        APM = 148,
        Profiler = 150,
        ErrorUpload = 151,
        Audio = 153,
        NPNS = 154,
        NPNSHTTPStream = 155,
        ARP = 157,
        SWKBD = 158,
        Boot = 159,
        NFCMifare = 161,
        Userlandassert = 162,
        Fatal = 163,
        NIMShop = 164,
        SPSM = 165,
        BGTC = 167,
        Userlandcrash = 168,
        SREPO = 180,
        Dauth = 181,
        HID = 202,
        LDN = 203,
        Irsensor = 205,
        Capture = 206,
        Manu = 208,
        ATK = 209,
        Web = 210,
        GRC = 212,
        Migration = 216,
        MigrationIdcServer = 217,
        Libtransistor = 221,
        Libnx = 345,
        HomebrewABI = 346,
        HomebrewLoader = 347,
        LibnxNvidiaErrors = 348,
        MegatonHammer = 450,
        MegatonHammerLinux = 451,
        GeneralWebApplet = 800,
        WifiWebAuthApplet = 809,
        WhitelistedApplet = 810,
        ShopN = 811,
    }
}

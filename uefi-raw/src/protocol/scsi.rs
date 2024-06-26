use core::ffi::c_void;

use crate::{guid, Event, Guid, Status};

/// TODO: use #define TARGET_MAX_BYTES 0x10, the limit of target.
#[allow(unused)]
const TARGET_MAX_BYTES: u32 = 0x10;

newtype_enum! {
    /// DataDirection
    #[derive(Default)]
    pub enum DataDirection: u8 => {
        READ            = 0,
        WRITE           = 1,
        BIDIRECTIONAL   = 2,
    }
}

newtype_enum! {
    /// HostAdapterStatus
    #[derive(Default)]
    pub enum HostAdapterStatus: u8 => {

        /// EFI_SCSI_IO_STATUS_HOST_ADAPTER_OK
       OK                    = 0x00,
       TIMEOUT_COMMAND       = 0x09,
       TIMEOUT               = 0x0b,
       MESSAGE_REJECT        = 0x0d,
       BUS_RESET             = 0x0e,
       PARITY_ERROR          = 0x0f,
       REQUEST_SENSE_FAILED  = 0x10,
       SELECTION_TIMEOUT     = 0x11,
       DATA_OVERRUN_UNDERRUN = 0x12,
       BUS_FREE              = 0x13,
       PHASE_ERROR           = 0x14,
       OTHER                 = 0x7f,
    }
}

newtype_enum! {
    /// TargetStatus
    #[derive(Default)]
    pub enum TargetStatus: u8 => {
        /// EFI_SCSI_IO_STATUS_TARGET_GOOD
        GOOD                         = 0x00,
        CHECK_CONDITION              = 0x02,
        CONDITION_MET                = 0x04,
        BUSY                         = 0x08,
        INTERMEDIATE                 = 0x10,
        INTERMEDIATE_CONDITION_MET   = 0x14,
        RESERVATION_CONFLICT         = 0x18,
        COMMAND_TERMINATED           = 0x22,
        QUEUE_FULL                   = 0x28,
    }
}

/// EFI_SCSI_IO_SCSI_REQUEST_PACKET
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ScsiIoScsiRequestPacket {
    /// Timeout: The timeout, in 100 ns units, to use for the execution of this SCSI Request Packet.
    /// A Timeout value of 0 means that this function will wait indefinitely for the SCSI Request Packet to execute.
    /// If Timeout is greater than zero, then this function will return EFI_TIMEOUT if the time required to execute the SCSI Request Packet is greater than Timeout .
    pub timeout: u64,

    /// []DataBuffer: A pointer to the data buffer to transfer from or to the SCSI device.
    /// InDataBuffer: A pointer to the data buffer to transfer between the SCSI controller and the SCSI device for SCSI READ command.
    /// For all SCSI WRITE Commands this must point to NULL .
    pub in_data_buffer: *mut c_void,
    /// OutDataBuffer: A pointer to the data buffer to transfer between the SCSI controller and the SCSI device for SCSI WRITE command.
    /// For all SCSI READ commands this field must point to NULL .
    pub out_data_buffer: *mut c_void,
    /// SenseData: A pointer to the sense data that was generated by the execution of the SCSI Request Packet.
    pub sense_data: *mut c_void,
    /// Cdb: A pointer to buffer that contains the Command Data Block to send to the SCSI device.
    pub cdb: *mut c_void,

    /// InTransferLength: On Input, the size, in bytes, of InDataBuffer .
    /// On output, the number of bytes transferred between the SCSI controller and the SCSI device.
    /// If InTransferLength is larger than the SCSI controller can handle, no data will be transferred,
    /// InTransferLength will be updated to contain the number of bytes that the SCSI controller is able to transfer, and EFI_BAD_BUFFER_SIZE will be returned.
    pub in_transfer_length: u32,
    /// OutTransferLength: On Input, the size, in bytes of OutDataBuffer .
    /// On Output, the Number of bytes transferred between SCSI Controller and the SCSI device.
    /// If OutTransferLength is larger than the SCSI controller can handle, no data will be transferred,
    /// OutTransferLength will be updated to contain the number of bytes that the SCSI controller is able to transfer, and EFI_BAD_BUFFER_SIZE will be returned.
    pub out_transfer_length: u32,

    /// CdbLength: The length, in bytes, of the buffer Cdb .
    /// The standard values are 6, 10, 12, and 16, but other values are possible if a variable length CDB is used.
    pub cdb_length: u8,
    /// DataDirection: The direction of the data transfer. 0 for reads, 1 for writes.
    /// A value of 2 is Reserved for Bi-Directional SCSI commands. For example XDREADWRITE.
    /// All other values are reserved, and must not be used.
    pub data_direction: DataDirection,
    /// HostAdapterStatus: The status of the SCSI Host Controller that produces the SCSI bus
    /// where the SCSI device attached when the SCSI Request Packet was executed on the SCSI Controller.
    pub host_adapter_status: HostAdapterStatus,
    /// TargetStatus: The status returned by the SCSI device when the SCSI Request Packet was executed.
    pub target_status: TargetStatus,
    /// SenseDataLength: On input, the length in bytes of the SenseData buffer.
    /// On output, the number of bytes written to the SenseData buffer.
    pub sense_data_length: u8,
}

newtype_enum! {
    /// DeviceType
    /// Defined in the SCSI Primary Commands standard (e.g., SPC-4)
    #[derive(Default)]
    pub enum DeviceType: u8  => {
        DISK              = 0x00, // Disk device
        TAPE              = 0x01, // Tape device
        PRINTER           = 0x02,// Printer
        PROCESSOR         = 0x03,// Processor
        WORM              = 0x04,// Write-once read-multiple
        CDROM             = 0x05,// CD or DVD device
        SCANNER           = 0x06,// Scanner device
        OPTICAL           = 0x07,// Optical memory device
        MEDIUMCHANGER     = 0x08,// Medium Changer device
        COMMUNICATION     = 0x09,// Communications device


        MFI_A               =   0x0A, // Obsolete
        MFI_B               =   0x0B, // Obsolete
        MFI_RAID            =   0x0C, // Storage array controller
        MFI_SES             =   0x0D, // Enclosure services device
        MFI_RBC             =   0x0E, // Simplified direct-access
        MFI_OCRW            =   0x0F, // Optical card reader/
        MFI_BRIDGE          =   0x10, // Bridge Controller
        MFI_OSD             =   0x11, // Object-based Storage

        RESERVED_LOW    =   0x12, // Reserved (low)
        RESERVED_HIGH   =   0x1E, // Reserved (high)
        UNKNOWN         =   0x1F, // Unknown no device type
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ScsiIoProtocol {
    //TODO: return deviceType
    pub get_device_type:
        unsafe extern "efiapi" fn(this: *const Self, device_type: *mut DeviceType) -> Status,
    //TODO: raw pointer need to fixed, see uefi-rs service code like pointer *u8
    pub get_device_location:
        unsafe extern "efiapi" fn(this: *const Self, target: *mut *mut u8, lun: *mut u64) -> Status,
    pub reset_bus: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub reset_device: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub execute_scsi_command: unsafe extern "efiapi" fn(
        this: *const Self,
        packet: *mut ScsiIoScsiRequestPacket,
        event: Event,
    ) -> Status,
    pub io_align: u32,
}

/// 15.4. EFI SCSI I/O Protocol
impl ScsiIoProtocol {
    pub const GUID: Guid = guid!("932f47e6-2362-4002-803e-3cd54b138f85");
}

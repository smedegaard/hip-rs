use super::sys;
use crate::types::{Device, DeviceP2PAttribute, HipErrorKind, HipResult, PCIBusId, Result};
use semver::Version;
use std::ffi::CStr;
use std::i32;
use uuid::Uuid;

/// Initialize the HIP runtime.
///
/// Most HIP APIs implicitly initialize the HIP runtime.
/// This API provides control over the timing of the initialization.
///
/// # Returns
/// * `Result<()>` - Success or error status
///
/// # Errors
/// Returns `HipError` if:
/// * The runtime fails to initialize
/// * The runtime is already initialized
pub fn initialize() -> Result<()> {
    unsafe {
        let code = sys::hipInit(0);
        ((), code).to_result()
    }
}

/// Get the number of available HIP devices.
///
/// # Returns
/// * `Result<u32>` - The number of devices if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The runtime is not initialized (`HipErrorKind::NotInitialized`)
/// * The operation fails for other reasons
pub fn get_device_count() -> Result<i32> {
    unsafe {
        let mut count = 0;
        let code = sys::hipGetDeviceCount(&mut count);
        (count, code).to_result()
    }
}

/// Gets the currently active HIP device.
///
/// # Returns
/// Returns a `Result` containing either:
/// * `Ok(Device)` - The currently active device [`crate::Device`] if one is set
/// * `Err(HipError)` - If getting the device failed
///
/// # Errors
/// Returns `HipError` if:
/// * No device is currently active
/// * HIP runtime is not initialized
/// * There was an error accessing device information
pub fn get_device() -> Result<Device> {
    unsafe {
        let mut device_id: i32 = -1;
        let code = sys::hipGetDevice(&mut device_id);
        (Device::new(device_id), code).to_result()
    }
}

/// Sets the active HIP device for the current host thread.
///
/// This function makes the specified device active for all subsequent HIP operations
/// in the current host thread. Other host threads are not affected.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to make active
///
/// # Returns
/// * `Ok(())` if the device was successfully made active
/// * `Err(HipError)` if the operation failed
///
/// # Errors
/// Returns `HipError` if:
/// * The device ID is invalid (greater than or equal to device count)
/// * The HIP runtime is not initialized
/// * The specified device has encountered a previous error and is in a broken state
pub fn set_device(device: Device) -> Result<Device> {
    unsafe {
        let code = sys::hipSetDevice(device.id);
        (device, code).to_result()
    }
}

/// Gets the compute capability version of a HIP device.
///
/// This function retrieves the major and minor version numbers that specify the compute capability
/// of the given HIP device. Compute capability indicates the technical specifications and features
/// supported by the device's architecture.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to query instance representing the HIP device to query
///
/// # Returns
/// * `Result<Version>` - On success, returns a `Version` struct containing the major and minor version
///   numbers of the device's compute capability. On failure, returns an error indicating what went wrong.
pub fn device_compute_capability(device: Device) -> Result<Version> {
    unsafe {
        let mut major: i32 = -1;
        let mut minor: i32 = -1;
        let code = sys::hipDeviceComputeCapability(&mut major, &mut minor, device.id);
        let version = Version::new(major as u64, minor as u64, 0);
        (version, code).to_result()
    }
}

/// Returns the total amount of memory on a HIP device.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to query
///
/// # Returns
/// * `Result<usize>` - The total memory in bytes if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The device is invalid
/// * The runtime is not initialized
pub fn device_total_mem(device: Device) -> Result<usize> {
    unsafe {
        let mut size: usize = 0;
        let code = sys::hipDeviceTotalMem(&mut size, device.id);
        (size, code).to_result()
    }
}

/// Decodes a HIP version number from its internal integer representation.
///
/// The version is encoded as: major * 1_000_000 + minor * 1_000 + patch
///
/// # Arguments
/// * `version` - The encoded version number
///
/// # Returns
/// * `Version` - A semantic version with major, minor and patch components
fn decode_hip_version(version: i32) -> Version {
    if version == -1 {
        return Version::new(0, 0, 0);
    }
    let major = version / 1_000_000;
    let minor = (version / 1_000) % 1_000;
    let patch = version % 1_000;
    Version::new(major as u64, minor as u64, patch as u64)
}

/// Gets the version of the HIP runtime.
///
/// # Returns
/// * `Result<Version>` - The runtime version if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The runtime is not initialized
/// * Getting the version fails
pub fn runtime_get_version() -> Result<Version> {
    unsafe {
        let mut version: i32 = -1;
        let code = sys::hipRuntimeGetVersion(&mut version);
        let version = decode_hip_version(version);
        (version, code).to_result()
    }
}

/// Gets the name of a HIP device.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to query
///
/// # Returns
/// * `Result<String>` - The device name if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The device ID is invalid
/// * There was an error retrieving the device name
/// * The name string could not be converted to valid UTF-8
pub fn get_device_name(device: Device) -> Result<String> {
    const buffer_size: usize = 64;
    let mut buffer = vec![0i8; buffer_size];

    unsafe {
        let code = sys::hipDeviceGetName(buffer.as_mut_ptr(), buffer.len() as i32, device.id);
        // Convert the C string to a Rust String
        let c_str = CStr::from_ptr(buffer.as_ptr());
        (c_str.to_string_lossy().into_owned(), code).to_result()
    }
}

/// Gets the UUID bytes for a HIP device.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to query
///
/// # Returns
/// * `Result<[i8; 16]>` - The UUID as a 16-byte array if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The device is invalid
/// * The runtime is not initialized
/// * There was an error retrieving the UUID
fn get_device_uuid_bytes(device: Device) -> Result<[i8; 16]> {
    let mut hip_bytes = sys::hipUUID_t { bytes: [0; 16] };
    unsafe {
        let code = sys::hipDeviceGetUuid(&mut hip_bytes, device.id);
        (hip_bytes.bytes, code).to_result()
    }
}

/// Gets the UUID for a HIP device.
///
/// Retrieves the unique identifier (UUID) for a specified HIP device,
///
/// # Arguments
/// * `device` - The device [`crate::Device`]  to query
///
/// # Returns
/// * `Result<Uuid>` - The device UUID if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The device is invalid
/// * The runtime is not initialized
/// * There was an error retrieving the UUID
pub fn get_device_uuid(device: Device) -> Result<Uuid> {
    get_device_uuid_bytes(device).map(|bytes| {
        let uuid_bytes: [u8; 16] = bytes.map(|b| b as u8);
        Uuid::from_bytes(uuid_bytes)
    })
}

/// Retrieves a peer-to-peer attribute value between two HIP devices.
///
/// This function queries the specified peer-to-peer attribute between a source and destination device.
/// The attribute can be used to determine various P2P capabilities and performance characteristics
/// between the two devices.
///
/// # Arguments
/// * `src_device` - Source [`crate::Device`] for P2P attribute query
/// * `dst_device` - Target [`crate::Device`] for P2P attribute query
/// * `attr` - The [`DeviceP2PAttribute`](DeviceP2PAttribute) to query
///
/// # Returns
/// * `Result<i32>` - The attribute value if successful
///
/// # Errors
/// Returns `HipError` if:
/// * Either device ID is invalid
/// * The devices are the same
/// * The runtime is not initialized
/// * Getting the attribute fails
pub fn get_device_p2p_attribute(
    attr: DeviceP2PAttribute,
    src_device: Device,
    dst_device: Device,
) -> Result<i32> {
    let mut value = -1;
    unsafe {
        let code =
            sys::hipDeviceGetP2PAttribute(&mut value, attr.into(), src_device.id, dst_device.id);
        (value, code).to_result()
    }
}

/// Gets the PCI bus ID string for a HIP device.
///
/// # Arguments
/// * `device` - The device [`crate::Device`] to query
///
/// # Returns
/// * `Result<String>` - The PCI bus ID string if successful
///
/// # Errors
/// Returns `HipError` if:
/// * The device is invalid
/// * The runtime is not initialized
/// * There was an error retrieving the PCI bus ID
pub fn get_device_pci_bus_id(device: Device) -> Result<PCIBusId> {
    let mut pci_bus_id = PCIBusId::new();

    unsafe {
        let code = sys::hipDeviceGetPCIBusId(pci_bus_id.as_mut_ptr(), pci_bus_id.len(), device.id);
        (pci_bus_id, code).to_result()
    }
}

/// Gets a HIP device by its PCI bus ID.
///
/// # Arguments
/// * `pci_bus_id` - The PCI bus ID [`PCIBusId`] string identifying the device
///
/// # Returns
/// * `Result<Device>` - The device if found
///
/// # Errors
/// Returns `HipError` if:
/// * The PCI bus ID string is invalid
/// * No device with the specified PCI bus ID exists
/// * The runtime is not initialized
pub fn get_device_by_pci_bus_id(mut pci_bus_id: PCIBusId) -> Result<Device> {
    let mut device_id = i32::MAX;
    unsafe {
        let code = sys::hipDeviceGetByPCIBusId(&mut device_id, pci_bus_id.as_mut_ptr());
        (Device::new(device_id), code).to_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_device_by_pci_bus_id() {
        // we are relying on `get_device_pci_bus_id()` working as intended to test this function.
        // TODO: consider mocking to avoid test dependencies
        // First get a valid PCI bus ID from an existing device
        let device = Device::new(0);
        let pci_id = get_device_pci_bus_id(device).unwrap();

        // Test getting device by that PCI ID
        let result = get_device_by_pci_bus_id(pci_id);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id(), device.id());
    }

    #[test]
    fn test_get_device_by_invalid_pci_bus_id() {
        let invalid_pci_id = PCIBusId::new(); // invalid PCI ID, only contains `0`'s
        let result = get_device_by_pci_bus_id(invalid_pci_id);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_device_pci_bus_id() {
        let device = Device::new(0);
        let result = get_device_pci_bus_id(device);
        assert!(result.is_ok());
        let pci_id = result.unwrap();
        println!("Device PCI Bus ID: {:?}", pci_id);
    }

    #[test]
    fn test_get_device_pci_bus_id_invalid_device() {
        let invalid_device = Device::new(99);
        let result = get_device_pci_bus_id(invalid_device);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, HipErrorKind::InvalidDevice);
    }

    #[test]
    fn test_get_device_p2p_attribute() {
        let device_0 = Device::new(0);
        let device_1 = Device::new(1);

        let attributes = vec![
            DeviceP2PAttribute::PerformanceRank,
            DeviceP2PAttribute::AccessSupported,
            DeviceP2PAttribute::NativeAtomicSupported,
            DeviceP2PAttribute::HipArrayAccessSupported,
        ];

        for attr in attributes {
            let result = get_device_p2p_attribute(attr, device_0, device_1);
            assert!(result.is_ok());
            let value = result.unwrap();
            println!(
                "{:?} attribute value between device {} and {}: {}",
                attr,
                device_0.id(),
                device_1.id(),
                value
            );
        }
    }

    #[test]
    fn test_get_device_p2p_attribute_same_device() {
        let device = Device::new(0);

        let attributes = vec![
            DeviceP2PAttribute::PerformanceRank,
            DeviceP2PAttribute::AccessSupported,
            DeviceP2PAttribute::NativeAtomicSupported,
            DeviceP2PAttribute::HipArrayAccessSupported,
        ];

        for attr in attributes {
            let result = get_device_p2p_attribute(attr, device, device);
            assert!(
                result.is_err(),
                "expect getting P2P attribute from same device will fail, failed for attribute {:?}",
                attr
            );
        }
    }

    #[test]
    fn test_get_device_p2p_attribute_invalid_device() {
        let device = Device::new(0);
        let invalid_device = Device::new(99);

        let result =
            get_device_p2p_attribute(DeviceP2PAttribute::AccessSupported, device, invalid_device);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, HipErrorKind::InvalidDevice);
    }

    #[test]
    fn test_get_device_uuid_bytes() {
        let device = Device::new(0);
        let result = get_device_uuid_bytes(device);
        assert!(result.is_ok());
        let uuid_bytes = result.unwrap();
        assert_eq!(uuid_bytes.len(), 16);
        println!("Device UUID bytes: {:?}", uuid_bytes);
    }

    #[test]
    fn test_get_device_uuid() {
        let device = Device::new(0);
        let result = get_device_uuid(device);
        assert!(result.is_ok());
        let uuid = result.unwrap();
        println!("Device UUID: {}", uuid);
    }

    #[test]
    fn test_get_device_name() {
        let device = Device::new(0);
        let result = get_device_name(device);
        assert!(result.is_ok());
        let name = result.unwrap();
        println!("Device name: {}", name);
    }

    #[test]
    fn test_runtime_get_version() {
        let result = runtime_get_version();
        assert!(result.is_ok());
        let version = result.unwrap();
        println!(
            "Runtime version: {}.{}.{}",
            version.major, version.minor, version.patch
        );
    }

    #[test]
    fn test_device_total_mem() {
        let device = Device::new(0);
        let result = device_total_mem(device);
        assert!(result.is_ok());
        let size = result.unwrap();
        assert!(size > 0);
        println!("Total memory in bytes: {}", size);
    }

    #[test]
    fn test_get_device_compute_capability() {
        let device = Device::new(0);
        let result = device_compute_capability(device);
        assert!(result.is_ok());
        let version = result.unwrap();
        assert!(version.major > 0);
        println!("Compute Capability: {}.{}", version.major, version.minor);
    }

    #[test]
    fn test_initialize() {
        // Test success case
        let result = initialize();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_device_count() {
        // Test success case
        let result = get_device_count();
        assert!(result.is_ok());
        let count = result.unwrap();
        println!("Found {} devices", count);
        assert!(count > 0);
    }

    #[test]
    fn test_get_device() {
        // Test success case
        let result = get_device();
        assert!(result.is_ok());
        let device = result.unwrap();
        println!("Device {} is currently active", device.id);
        assert_eq!(device.id, 0);
    }

    #[test]
    fn test_set_device() {
        // Test success case with valid device
        let device = Device::new(1);
        let result = set_device(device);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id(), 1)
    }

    #[test]
    fn test_set_invalid_device() {
        // Test error case with invalid device
        let invalid_device = Device::new(99);
        let result = set_device(invalid_device);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind, HipErrorKind::InvalidDevice);
    }
}
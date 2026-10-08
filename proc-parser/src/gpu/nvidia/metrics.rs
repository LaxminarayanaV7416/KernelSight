use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::error::NvmlError;

#[derive(Default, Debug, Copy, Clone)]
pub struct GPUMetrics {}

pub struct NvidiaGPUMetrics {
    pub nvml: Option<Nvml>,
    pub values: GPUMetrics,
    pub gpu_count: u32,
}

impl NvidiaGPUMetrics {
    pub fn new() -> Self {
        let nvml = Nvml::init().unwrap();
        let gpu_count = nvml.device_count().unwrap();

        Self {
            nvml: Some(nvml),
            values: GPUMetrics::default(),
            gpu_count,
        }
    }

    pub fn get_device_details(&mut self) -> Vec<String> {
        let mut results = Vec::new();
        for index in 0..self.gpu_count {
            if let Some(nvml) = self.nvml.take() {
                let device = nvml.device_by_index(index).unwrap();
                let name = device.name().unwrap();
                let temperature = device.temperature(TemperatureSensor::Gpu).unwrap();
                let mem_info = device.memory_info().unwrap();
                let graphics_clock = device.clock_info(Clock::Graphics).unwrap();
                let mem_clock = device.clock_info(Clock::Memory).unwrap();
                let link_gen = device.current_pcie_link_gen().unwrap();
                let link_speed = device
                    .pcie_link_speed()
                    .map(u64::from)
                    // Convert megabytes to bytes
                    .map(|x| x * 1000000)
                    .unwrap();
                let link_width = device.current_pcie_link_width().unwrap();
                let max_link_gen = device.max_pcie_link_gen().unwrap();
                let max_link_width = device.max_pcie_link_width().unwrap();
                let max_link_speed = device
                    .max_pcie_link_speed()
                    .unwrap()
                    .as_integer()
                    .map(u64::from)
                    // Convert megabytes to bytes
                    .map(|x| x * 1000000);
                let cuda_cores = device.num_cores().unwrap();
                let architecture = device.architecture().unwrap();
                results.push(format!(
                    "Your {name} (architecture: {architecture}, CUDA cores: {cuda_cores})
                    is currently sitting at {temperature} °C with a graphics clock of
                    {graphics_clock} MHz and a memory clock of {mem_clock} MHz. Memory
                    usage is {used_mem} out of an available {total_mem}. Right now the
                    device is connected via a PCIe gen {link_gen} x{link_width} interface
                    with a transfer rate of {link_speed} per lane; the max your hardware
                    supports is PCIe gen {max_link_gen} x{max_link_width} at a transfer
                    rate of {max_link_speed} per lane.",
                    name = name,
                    temperature = temperature,
                    graphics_clock = graphics_clock,
                    mem_clock = mem_clock,
                    used_mem = mem_info.used,
                    total_mem = mem_info.total,
                    link_gen = link_gen,
                    // Convert byte output to transfers/sec
                    link_speed = link_speed,
                    link_width = link_width,
                    max_link_gen = max_link_gen,
                    max_link_width = max_link_width,
                    cuda_cores = cuda_cores,
                    architecture = architecture,
                    max_link_speed = max_link_speed.unwrap(),
                ));
                // put back the nvml
                self.nvml.insert(nvml);
            }
        }
        results
    }

    pub fn close_nvml(&mut self) -> Result<(), NvmlError> {
        if let Some(nvml) = self.nvml.take() {
            nvml.shutdown()?;
        }
        Ok(())
    }
}

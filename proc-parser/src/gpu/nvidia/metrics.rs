use nvml_wrapper::Nvml;
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

    pub fn close_nvml(&mut self) -> Result<(), NvmlError> {
        if let Some(nvml) = self.nvml.take() {
            nvml.shutdown()?;
        }

        Ok(())
    }
}

use candle_core::{Device, Shape, Tensor};

#[test]
fn matmul_two_tensors_with_cpu() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;
    let a = Tensor::arange(0f32, 6f32, &device)?.reshape((2, 3))?;
    let b = Tensor::arange(0f32, 6f32, &device)?.reshape((3, 2))?;
    let c = a.matmul(&b)?;
    assert_eq!(c.shape(), &Shape::from((2, 2)));
    Ok(())
}

#[cfg(feature = "metal")]
#[test]
fn matmul_two_tensors_with_metal() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::new_metal(0)?;
    let a = Tensor::arange(0f32, 6f32, &device)?.reshape((2, 3))?;
    let b = Tensor::arange(0f32, 6f32, &device)?.reshape((3, 2))?;
    let c = a.matmul(&b)?;
    assert_eq!(c.shape(), &Shape::from((2, 2)));
    Ok(())
}

use candle_core::{Device, Shape, Tensor};

#[test]
fn matmul_two_tensors() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::new_metal(0)?;
    let a = Tensor::arange(0f32, 10000000f32, &device)?.reshape((10000, 1000))?;
    let b = Tensor::arange(0f32, 10000000f32, &device)?.reshape((1000, 10000))?;
    for _ in 0..100 {
        let c = a.matmul(&b)?;
        assert_eq!(c.shape(), &Shape::from((10000, 10000)));
    }
    Ok(())
}

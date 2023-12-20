use candle_core::{Device, Shape, Tensor};
use candle_transformers::models::quantized_mistral;
use tokenizers::tokenizer::Tokenizer;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

#[test]
fn matmul_two_tensors_with_cpu() -> Result<()> {
    let device = Device::Cpu;
    let a = Tensor::arange(0f32, 6f32, &device)?.reshape((2, 3))?;
    let b = Tensor::arange(0f32, 6f32, &device)?.reshape((3, 2))?;
    let c = a.matmul(&b)?;
    assert_eq!(c.shape(), &Shape::from((2, 2)));
    Ok(())
}

#[cfg(feature = "metal")]
#[test]
fn matmul_two_tensors_with_metal() -> Result<()> {
    let device = Device::new_metal(0)?;
    let a = Tensor::arange(0f32, 6f32, &device)?.reshape((2, 3))?;
    let b = Tensor::arange(0f32, 6f32, &device)?.reshape((3, 2))?;
    let c = a.matmul(&b)?;
    assert_eq!(c.shape(), &Shape::from((2, 2)));
    Ok(())
}

#[test]
fn tokenize_prompt() -> Result<()> {
    let path = "/Users/adamkowalski/code/mistral7b/tokenizer.json";
    let tokenizer = Tokenizer::from_file(path)?;
    let prompt = "Hello, world!";
    let add_special_tokens = true;
    let tokens = tokenizer
        .encode(prompt, add_special_tokens)?
        .get_tokens()
        .to_vec();
    assert_eq!(tokens, vec!["<s>", "▁Hello", ",", "▁world", "!"]);
    Ok(())
}

#[test]
fn load_model() -> Result<()> {
    let path = "/Users/adamkowalski/code/mistral7b/model-q8_0.gguf";
    let use_flash_attn = true;
    let config = quantized_mistral::Config::config_7b_v0_1(use_flash_attn);
    let vb = quantized_mistral::VarBuilder::from_gguf(path)?;
    let model = quantized_mistral::Model::new(&config, vb)?;
    Ok(())
}

use warp::{Reply, Rejection, Filter};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn audio_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("audio" / "process")
        .and(warp::post())
        .and_then(process_audio)
}

async fn process_audio() -> Result<impl Reply, Rejection> {
    // Call the function to play the audio.
    match play_audio() {
        Ok(_) => Ok(warp::reply::html("Audio Processed!")),
        Err(err) => {
            eprintln!("Error playing audio: {:?}", err);
            Err(warp::reject::not_found())
        }
    }
}

fn play_audio() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("Default device not available")?;
    let config = device.default_output_config().map_err(|e| format!("{:?}", e))?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
}

fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), Box<dyn std::error::Error>>
where
    T: cpal::Sample,
{
    // Your audio data loading logic here.

    let error_callback = |err| eprintln!("an error occurred on stream: {:?}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Your data writing logic here.
        },
        error_callback,
    )?;
    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}

